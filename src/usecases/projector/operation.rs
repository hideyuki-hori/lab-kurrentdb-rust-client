use anyhow::Result;
use kurrentdb::{Position, StreamPosition, SubscribeToAllOptions, SubscriptionFilter};

use crate::domain::AccountEvent;
use crate::infrastructure::event_store::EventStore;
use crate::infrastructure::projection::Projection;

const CHECKPOINT_ID: &str = "account-projector";
const UPSERT_INCOME: &str = include_str!("upsert_income.sql");
const UPSERT_EXPENSE_BALANCE: &str = include_str!("upsert_expense_balance.sql");
const UPSERT_EXPENSE_CATEGORY: &str = include_str!("upsert_expense_category.sql");
const LOAD_CHECKPOINT: &str = include_str!("load_checkpoint.sql");
const SAVE_CHECKPOINT: &str = include_str!("save_checkpoint.sql");

pub struct Op<'a> {
    pub event_store: &'a EventStore,
    pub projection: &'a Projection,
}

impl super::Operation for Op<'_> {
    async fn execute(&self) -> Result<()> {
        let position = self.load_checkpoint().await?;

        let filter = SubscriptionFilter::on_stream_name().add_prefix("account-");
        let options = match position {
            Some(pos) => SubscribeToAllOptions::default()
                .position(StreamPosition::Position(pos))
                .filter(filter),
            None => SubscribeToAllOptions::default()
                .position(StreamPosition::Start)
                .filter(filter),
        };

        let mut subscription = self.event_store.subscribe_to_all(&options).await;

        println!("projector started. waiting for events...");

        loop {
            let event = subscription.next().await?;
            let recorded = event.get_original_event();

            if recorded.event_type.starts_with('$') {
                continue;
            }

            let account = recorded
                .stream_id()
                .strip_prefix("account-")
                .unwrap_or(recorded.stream_id());

            let account_event = recorded.as_json::<AccountEvent>()?;

            let mut tx = self.projection.begin().await?;
            Self::apply_event(&mut *tx, account, &account_event).await?;
            Self::save_checkpoint(&mut *tx, recorded.position).await?;
            tx.commit().await?;

            let label = match &account_event {
                AccountEvent::Income { amount, .. } => {
                    format!("+{} -> {}", amount, account)
                }
                AccountEvent::Expense {
                    amount, category, ..
                } => format!("-{} [{}] -> {}", amount, category, account),
            };
            println!("  projected: {}", label);
        }
    }
}

impl Op<'_> {
    async fn load_checkpoint(&self) -> Result<Option<Position>> {
        let row: Option<(i64, i64)> = self
            .projection
            .query_one(LOAD_CHECKPOINT, &[CHECKPOINT_ID])
            .await?;

        Ok(row.map(|(commit, prepare)| Position {
            commit: commit as u64,
            prepare: prepare as u64,
        }))
    }

    async fn apply_event(
        conn: &mut sqlx::PgConnection,
        account: &str,
        event: &AccountEvent,
    ) -> Result<()> {
        match event {
            AccountEvent::Income { amount, .. } => {
                let amount = amount.value() as i64;
                sqlx::query(UPSERT_INCOME)
                    .bind(account)
                    .bind(amount)
                    .execute(&mut *conn)
                    .await?;
            }
            AccountEvent::Expense {
                amount, category, ..
            } => {
                let amount = amount.value() as i64;
                sqlx::query(UPSERT_EXPENSE_BALANCE)
                    .bind(account)
                    .bind(amount)
                    .execute(&mut *conn)
                    .await?;

                sqlx::query(UPSERT_EXPENSE_CATEGORY)
                    .bind(category.as_ref())
                    .bind(amount)
                    .execute(&mut *conn)
                    .await?;
            }
        }
        Ok(())
    }

    async fn save_checkpoint(
        conn: &mut sqlx::PgConnection,
        position: Position,
    ) -> Result<()> {
        sqlx::query(SAVE_CHECKPOINT)
            .bind(CHECKPOINT_ID)
            .bind(position.commit as i64)
            .bind(position.prepare as i64)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }
}
