use std::future::Future;

use anyhow::Result;

use crate::domain::Account;
use crate::domain::query::AccountBalance;

use super::args::Args;
use super::operation::Input;

pub trait Operation {
    fn execute(&self, input: &Input) -> impl Future<Output = Result<Option<AccountBalance>>> + Send;
}

pub trait Presenter {
    fn render(&self, account: &Account, result: Option<AccountBalance>);
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
        let input = Input { account: args.account };
        let result = self.op.execute(&input).await?;
        self.presenter.render(&input.account, result);
        Ok(())
    }
}
