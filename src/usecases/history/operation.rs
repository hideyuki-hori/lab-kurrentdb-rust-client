use anyhow::Result;

use crate::domain::Account;
use crate::domain::query::HistoryEntry;
use crate::infrastructure::event_store::EventStore;

pub struct Input {
    pub account: Account,
}

pub struct Op<'a> {
    pub event_store: &'a EventStore,
}

impl super::Operation for Op<'_> {
    async fn execute(&self, input: &Input) -> Result<Vec<HistoryEntry>> {
        let stream = input.account.stream_name();
        self.event_store.read_events(&stream).await
    }
}
