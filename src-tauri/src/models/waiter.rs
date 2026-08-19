use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Waiter {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub created_at: String,
    pub today_receivable: f64,
}
