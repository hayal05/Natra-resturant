use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMaterial {
    pub id: i64,
    pub name: String,
    pub unit: String,
    pub quantity: f64,
    pub unit_cost: f64,
    pub current_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMaterialPurchase {
    pub raw_material_id: i64,
    pub quantity: f64,
    pub unit_cost: f64,
    pub total_cost: f64,
    pub note: Option<String>,
}
