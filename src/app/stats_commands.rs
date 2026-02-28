use clap::Subcommand;

#[derive(Subcommand)]
pub enum StatsCommands {
    #[command(about = "Show expenses by category")]
    Category,
    #[command(about = "Show overall account summary")]
    Summary,
}
