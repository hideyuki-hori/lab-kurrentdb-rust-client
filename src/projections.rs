use serde::Deserialize;
use std::collections::HashMap;

pub const BALANCE_JS: &str = include_str!("../projections/dist/balance.js");
pub const CATEGORY_JS: &str = include_str!("../projections/dist/category.js");
pub const ALERT_JS: &str = include_str!("../projections/dist/alert.js");
pub const SUMMARY_JS: &str = include_str!("../projections/dist/summary.js");

pub const BALANCE_NAME: &str = "account-balance";
pub const CATEGORY_NAME: &str = "category-expense";
pub const ALERT_NAME: &str = "budget-alert";
pub const SUMMARY_NAME: &str = "account-summary";

pub const ALL_PROJECTIONS: &[(&str, &str, bool)] = &[
    (BALANCE_NAME, BALANCE_JS, false),
    (CATEGORY_NAME, CATEGORY_JS, false),
    (ALERT_NAME, ALERT_JS, true),
    (SUMMARY_NAME, SUMMARY_JS, false),
];

#[derive(Debug, Deserialize)]
pub struct BalanceState {
    pub balance: f64,
    pub total_income: f64,
    pub total_expense: f64,
    pub event_count: f64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryEntry {
    pub total: f64,
    pub count: f64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryState {
    pub categories: HashMap<String, CategoryEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SummaryState {
    pub total_income: f64,
    pub total_expense: f64,
    pub net: f64,
    pub accounts: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct BudgetExceededEvent {
    pub category: String,
    pub total: f64,
    pub triggered_by: f64,
    pub account: String,
}
