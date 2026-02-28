use anyhow::Result;

use crate::usecases;
use crate::infrastructure::event_store::EventStore;
use crate::infrastructure::projection::Projection;

pub struct Resolver<'a> {
    event_store: &'a EventStore,
    projection: &'a Projection,
}

impl<'a> Resolver<'a> {
    pub fn new(event_store: &'a EventStore, projection: &'a Projection) -> Self {
        Self { event_store, projection }
    }

    pub async fn income(&self, args: usecases::income::Args) -> Result<()> {
        usecases::income::UseCase::new(
            usecases::income::operation::Op { event_store: self.event_store },
            usecases::income::presentation::View,
        ).run(args).await
    }

    pub async fn expense(&self, args: usecases::expense::Args) -> Result<()> {
        usecases::expense::UseCase::new(
            usecases::expense::operation::Op { event_store: self.event_store },
            usecases::expense::presentation::View,
        ).run(args).await
    }

    pub async fn history(&self, args: usecases::history::Args) -> Result<()> {
        usecases::history::UseCase::new(
            usecases::history::operation::Op { event_store: self.event_store },
            usecases::history::presentation::View,
        ).run(args).await
    }

    pub async fn balance(&self, args: usecases::balance::Args) -> Result<()> {
        usecases::balance::UseCase::new(
            usecases::balance::operation::Op { projection: self.projection },
            usecases::balance::presentation::View,
        ).run(args).await
    }

    pub async fn watch(&self, args: usecases::watch::Args) -> Result<()> {
        usecases::watch::UseCase::new(
            usecases::watch::operation::Op { event_store: self.event_store },
            usecases::watch::presentation::View,
        ).run(args).await
    }

    pub async fn projector(&self) -> Result<()> {
        usecases::projector::UseCase::new(
            usecases::projector::operation::Op {
                event_store: self.event_store,
                projection: self.projection,
            },
        ).run().await
    }

    pub async fn stats_category(&self) -> Result<()> {
        usecases::stats_category::UseCase::new(
            usecases::stats_category::operation::Op { projection: self.projection },
            usecases::stats_category::presentation::View,
        ).run().await
    }

    pub async fn stats_summary(&self) -> Result<()> {
        usecases::stats_summary::UseCase::new(
            usecases::stats_summary::operation::Op { projection: self.projection },
            usecases::stats_summary::presentation::View,
        ).run().await
    }
}
