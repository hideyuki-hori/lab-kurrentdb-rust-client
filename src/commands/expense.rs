use kurrentdb::{AppendToStreamOptions, Client, EventData, StreamState};

use super::format_amount;
use crate::domain::{self, AccountEvent, AccountState};

pub async fn rebuild_state(
    client: &Client,
    stream: &str,
) -> Result<AccountState, Box<dyn std::error::Error>> {
    let mut state = AccountState::default();
    let mut read = client.read_stream(stream.to_owned(), &Default::default()).await?;
    while let Some(event) = read.next().await? {
        let recorded = event.get_original_event();
        let account_event = recorded.as_json::<AccountEvent>()?;
        state = state.apply(&account_event, recorded.revision);
    }

    Ok(state)
}

pub async fn run(
    client: &Client,
    account: &str,
    amount: u64,
    category: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let stream = domain::stream_name(account);
    let max_retries = 3;

    for attempt in 0..max_retries {
        let state = rebuild_state(client, &stream).await?;
        
        if state.balance < i64::try_from(amount).unwrap_or(i64::MAX) {
            eprintln!(
                "error: insufficient balance ({} < {})",
                format_amount(state.balance.unsigned_abs()),
                format_amount(amount),
            );
            std::process::exit(1);
        }

        let event = AccountEvent::Expense { 
            amount,
            category: category.to_string(),
            description: description.to_string(),
        };
        let event_data = EventData::json("expense", &event)?;

        let stream_state = if state.event_count == 0 {
            StreamState::NoStream
        } else {
            StreamState::StreamRevision(state.revision)
        };

        let options = AppendToStreamOptions::default().stream_state(stream_state);

        let appended = client.append_to_stream(stream.clone(), &options, event_data).await;

        match appended {
            Ok(result) => {
                println!(
                    "\u{2713} - {} \u{2192} {} [{}] (rev: {})",
                    format_amount(amount),
                    account,
                    category,
                    result.next_expected_version,
                );
                return Ok(());
            }
            Err(e) => {
                let is_wrong_version = matches!(&e, kurrentdb::Error::WrongExpectedVersion { .. });
                if is_wrong_version && attempt < max_retries - 1 {
                    continue;
                }
                if is_wrong_version {
                    eprintln!("error: concurrency conflict after {} retries", max_retries);
                    std::process::exit(1);
                }
                return Err(Box::new(e));
            }
        }
    }

    unreachable!()
}