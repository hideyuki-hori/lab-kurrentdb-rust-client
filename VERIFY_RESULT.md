## 検証結果

### 検証環境

- kurrentdb crate: 1.0.0
- KurrentDB Server: kurrentplatform/kurrentdb:latest
- 起動オプション: `--insecure --run-projections=All --start-standard-projections=true`

### proto定義

- query フィールドの型: `string`（`projections.proto` の `CreateReq.Options` で `string query = 4;`）
- コメント・説明の有無: なし。proto ファイルに JavaScript や言語に関する制約コメントは一切なし
- Rust 生成コード: `pub query: ::prost::alloc::string::String`（単純な String 型）

### JS文字列

```javascript
fromStream('account-alice')
  .when({
    $init: function() { return { balance: 0 }; },
    income: function(s, e) { s.balance += e.body.amount; },
    expense: function(s, e) { s.balance -= e.body.amount; }
  })
  .outputState()
```

- create: **Ok**
- status: **Running**
- get_state結果: `{"balance": 38000.0}`（income 50000 - expense 12000 = 38000）

### 空文字列

- create: **Ok**（サーバーは受け付けるが、非同期でバリデーションエラー）
- status: **Faulted**
- state_reason: `None of streams and categories are included`

### 適当な文字列

`"hello world"`

- create: **Ok**（サーバーは受け付ける）
- status: **Faulted**
- state_reason: `Unexpected identifier 'world' (<anonymous>:1:7)`

### SQL風

`"SELECT * FROM account-alice"`

- create: **Ok**（サーバーは受け付ける）
- status: **Faulted**
- state_reason: `Unexpected identifier 'account' (<anonymous>:1:15)`

### JSON

`{"type": "balance"}`

- create: **Ok**（サーバーは受け付ける）
- status: **Faulted**
- state_reason: `Unexpected token ':' (<anonymous>:1:8)`

### emit機能

```javascript
fromStream('account-alice')
  .when({
    expense: function(s, e) {
      if (e.body.amount > 5000) {
        emit('alert-alice', 'BudgetExceeded', { amount: e.body.amount });
      }
    }
  })
```

- create: **Ok**（`CreateProjectionOptions::default().emit(true)` を使用）
- status: **Running**
- 派生ストリームへの書き込み: **確認できた**（`alert-alice` ストリームに `BudgetExceeded` イベントが出力された）
- read_streamでの読み取り: **確認できた**（`event[0]: type=BudgetExceeded, data={"amount":12000}`）

### 結論

query パラメータは **JavaScript 限定** である。

根拠:

1. **proto定義**: `string` 型で言語の制約は明記されていないが、型としては JavaScript コードを文字列で渡す設計
2. **実機検証**: JavaScript 以外の文字列は全て `create` 自体は成功するが、サーバー側の JavaScript エンジンが非同期でパースし、構文エラーの場合は Faulted になる
3. **エラーメッセージ**: 全て JavaScript パースエラーの形式（`Unexpected identifier`, `Unexpected token` + `<anonymous>:行:列`）であり、サーバー内部に JavaScript エンジンが組み込まれていることが確認できる
4. **バリデーションのタイミング**: `create` RPC 自体はバリデーションなしで成功し、サーバー側で非同期に JavaScript としてパース・実行される。不正なクエリは Faulted ステータスと `state_reason` で確認できる
5. **emit機能**: `emit(true)` オプションと JavaScript 内の `emit()` 関数で、別ストリームへのイベント転送が正常に動作する

### 補足: CreateProjectionOptions

- `emit(bool)`: JS 内で `emit()` / `linkTo()` を使う場合に `true` を設定する。実装上は `create` 後に別途 `update` RPC で emit フラグを設定している
- `track_emitted_streams(bool)`: emit されたストリームを追跡するか指定する
