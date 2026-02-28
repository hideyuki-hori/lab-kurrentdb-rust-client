use anyhow::Result;

use crate::domain::query::{CategoryExpense, CategoryExpenseRow};
use crate::infrastructure::projection::Projection;

const QUERY: &str = include_str!("query.sql");

pub struct Op<'a> {
    pub projection: &'a Projection,
}

impl super::Operation for Op<'_> {
    async fn execute(&self) -> Result<Vec<CategoryExpense>> {
        let rows: Vec<CategoryExpenseRow> = self.projection.query_all(QUERY).await?;

        Ok(rows
            .into_iter()
            .map(|r| CategoryExpense {
                category: r.category,
                total: r.total,
                count: r.count,
            })
            .collect())
    }
}
