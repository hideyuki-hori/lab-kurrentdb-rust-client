use std::future::Future;

use anyhow::Result;

use crate::domain::Account;
use crate::domain::query::ReceivedEvent;
use crate::domain::AccountState;
use crate::infrastructure::event_store::EventSubscription;

use super::args::Args;

pub struct WatchSession {
    pub state: AccountState,
    pub subscription: EventSubscription,
}

pub trait Operation {
    fn execute(&self, account: &Account) -> impl Future<Output = Result<WatchSession>> + Send;
}

pub trait Presenter {
    fn render_status(&self, balance: i64, event_count: usize);
    fn render_event(&self, received: &ReceivedEvent, balance: i64);
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
        let mut session = self.op.execute(&args.account).await?;

        self.presenter.render_status(session.state.balance, session.state.event_count);

        loop {
            let received = session.subscription.next().await?;
            session.state = session.state.apply(&received.event, received.revision);
            self.presenter.render_event(&received, session.state.balance);
        }
    }
}
