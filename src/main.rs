mod commands;
mod domain;

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
    }

    Ok(())
}
