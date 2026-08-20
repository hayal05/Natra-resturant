use crate::database::Database;
use crate::errors::AppResult;
use crate::models::{Category, Item};
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct AddCategoryInput {
    pub name: String,
    pub item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AddItemInput {
    pub category_id: i64,
    pub name: String,
    pub purchase_cost: Option<f64>,
    pub selling_price: f64,
    pub quantity: f64,
}

#[tauri::command]
pub fn list_categories(db: State<'_, Mutex<Database>>) -> AppResult<Vec<Category>> {
    Ok(db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.list_categories()?)
}

#[tauri::command]
pub fn add_category(db: State<'_, Mutex<Database>>, input: AddCategoryInput) -> AppResult<Category> {
    db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.add_category(&input.name, &input.item_type).map_err(|message| crate::errors::AppError { message })
}

#[tauri::command]
pub fn remove_category(db: State<'_, Mutex<Database>>, id: i64) -> AppResult<()> {
    db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.remove_category(id).map_err(|message| crate::errors::AppError { message })
}

#[tauri::command]
pub fn list_items(db: State<'_, Mutex<Database>>, item_type: Option<String>) -> AppResult<Vec<Item>> {
    Ok(db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.list_items(item_type.as_deref())?)
}

#[tauri::command]
pub fn add_item(db: State<'_, Mutex<Database>>, input: AddItemInput) -> AppResult<Item> {
    db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.add_item(input.category_id, &input.name, input.purchase_cost, input.selling_price, input.quantity).map_err(|message| crate::errors::AppError { message })
}

#[tauri::command]
pub fn remove_item(db: State<'_, Mutex<Database>>, id: i64) -> AppResult<()> {
    Ok(db.lock().map_err(|_| crate::errors::AppError { message: "Database lock failed".into() })?.remove_item(id)?)
}
