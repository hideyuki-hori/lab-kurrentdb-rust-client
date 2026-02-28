use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProjectorCommands {
    #[command(about = "Start projecting events to PostgreSQL")]
    Run,
}
