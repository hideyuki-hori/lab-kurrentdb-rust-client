use kurrentdb::{Client, StreamPosition, SubscribeToStreamOptions};

use crate::projections::BudgetExceededEvent;
use super::format_amount_f64;

pub async fn run(client: &Client, account: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = format!("alert-{}", account);

    println!("watching alerts for {}... (Ctrl+C to stop)", account);

    let options = SubscribeToStreamOptions::default().start_from(StreamPosition::Start);
    let mut subscription = client.subscribe_to_stream(stream, &options).await;

    loop {
        let event = subscription.next().await?;
        let recorded = event.get_original_event();
        let alert = recorded.as_json::<BudgetExceededEvent>()?;

        println!(
            "[ALERT] {} - category '{}' exceeded budget: {} (triggered by +{})",
            alert.account,
            alert.category,
            format_amount_f64(alert.total),
            format_amount_f64(alert.triggered_by),
        );
    }
}
