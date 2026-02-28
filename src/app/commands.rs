use clap::Subcommand;

use crate::usecases;

use super::projector_commands::ProjectorCommands;
use super::stats_commands::StatsCommands;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Record income", after_help = "Example:\n  cargo r -- income alice 100000 salary")]
    Income(usecases::income::Args),
    #[command(about = "Record expense", after_help = "Example:\n  cargo r -- expense alice 30000 food lunch")]
    Expense(usecases::expense::Args),
    #[command(about = "Show event history", after_help = "Example:\n  cargo r -- history alice")]
    History(usecases::history::Args),
    #[command(about = "Show account balance", after_help = "Example:\n  cargo r -- balance alice")]
    Balance(usecases::balance::Args),
    #[command(about = "Watch account events in real-time", after_help = "Example:\n  cargo r -- watch alice")]
    Watch(usecases::watch::Args),
    #[command(subcommand, about = "Manage projector")]
    Projector(ProjectorCommands),
    #[command(subcommand, about = "Show statistics")]
    Stats(StatsCommands),
}
