use clap::Parser;

use super::commands::Commands;

#[derive(Parser)]
#[command(
    about = "Event-sourced household ledger on KurrentDB",
    after_help = "\
Examples:
  cargo r -- income alice 100000 salary
  cargo r -- expense alice 30000 food lunch
  cargo r -- balance alice
  cargo r -- history alice
  cargo r -- watch alice
  cargo r -- projector run
  cargo r -- stats category
  cargo r -- stats summary"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
