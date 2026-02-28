use anyhow::Result;

use crate::infrastructure::projection::Projection;

const CREATE_ACCOUNT_BALANCES: &str = include_str!("create_account_balances.sql");
const CREATE_CATEGORY_EXPENSES: &str = include_str!("create_category_expenses.sql");
const CREATE_CHECKPOINTS: &str = include_str!("create_checkpoints.sql");

pub struct Op<'a> {
    pub projection: &'a Projection,
}

impl super::Operation for Op<'_> {
    async fn execute(&self) -> Result<()> {
        self.projection.execute(CREATE_ACCOUNT_BALANCES).await?;
        self.projection.execute(CREATE_CATEGORY_EXPENSES).await?;
        self.projection.execute(CREATE_CHECKPOINTS).await?;
        Ok(())
    }
}
