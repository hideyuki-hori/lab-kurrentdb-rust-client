use kurrentdb::{Client, ReadStreamOptions};

use super::format_amount;
use crate::domain::{self, AccountEvent};

pub async fn run(client: &Client, account: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = domain::stream_name(account);
    let options = ReadStreamOptions::default();
    let mut read = client.read_stream(stream, &options).await?;

    let mut count = 0usize;

    while let Some(event) = read.next().await? {
        let recorded = event.get_original_event();
        let account_event = recorded.as_json::<AccountEvent>()?;

        let ts = recorded.created.format("%Y-%m-%d %H:%M");

        let (sign, amt, desc) = match &account_event {
            AccountEvent::Income {
                amount,
                description,
            } => ("+", *amount, description.clone()),
            AccountEvent::Expense {
                amount,
                category,
                description,
            } => {
                let desc = format!("[{}] {}", category, description);
                ("-", *amount, desc)
            }
        };

        println!(
            "#{:<3} {}{:>9} {:<30} {}",
            recorded.revision,
            sign,
            format_amount(amt),
            desc,
            ts,
        );

        count += 1;
    }

    if count == 0 {
        println!("No events found.");
    }

    Ok(())
}
