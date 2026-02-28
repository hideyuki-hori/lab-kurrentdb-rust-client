use std::future::Future;

use anyhow::Result;

use super::operation::Output;

pub trait Operation {
    fn execute(&self) -> impl Future<Output = Result<Output>> + Send;
}

pub trait Presenter {
    fn render(&self, output: &Output);
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
        let output = self.op.execute().await?;
        self.presenter.render(&output);
        Ok(())
    }
}
