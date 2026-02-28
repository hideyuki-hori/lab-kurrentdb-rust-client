use super::error::DomainError;
use super::event::AccountEvent;
use super::value_objects::Amount;

#[derive(Debug, Clone, Default)]
pub struct AccountState {
    pub balance: i64,
    pub total_income: u64,
    pub total_expense: u64,
    pub revision: u64,
    pub event_count: usize,
}

impl AccountState {
    pub fn apply(mut self, event: &AccountEvent, revision: u64) -> Self {
        match event {
            AccountEvent::Income { amount, .. } => {
                self.balance += amount.value() as i64;
                self.total_income += amount.value();
            }
            AccountEvent::Expense { amount, .. } => {
                self.balance -= amount.value() as i64;
                self.total_expense += amount.value();
            }
        }
        self.revision = revision;
        self.event_count += 1;
        self
    }

    pub fn validate_expense(&self, amount: &Amount) -> Result<(), DomainError> {
        if self.balance < i64::try_from(amount.value()).unwrap_or(i64::MAX) {
            return Err(DomainError::InsufficientBalance {
                balance: self.balance.unsigned_abs(),
                required: *amount,
            });
        }
        Ok(())
    }
}
