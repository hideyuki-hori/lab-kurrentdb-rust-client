use kurrentdb::{Client, CreateProjectionOptions, ProjectionClient};

use crate::projections::ALL_PROJECTIONS;

pub async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let proj_client = ProjectionClient::from(client.clone());

    for &(name, query, emit) in ALL_PROJECTIONS {
        let options = CreateProjectionOptions::default()
            .emit(emit)
            .track_emitted_streams(emit);
        match proj_client.create(name, query.to_string(), &options).await {
            Ok(()) => println!("  \u{2713} {}", name),
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("Conflict") || msg.contains("already exists") {
                    println!("  - {} (already exists)", name);
                } else {
                    eprintln!("  \u{2717} {} : {}", name, e);
                }
            }
        }
    }

    println!("\nprojection setup complete.");
    Ok(())
}
