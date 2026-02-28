use super::event::AccountEvent;

pub struct AccountBalance {
    pub account: String,
    pub balance: i64,
    pub total_income: i64,
    pub total_expense: i64,
    pub event_count: i64,
}

#[derive(sqlx::FromRow)]
pub struct BalanceRow {
    pub balance: i64,
    pub total_income: i64,
    pub total_expense: i64,
    pub event_count: i64,
}

#[derive(sqlx::FromRow)]
pub struct AccountBalanceRow {
    pub account: String,
    pub balance: i64,
    pub total_income: i64,
    pub total_expense: i64,
    pub event_count: i64,
}

#[derive(sqlx::FromRow)]
pub struct CategoryExpenseRow {
    pub category: String,
    pub total: i64,
    pub count: i64,
}

pub struct CategoryExpense {
    pub category: String,
    pub total: i64,
    pub count: i64,
}

pub struct HistoryEntry {
    pub revision: u64,
    pub event: AccountEvent,
    pub timestamp: String,
}

pub struct ReceivedEvent {
    pub revision: u64,
    pub event: AccountEvent,
    pub hours: String,
    pub minutes: String,
}
