use std::collections::HashMap;

use kurrentdb::{
    Client, CreateProjectionOptions, DeleteProjectionOptions, EventData,
    GenericProjectionOptions, ProjectionClient,
};

type BoxErr = Box<dyn std::error::Error>;

async fn wait_for_projection(projection: &ProjectionClient, name: &str) {
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if let Ok(Some(status)) = projection.get_status(name, &GenericProjectionOptions::default()).await {
            let s = status.status.to_lowercase();
            if s.contains("running") || s.contains("faulted") || s.contains("stopped") {
                return;
            }
        }
    }
}

async fn cleanup(projection: &ProjectionClient, name: &str) {
    let _ = projection.disable(name, &GenericProjectionOptions::default()).await;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let _ = projection
        .delete(
            name,
            &DeleteProjectionOptions::default(),
        )
        .await;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
}

async fn get_projection_status(projection: &ProjectionClient, name: &str) -> (String, String) {
    match projection.get_status(name, &GenericProjectionOptions::default()).await {
        Ok(Some(status)) => (status.status.clone(), status.state_reason.clone()),
        Ok(None) => ("None".to_string(), String::new()),
        Err(e) => (format!("Error: {e}"), String::new()),
    }
}

async fn test_create(
    projection: &ProjectionClient,
    name: &str,
    query: &str,
    options: &CreateProjectionOptions,
) -> Result<(), String> {
    match projection.create(name, query.to_string(), options).await {
        Ok(()) => {
            println!("  create: Ok");
            Ok(())
        }
        Err(e) => {
            let msg = format!("{e}");
            println!("  create: Err -> {msg}");
            Err(msg)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), BoxErr> {
    let settings = "kurrentdb://admin:changeit@localhost:2113?tls=false".parse()?;
    let client = Client::new(settings)?;
    let projection: ProjectionClient = client.clone().into();
    let options = CreateProjectionOptions::default();

    println!("=== ProjectionClient query パラメータ検証 ===\n");

    println!("--- 1. JS文字列 (正常系) ---");
    let js_query = r#"
fromStream('account-alice')
  .when({
    $init: function() { return { balance: 0 }; },
    income: function(s, e) { s.balance += e.body.amount; },
    expense: function(s, e) { s.balance -= e.body.amount; }
  })
  .outputState()
"#;

    cleanup(&projection, "test-balance").await;

    if test_create(&projection, "test-balance", js_query, &options).await.is_ok() {
        wait_for_projection(&projection, "test-balance").await;
        let (status, reason) = get_projection_status(&projection, "test-balance").await;
        println!("  status: {status}");
        if !reason.is_empty() {
            println!("  state_reason: {reason}");
        }

        println!("\n  イベント投入中...");
        let stream = "account-alice";
        let income = serde_json::json!({"type": "income", "amount": 50000, "description": "salary"});
        let expense = serde_json::json!({"type": "expense", "amount": 12000, "category": "food", "description": "groceries"});
        let income_event = EventData::json("income", &income)?;
        let expense_event = EventData::json("expense", &expense)?;

        client.append_to_stream(stream, &Default::default(), income_event).await?;
        client.append_to_stream(stream, &Default::default(), expense_event).await?;
        println!("  投入完了: income=50000, expense=12000");

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let (status, _) = get_projection_status(&projection, "test-balance").await;
        println!("  投入後status: {status}");

        match projection
            .get_state::<_, HashMap<String, serde_json::Value>>("test-balance", &Default::default())
            .await
        {
            Ok(Ok(state)) => println!("  get_state: {state:?}"),
            Ok(Err(e)) => println!("  get_state deserialize error: {e}"),
            Err(e) => println!("  get_state error: {e}"),
        }
    }

    println!("\n--- 2. 空文字列 ---");
    cleanup(&projection, "test-empty").await;
    if test_create(&projection, "test-empty", "", &options).await.is_ok() {
        wait_for_projection(&projection, "test-empty").await;
        let (status, reason) = get_projection_status(&projection, "test-empty").await;
        println!("  status: {status}");
        println!("  state_reason: {reason}");
    }

    println!("\n--- 3. 適当な文字列 ---");
    cleanup(&projection, "test-random").await;
    if test_create(&projection, "test-random", "hello world", &options).await.is_ok() {
        wait_for_projection(&projection, "test-random").await;
        let (status, reason) = get_projection_status(&projection, "test-random").await;
        println!("  status: {status}");
        println!("  state_reason: {reason}");
    }

    println!("\n--- 4. SQL風クエリ ---");
    cleanup(&projection, "test-sql").await;
    if test_create(&projection, "test-sql", "SELECT * FROM account-alice", &options).await.is_ok() {
        wait_for_projection(&projection, "test-sql").await;
        let (status, reason) = get_projection_status(&projection, "test-sql").await;
        println!("  status: {status}");
        println!("  state_reason: {reason}");
    }

    println!("\n--- 5. JSON ---");
    cleanup(&projection, "test-json").await;
    if test_create(&projection, "test-json", r#"{"type": "balance"}"#, &options)
        .await
        .is_ok()
    {
        wait_for_projection(&projection, "test-json").await;
        let (status, reason) = get_projection_status(&projection, "test-json").await;
        println!("  status: {status}");
        println!("  state_reason: {reason}");
    }

    println!("\n--- 6. emit機能 ---");
    let js_with_emit = r#"
fromStream('account-alice')
  .when({
    expense: function(s, e) {
      if (e.body.amount > 5000) {
        emit('alert-alice', 'BudgetExceeded', { amount: e.body.amount });
      }
    }
  })
"#;
    cleanup(&projection, "test-alert").await;
    let emit_options = CreateProjectionOptions::default().emit(true);
    if test_create(&projection, "test-alert", js_with_emit, &emit_options)
        .await
        .is_ok()
    {
        wait_for_projection(&projection, "test-alert").await;
        let (status, reason) = get_projection_status(&projection, "test-alert").await;
        println!("  status: {status}");
        if !reason.is_empty() {
            println!("  state_reason: {reason}");
        }

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        println!("  alert-alice ストリーム読み取り...");
        let mut stream = client
            .read_stream("alert-alice", &Default::default())
            .await?;

        let mut count = 0;
        while let Ok(Some(event)) = stream.next().await {
            let resolved = event.get_original_event();
            let data: serde_json::Value =
                serde_json::from_slice(&resolved.data)?;
            println!(
                "  event[{count}]: type={}, data={data}",
                resolved.event_type
            );
            count += 1;
        }
        if count == 0 {
            println!("  alert-alice ストリーム: イベントなし（またはストリーム未作成）");
        }
    }

    println!("\n=== 検証完了 ===");
    Ok(())
}
