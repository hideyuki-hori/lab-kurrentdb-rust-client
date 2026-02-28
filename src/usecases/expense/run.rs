use std::future::Future;

use anyhow::Result;

use super::args::Args;
use super::operation::{Input, Output};

pub trait Operation {
    fn execute(&self, input: &Input) -> impl Future<Output = Result<Output>> + Send;
}

pub trait Presenter {
    fn render(&self, input: &Input, output: &Output);
}

pub struct UseCase<O, P> {
    op: O,
    presenter: P,
}

impl<O: Operation, P: Presenter> UseCase<O, P> {
    pub fn new(op: O, presenter: P) -> Self {
        Self { op, presenter }
    }

    pub async fn run(&self, args: Args) -> Result<()> {
        let input = Input {
            account: args.account,
            amount: args.amount,
            category: args.category,
            description: args.description,
        };
        let output = self.op.execute(&input).await?;
        self.presenter.render(&input, &output);
        Ok(())
    }
}
