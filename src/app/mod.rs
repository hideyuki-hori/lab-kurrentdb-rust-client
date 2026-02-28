mod cli;
mod commands;
mod dispatch;
mod projector_commands;
mod resolver;
mod stats_commands;

use anyhow::Result;
use clap::Parser;
use dispatch::dispatch;

use crate::infrastructure::event_store::EventStore;
use crate::infrastructure::projection::Projection;
use crate::usecases;

pub async fn run() -> Result<()> {
    let parsed = cli::Cli::parse();
    let env = crate::infrastructure::env::Env::load()?;
    let event_store = EventStore::connect(&env)?;
    let projection = Projection::connect(&env).await?;
    usecases::init::UseCase::new(
        usecases::init::operation::Op { projection: &projection },
    ).run().await?;
    let resolver = resolver::Resolver::new(&event_store, &projection);
    dispatch(parsed.command, &resolver).await
}
