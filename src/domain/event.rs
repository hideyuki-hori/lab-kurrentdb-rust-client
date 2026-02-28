use serde::{Deserialize, Serialize};

use super::value_objects::{Amount, Category, Description};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccountEvent {
    #[serde(rename = "income")]
    Income {
        amount: Amount,
        description: Description,
    },
    #[serde(rename = "expense")]
    Expense {
        amount: Amount,
        category: Category,
        description: Description,
    },
}
