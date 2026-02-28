use anyhow::Result;

use crate::domain::Account;
use crate::infrastructure::event_store::EventStore;

pub struct Op<'a> {
    pub event_store: &'a EventStore,
}

impl super::Operation for Op<'_> {
    async fn execute(&self, account: &Account) -> Result<super::WatchSession> {
        let stream = account.stream_name();
        let state = self.event_store.rebuild_state(&stream).await?;
        let subscription = self.event_store.subscribe_from_end(stream).await;
        Ok(super::WatchSession { state, subscription })
    }
}
