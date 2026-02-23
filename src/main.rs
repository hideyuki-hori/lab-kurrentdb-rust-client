mod commands;
mod domain;
mod projections;

use clap::{Parser, Subcommand};
use kurrentdb::Client;

#[derive(Parser)]
#[command(name = "kktb", about = "Event-sourced household ledger on KurrentDB")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Income {
        account: String,
        amount: u64,
        description: String,
    },
    Expense {
        account: String,
        amount: u64,
        category: String,
        description: String,
    },
    History {
        account: String,
    },
    Balance {
        account: String,
    },
    Watch {
        account: String,
    },
    #[command(subcommand)]
    Projection(ProjectionCommands),
    #[command(subcommand)]
    Stats(StatsCommands),
    #[command(subcommand)]
    Alert(AlertCommands),
}

#[derive(Subcommand)]
enum ProjectionCommands {
    Setup,
    Status,
}

#[derive(Subcommand)]
enum StatsCommands {
    Category,
    Summary,
}

#[derive(Subcommand)]
enum AlertCommands {
    Watch { account: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = "kurrentdb://admin:changeit@localhost:2113?tls=false".parse()?;
    let client = Client::new(settings)?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Income {
            account,
            amount,
            description,
        } => commands::income::run(&client, &account, amount, &description).await?,
        Commands::Expense {
            account,
            amount,
            category,
            description,
        } => commands::expense::run(&client, &account, amount, &category, &description).await?,
        Commands::History { account } => commands::history::run(&client, &account).await?,
        Commands::Balance { account } => commands::balance::run(&client, &account).await?,
        Commands::Watch { account } => commands::watch::run(&client, &account).await?,
        Commands::Projection(cmd) => match cmd {
            ProjectionCommands::Setup => commands::projection_setup::run(&client).await?,
            ProjectionCommands::Status => commands::projection_status::run(&client).await?,
        },
        Commands::Stats(cmd) => match cmd {
            StatsCommands::Category => commands::stats_category::run(&client).await?,
            StatsCommands::Summary => commands::stats_summary::run(&client).await?,
        },
        Commands::Alert(cmd) => match cmd {
            AlertCommands::Watch { account } => {
                commands::alert_watch::run(&client, &account).await?
            }
        },
    }

    Ok(())
}
