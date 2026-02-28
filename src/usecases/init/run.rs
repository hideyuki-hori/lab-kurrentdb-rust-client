use std::future::Future;

use anyhow::Result;

pub trait Operation {
    fn execute(&self) -> impl Future<Output = Result<()>> + Send;
}

pub struct UseCase<O> {
    op: O,
}

impl<O: Operation> UseCase<O> {
    pub fn new(op: O) -> Self {
        Self { op }
    }

    pub async fn run(&self) -> Result<()> {
        self.op.execute().await
    }
}
