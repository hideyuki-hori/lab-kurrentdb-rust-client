use crate::domain::{Account, Amount, Category, Description};

#[derive(clap::Args)]
pub struct Args {
    pub account: Account,
    pub amount: Amount,
    pub category: Category,
    pub description: Description,
}
