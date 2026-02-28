use anyhow::Result;

use crate::domain::query::{AccountBalance, AccountBalanceRow};
use crate::infrastructure::projection::Projection;

const QUERY: &str = include_str!("query.sql");

pub struct Output {
    pub accounts: Vec<AccountBalance>,
    pub total_income: i64,
    pub total_expense: i64,
    pub net: i64,
}

pub struct Op<'a> {
    pub projection: &'a Projection,
}

impl super::Operation for Op<'_> {
    async fn execute(&self) -> Result<Output> {
        let rows: Vec<AccountBalanceRow> = self.projection.query_all(QUERY).await?;

        let accounts: Vec<AccountBalance> = rows
            .into_iter()
            .map(|r| AccountBalance {
                account: r.account,
                balance: r.balance,
                total_income: r.total_income,
                total_expense: r.total_expense,
                event_count: r.event_count,
            })
            .collect();

        let total_income: i64 = accounts.iter().map(|r| r.total_income).sum();
        let total_expense: i64 = accounts.iter().map(|r| r.total_expense).sum();
        let net = total_income - total_expense;

        Ok(Output { accounts, total_income, total_expense, net })
    }
}
