use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub category_id: i64,
    pub category_name: String,
    pub name: String,
    pub item_type: String,
    pub purchase_cost: Option<f64>,
    pub selling_price: f64,
    pub quantity: f64,
    pub active: bool,
}
