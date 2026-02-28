use anyhow::Result;

use super::commands::Commands;
use super::projector_commands::ProjectorCommands;
use super::resolver::Resolver;
use super::stats_commands::StatsCommands;

pub async fn dispatch(command: Commands, resolver: &Resolver<'_>) -> Result<()> {
    match command {
        Commands::Income(args) => resolver.income(args).await,
        Commands::Expense(args) => resolver.expense(args).await,
        Commands::History(args) => resolver.history(args).await,
        Commands::Balance(args) => resolver.balance(args).await,
        Commands::Watch(args) => resolver.watch(args).await,
        Commands::Projector(ProjectorCommands::Run) => resolver.projector().await,
        Commands::Stats(StatsCommands::Category) => resolver.stats_category().await,
        Commands::Stats(StatsCommands::Summary) => resolver.stats_summary().await,
    }
}
