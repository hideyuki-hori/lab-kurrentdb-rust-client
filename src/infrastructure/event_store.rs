use anyhow::{Context, Result};
use kurrentdb::{
    AppendToStreamOptions, Client, EventData, StreamPosition, StreamState,
    SubscribeToAllOptions, SubscribeToStreamOptions, Subscription,
};

use crate::domain::query::{HistoryEntry, ReceivedEvent};
use crate::domain::{AccountEvent, AccountState};

use super::env::Env;

pub struct EventStore {
    client: Client,
}

impl EventStore {
    pub fn connect(env: &Env) -> Result<Self> {
        let settings = env.kurrentdb_url.parse()
            .context("failed to parse KURRENTDB_URL")?;
        let client = Client::new(settings)
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("failed to create KurrentDB client")?;
        Ok(Self { client })
    }

    pub async fn rebuild_state(&self, stream: &str) -> Result<AccountState> {
        let mut state = AccountState::default();
        let mut read = self.client
            .read_stream(stream.to_owned(), &Default::default())
            .await?;
        while let Some(event) = read.next().await? {
            let recorded = event.get_original_event();
            let account_event = recorded.as_json::<AccountEvent>()?;
            state = state.apply(&account_event, recorded.revision);
        }
        Ok(state)
    }

    pub async fn read_events(&self, stream: &str) -> Result<Vec<HistoryEntry>> {
        let mut read = self.client
            .read_stream(stream.to_owned(), &Default::default())
            .await?;
        let mut entries = Vec::new();
        while let Some(event) = read.next().await? {
            let recorded = event.get_original_event();
            let account_event = recorded.as_json::<AccountEvent>()?;
            entries.push(HistoryEntry {
                revision: recorded.revision,
                event: account_event,
                timestamp: recorded.created.format("%Y-%m-%d %H:%M").to_string(),
            });
        }
        Ok(entries)
    }

    pub async fn append_event(
        &self,
        stream: &str,
        state: &AccountState,
        event_type: &str,
        event: &AccountEvent,
    ) -> Result<AppendResult> {
        let event_data = EventData::json(event_type, event)?;
        let stream_state = if state.event_count == 0 {
            StreamState::NoStream
        } else {
            StreamState::StreamRevision(state.revision)
        };
        let options = AppendToStreamOptions::default().stream_state(stream_state);
        match self.client
            .append_to_stream(stream.to_owned(), &options, event_data)
            .await
        {
            Ok(result) => Ok(AppendResult::Ok(result.next_expected_version)),
            Err(e) => {
                if matches!(&e, kurrentdb::Error::WrongExpectedVersion { .. }) {
                    Ok(AppendResult::ConcurrencyConflict)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn append_event_simple(
        &self,
        stream: &str,
        event_type: &str,
        event: &AccountEvent,
    ) -> Result<u64> {
        let event_data = EventData::json(event_type, event)?;
        let result = self.client
            .append_to_stream(stream.to_owned(), &Default::default(), event_data)
            .await?;
        Ok(result.next_expected_version)
    }

    pub async fn subscribe_from_end(&self, stream: String) -> EventSubscription {
        let options = SubscribeToStreamOptions::default().start_from(StreamPosition::End);
        let subscription = self.client.subscribe_to_stream(stream, &options).await;
        EventSubscription { subscription }
    }

    pub async fn subscribe_to_all(&self, options: &SubscribeToAllOptions) -> Subscription {
        self.client.subscribe_to_all(options).await
    }
}

pub enum AppendResult {
    Ok(u64),
    ConcurrencyConflict,
}

pub struct EventSubscription {
    subscription: Subscription,
}

impl EventSubscription {
    pub async fn next(&mut self) -> Result<ReceivedEvent> {
        let event = self.subscription.next().await?;
        let recorded = event.get_original_event();
        let account_event = recorded.as_json::<AccountEvent>()?;
        Ok(ReceivedEvent {
            revision: recorded.revision,
            event: account_event,
            hours: recorded.created.format("%H").to_string(),
            minutes: recorded.created.format("%M").to_string(),
        })
    }
}
