use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i64,
    pub description: String,
    pub category: String,
    pub amount: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpense {
    pub description: String,
    pub category: String,
    pub amount: f64,
    pub note: Option<String>,
}
