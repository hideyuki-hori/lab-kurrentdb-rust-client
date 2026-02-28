use anyhow::Result;

use crate::domain::{Account, AccountEvent, Amount, Description};
use crate::infrastructure::event_store::EventStore;

pub struct Input {
    pub account: Account,
    pub amount: Amount,
    pub description: Description,
}

pub struct Output {
    pub revision: u64,
}

pub struct Op<'a> {
    pub event_store: &'a EventStore,
}

impl super::Operation for Op<'_> {
    async fn execute(&self, input: &Input) -> Result<Output> {
        let stream = input.account.stream_name();
        let event = AccountEvent::Income {
            amount: input.amount,
            description: input.description.clone(),
        };
        let revision =
            self.event_store.append_event_simple(&stream, "income", &event).await?;
        Ok(Output { revision })
    }
}
