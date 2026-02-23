use kurrentdb::{Client, GenericProjectionOptions, ProjectionClient};

use crate::projections::ALL_PROJECTIONS;

pub async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let proj_client = ProjectionClient::from(client.clone());
    let options = GenericProjectionOptions::default();

    println!("{:<20} {:<12} {:>8}", "NAME", "STATUS", "PROGRESS");
    println!("{}", "-".repeat(42));

    for &(name, _, _) in ALL_PROJECTIONS {
        match proj_client.get_status(name, &options).await {
            Ok(Some(status)) => {
                println!(
                    "{:<20} {:<12} {:>7.1}%",
                    status.name, status.status, status.progress,
                );
            }
            Ok(None) => {
                println!("{:<20} {:<12}", name, "not found");
            }
            Err(e) => {
                println!("{:<20} error: {}", name, e);
            }
        }
    }

    Ok(())
}
