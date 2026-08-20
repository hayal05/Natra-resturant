use crate::{database::Database, errors::AppResult, models::{Expense, NewExpense}, services::expense_service};
use std::sync::Mutex;

#[tauri::command]
pub fn list_expenses(db: tauri::State<'_, Mutex<Database>>) -> AppResult<Vec<Expense>> {
    expense_service::list_expenses(&db.lock().map_err(|_| "database lock poisoned")?)
}

#[tauri::command]
pub fn add_expense(db: tauri::State<'_, Mutex<Database>>, input: NewExpense) -> AppResult<Expense> {
    expense_service::add_expense(&db.lock().map_err(|_| "database lock poisoned")?, input)
}

#[tauri::command]
pub fn delete_expense(db: tauri::State<'_, Mutex<Database>>, id: i64) -> AppResult<()> {
    expense_service::delete_expense(&db.lock().map_err(|_| "database lock poisoned")?, id)
}
