use kurrentdb::{Client, GetStateProjectionOptions, ProjectionClient};

use crate::projections::{SummaryState, SUMMARY_NAME};
use super::format_amount_f64;

pub async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let proj_client = ProjectionClient::from(client.clone());
    let options = GetStateProjectionOptions::default();

    let state: SummaryState = proj_client
        .get_state::<_, SummaryState>(SUMMARY_NAME, &options)
        .await??;

    println!("=== Account Summary ===\n");
    println!("  income:  {:>12}", format_amount_f64(state.total_income));
    println!("  expense: {:>12}", format_amount_f64(state.total_expense));
    println!("  net:     {:>12}", format_amount_f64(state.net));

    if !state.accounts.is_empty() {
        println!("\n{:<25} {:>12}", "ACCOUNT", "BALANCE");
        println!("{}", "-".repeat(39));

        let mut accounts: Vec<(&String, &f64)> = state.accounts.iter().collect();
        accounts.sort_by_key(|(name, _)| name.to_string());

        for (account, balance) in accounts {
            let display_name = account.strip_prefix("account-").unwrap_or(account);
            println!(
                "{:<25} {:>12}",
                display_name,
                format_amount_f64(*balance),
            );
        }
    }

    Ok(())
}
