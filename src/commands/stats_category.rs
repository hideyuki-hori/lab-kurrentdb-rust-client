use kurrentdb::{Client, GetStateProjectionOptions, ProjectionClient};

use crate::projections::{CategoryState, CATEGORY_NAME};
use super::format_amount_f64;

pub async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let proj_client = ProjectionClient::from(client.clone());
    let options = GetStateProjectionOptions::default();

    let state: CategoryState = proj_client
        .get_state::<_, CategoryState>(CATEGORY_NAME, &options)
        .await??;

    if state.categories.is_empty() {
        println!("No expense data found.");
        return Ok(());
    }

    println!("{:<15} {:>12} {:>6}", "CATEGORY", "TOTAL", "COUNT");
    println!("{}", "-".repeat(35));

    let mut entries: Vec<_> = state.categories.iter().collect();
    entries.sort_by(|a, b| b.1.total.partial_cmp(&a.1.total).unwrap());

    for (category, entry) in entries {
        println!(
            "{:<15} {:>12} {:>6}",
            category,
            format_amount_f64(entry.total),
            entry.count as u64,
        );
    }

    Ok(())
}
