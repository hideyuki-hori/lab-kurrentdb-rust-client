use anyhow::Result;

use crate::domain::Account;
use crate::domain::query::{AccountBalance, BalanceRow};
use crate::infrastructure::projection::Projection;

const QUERY: &str = include_str!("query.sql");

pub struct Input {
    pub account: Account,
}

pub struct Op<'a> {
    pub projection: &'a Projection,
}

impl super::Operation for Op<'_> {
    async fn execute(&self, input: &Input) -> Result<Option<AccountBalance>> {
        let row: Option<BalanceRow> = self.projection.query_one(QUERY, &[input.account.as_ref()]).await?;

        Ok(row.map(|r| AccountBalance {
            account: input.account.to_string(),
            balance: r.balance,
            total_income: r.total_income,
            total_expense: r.total_expense,
            event_count: r.event_count,
        }))
    }
}
