use anyhow::Result;

use crate::domain::{AccountEvent, Amount, Category, Description, DomainError};
use crate::infrastructure::event_store::{AppendResult, EventStore};

pub struct Input {
    pub account: crate::domain::Account,
    pub amount: Amount,
    pub category: Category,
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
        let max_retries = 3;

        for attempt in 0..max_retries {
            let state = self.event_store.rebuild_state(&stream).await?;

            state.validate_expense(&input.amount)?;

            let event = AccountEvent::Expense {
                amount: input.amount,
                category: input.category.clone(),
                description: input.description.clone(),
            };

            match self.event_store.append_event(&stream, &state, "expense", &event).await? {
                AppendResult::Ok(revision) => return Ok(Output { revision }),
                AppendResult::ConcurrencyConflict => {
                    if attempt >= max_retries - 1 {
                        return Err(DomainError::ConcurrencyConflict { retries: max_retries }.into());
                    }
                }
            }
        }

        unreachable!()
    }
}
