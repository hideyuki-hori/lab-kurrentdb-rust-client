# KurrentDB ProjectionClient `query` パラメータ検証仕様

## 背景

`kurrentdb` crate (v1.0.0) の `ProjectionClient::create` メソッドは `query: String` パラメータを受け取る。
このパラメータに何を渡すべきかについて、Rust crate側にドキュメントがない。

KurrentDBサーバー側のドキュメント (https://docs.kurrent.io/server/v25.1/features/projections/custom) では
Projectionの定義はJavaScriptで書くとされているが、Rustクライアント側のコードやドキュメントには
その制約が明記されていない。

## 目的

`ProjectionClient::create` の `query` パラメータが受け付ける内容を実機検証で確定させる。

## スコープ

- このブランチ (`verify/projection-query`) は検証のみを行う
- サンプル実装やCLIコマンドの追加は行わない
- 検証結果を `VERIFY_RESULT.md` にまとめて完了とする

## 前提

- リポジトリ: https://github.com/hideyuki-hori/lab-kurrentdb-rust-client
- 既存のブランチ `main` に家計簿CLIがある
- ブランチ名: `verify/projection-query`

## 検証手順

### 1. KurrentDBの起動設定変更

`docker-compose.yml` に以下を追加してProjectionを有効化する。

```
--run-projections=All
--start-standard-projections=true
```

### 2. 依存関係の確認

`Cargo.toml` で使用している crate を確認する。
- `eventstore` crate を使っている場合、`kurrentdb` crate (1.0.0) にアップデートが必要か確認
- `ProjectionClient` が使える状態にする

### 3. gRPC proto定義の確認

`kurrentdb` crateのソースコード内にある proto 定義を確認する。
ローカルの依存キャッシュ（`~/.cargo/registry/src/` 配下）または
https://github.com/kurrent-io/KurrentDB-Client-Rust の以下を調査：

- `kurrentdb/src/event_store/` 配下の projections 関連 proto / generated コード
- `projections::create_req::Options` の `query` フィールドの型定義
- proto に `query` の説明コメントがあるか

特に確認したいのは：
- proto定義に `query` が `string` 型でJavaScriptと明記されているか
- サーバー側の projections.proto にコメントや制約があるか

### 4. 実機検証：JS文字列を渡す

以下の最小コードで Projection を作成し、正常に動作するか確認する。

```rust
use kurrentdb::{Client, ProjectionClient, CreateProjectionOptions};

let settings = "kurrentdb://admin:changeit@localhost:2113?tls=false".parse()?;
let client = Client::new(settings)?;
let projection_client: ProjectionClient = client.into();

let js_query = r#"
fromStream('account-alice')
  .when({
    $init: function() { return { balance: 0 }; },
    income: function(s, e) { s.balance += e.body.amount; },
    expense: function(s, e) { s.balance -= e.body.amount; }
  })
  .outputState()
"#;

let options = CreateProjectionOptions::default();
projection_client.create("test-balance", js_query.to_string(), &options).await?;
```

確認項目：
- `create` が成功するか
- `get_status` でProjectionが Running になるか
- 既存の `income` / `expense` コマンドでイベントを投入後、`get_state` で残高が取得できるか

### 5. 実機検証：JS以外を渡す

以下のパターンを試し、エラー内容を記録する。

#### 5a. 空文字列
```rust
projection_client.create("test-empty", "".to_string(), &options).await?;
```

#### 5b. 適当な文字列
```rust
projection_client.create("test-random", "hello world".to_string(), &options).await?;
```

#### 5c. SQL風クエリ
```rust
projection_client.create("test-sql", "SELECT * FROM account-alice".to_string(), &options).await?;
```

#### 5d. JSON
```rust
projection_client.create("test-json", r#"{"type": "balance"}"#.to_string(), &options).await?;
```

各パターンについて以下を記録：
- `create` の戻り値（Ok / Err とエラーメッセージ）
- `get_status` の `status` フィールド（Running / Faulted 等）
- Faulted の場合 `state_reason` の内容

### 6. KurrentDBサーバーのproto定義確認（補足）

KurrentDBサーバー本体のリポジトリ https://github.com/kurrent-io/KurrentDB から
projections関連の proto ファイルを探し、`query` フィールドの定義・コメントを確認する。

### 7. `emit` 機能の確認

`CreateProjectionOptions` に `emit` フラグがある。
`emit(true)` を設定した場合に `emit()` / `linkTo()` をJS内で使えるか確認する。

```rust
let js_with_emit = r#"
fromStream('account-alice')
  .when({
    expense: function(s, e) {
      if (e.body.amount > 50000) {
        emit('alert-alice', 'BudgetExceeded', { amount: e.body.amount });
      }
    }
  })
"#;

let options = CreateProjectionOptions::default().emit(true);
projection_client.create("test-alert", js_with_emit.to_string(), &options).await?;
```

確認項目：
- `emit()` でイベントが `alert-alice` ストリームに書き込まれるか
- そのストリームを通常の `client.read_stream` で読めるか

## 成果物

検証結果を以下の形式で `VERIFY_RESULT.md` にまとめる。

```markdown
## 検証結果

### proto定義
- query フィールドの型: （string / その他）
- コメント・説明の有無: （あり→内容 / なし）

### JS文字列
- create: （Ok / Err）
- status: （Running / Faulted / その他）
- get_state結果: （取得できた値）

### 空文字列
- create: （Ok / Err → エラー内容）
- status: （state_reason）

### 適当な文字列
- create: （Ok / Err → エラー内容）
- status: （state_reason）

### SQL風
- create: （Ok / Err → エラー内容）
- status: （state_reason）

### JSON
- create: （Ok / Err → エラー内容）
- status: （state_reason）

### emit機能
- create: （Ok / Err）
- 派生ストリームへの書き込み: （確認できた / できなかった）
- read_streamでの読み取り: （確認できた / できなかった）

### 結論
query パラメータは（JavaScript限定 / それ以外も可 / 不明）である。
根拠: （proto定義の内容 / 実機検証の結果）
```