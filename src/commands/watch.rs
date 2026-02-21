use kurrentdb::{Client, StreamPosition, SubscribeToStreamOptions};

use super::expense::rebuild_state;
use super::format_amount;
use crate::domain::{self, AccountEvent};

fn format_balance(balance: i64) -> String {
    if balance < 0 {
        format!("-{}", format_amount(balance.unsigned_abs()))
    } else {
        format_amount(balance.unsigned_abs())
    }
}

pub async fn run(client: &Client, account: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = domain::stream_name(account);

    let mut state = rebuild_state(client, &stream).await?;

    println!(
        "current balance: {} ({} events)",
        format_balance(state.balance),
        state.event_count,
    );

    println!("watching... (Ctrl+C to stop)");

    let options = SubscribeToStreamOptions::default().start_from(StreamPosition::End);
    let mut subscription = client.subscribe_to_stream(stream, &options).await;

    loop {
        let event = subscription.next().await?;
        let recorded = event.get_original_event();
        let account_event = recorded.as_json::<AccountEvent>()?;

        state = state.apply(&account_event, recorded.revision);

        let hours = recorded.created.format("%H").to_string();
        let minutes = recorded.created.format("%M").to_string();

        let line = match &account_event {
            AccountEvent::Income {
                amount,
                description,
            } => {
                format!(
                    "[{}:{}] +{} {} \u{2192} {}",
                    hours,
                    minutes,
                    format_amount(*amount),
                    description,
                    format_balance(state.balance),
                )
            }
            AccountEvent::Expense {
                amount,
                category,
                description,
            } => {
                format!(
                    "[{}:{}] -{} [{}] {} \u{2192} {}",
                    hours,
                    minutes,
                    format_amount(*amount),
                    category,
                    description,
                    format_balance(state.balance),
                )
            }
        };

        println!("{}", line);
    }
}
