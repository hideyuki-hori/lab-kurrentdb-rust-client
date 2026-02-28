use std::future::Future;

use anyhow::Result;

use crate::domain::query::CategoryExpense;

pub trait Operation {
    fn execute(&self) -> impl Future<Output = Result<Vec<CategoryExpense>>> + Send;
}

pub trait Presenter {
    fn render(&self, rows: &[CategoryExpense]);
}

pub struct UseCase<O, P> {
    op: O,
    presenter: P,
}

impl<O: Operation, P: Presenter> UseCase<O, P> {
    pub fn new(op: O, presenter: P) -> Self {
        Self { op, presenter }
    }

    pub async fn run(&self) -> Result<()> {
        let rows = self.op.execute().await?;
        self.presenter.render(&rows);
        Ok(())
    }
}
