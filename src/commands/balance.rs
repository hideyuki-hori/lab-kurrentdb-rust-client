use kurrentdb::{Client, GetStateProjectionOptions, ProjectionClient};

use crate::domain;
use crate::projections::{BalanceState, BALANCE_NAME};
use super::format_amount_f64;

pub async fn run(
    client: &Client,
    account: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let proj_client = ProjectionClient::from(client.clone());
    let stream = domain::stream_name(account);
    let options = GetStateProjectionOptions::default().partition(stream);

    let state: BalanceState = proj_client
        .get_state::<_, BalanceState>(BALANCE_NAME, &options)
        .await??;

    if state.event_count == 0.0 {
        println!("No events found.");
        return Ok(());
    }

    let balance_str = format_amount_f64(state.balance);

    println!(
        "{}: {} ({} events)",
        account,
        balance_str,
        state.event_count as u64,
    );

    println!("  income:  {:>9}", format_amount_f64(state.total_income));
    println!("  expense: {:>9}", format_amount_f64(state.total_expense));

    Ok(())
}
