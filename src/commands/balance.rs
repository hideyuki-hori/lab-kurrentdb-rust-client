use kurrentdb::{Client, ReadStreamOptions};

use crate::domain::{self, AccountEvent, AccountState};
use super::format_amount;

pub async fn run(
  client: &Client,
  account: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let stream = domain::stream_name(account);
  let options = ReadStreamOptions::default();
  let mut read = client.read_stream(stream, &options).await?;

  let mut state = AccountState::default();

  while let Some(event) = read.next().await? {
    let recorded = event.get_original_event();
    let account_event = recorded.as_json::<AccountEvent>()?;
    state = state.apply(&account_event, recorded.revision);
  }

  if state.event_count == 0 {
    println!("No events found.");
    return Ok(());
  }

  let balance_str = if state.balance < 0 {
    format!("-{}", format_amount(state.balance.unsigned_abs()))
  } else {
    format_amount(state.balance.unsigned_abs())
  };

  println!(
    "{}: {} ({} events, rev: {})",
    account,
    balance_str,
    state.event_count,
    state.revision,
  );

  println!("  income:  {:>9}", format_amount(state.total_income));
  println!("  expense: {:>9}", format_amount(state.total_expense));

  Ok(())
}