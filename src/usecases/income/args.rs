use crate::domain::{Account, Amount, Description};

#[derive(clap::Args)]
pub struct Args {
    pub account: Account,
    pub amount: Amount,
    pub description: Description,
}
