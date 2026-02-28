use thiserror::Error;

use super::value_objects::Amount;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("insufficient balance ({balance} < {required})")]
    InsufficientBalance { balance: u64, required: Amount },
    #[error("concurrency conflict after {retries} retries")]
    ConcurrencyConflict { retries: usize },
}
