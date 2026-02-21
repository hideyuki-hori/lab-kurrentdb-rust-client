use kurrentdb::{Client, EventData};

use crate::domain::{self, AccountEvent};
use super::format_amount;

pub async fn run(
  client: &Client,
  account: &str,
  amount: u64,
  description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let stream = domain::stream_name(account);
  let event = AccountEvent::Income {
    amount,
    description: description.to_string(),
  };
  let event_data = EventData::json("income", &event)?;
  let result = client
    .append_to_stream(stream, &Default::default(),  event_data)
    .await?;

  println!(
    "\u{2713} + {} \u{2192} {} (rev: {})",
    format_amount(amount),
    account,
    result.next_expected_version,
  );
  Ok(())
}