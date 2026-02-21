use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccountEvent {
    #[serde(rename = "income")]
    Income {
        amount: u64,
        description: String,
    },
    #[serde(rename = "expense")]
    Expense {
        amount: u64,
        category: String,
        description: String,
    },
}

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
                self.balance += *amount as i64;
                self.total_income += amount;
            }
            AccountEvent::Expense { amount, .. } => {
                self.balance -= *amount as i64;
                self.total_expense += amount;
            }
        }
        self.revision = revision;
        self.event_count += 1;
        self
    }
}

pub fn stream_name(account: &str) -> String {
    format!("account-{account}")
}