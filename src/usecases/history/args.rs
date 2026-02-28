use crate::domain::Account;

#[derive(clap::Args)]
pub struct Args {
    pub account: Account,
}
