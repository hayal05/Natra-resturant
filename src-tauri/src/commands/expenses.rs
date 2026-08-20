use crate::{database::Database, errors::{AppError, AppResult}, models::{Expense, NewExpense}, services::expense_service};
use std::sync::Mutex;

fn lock_error() -> AppError {
    AppError { message: "Database lock failed".into() }
}

#[tauri::command]
pub fn list_expenses(db: tauri::State<'_, Mutex<Database>>) -> AppResult<Vec<Expense>> {
    let db = db.lock().map_err(|_| lock_error())?;
    expense_service::list_expenses(&db)
}

#[tauri::command]
pub fn add_expense(db: tauri::State<'_, Mutex<Database>>, input: NewExpense) -> AppResult<Expense> {
    let db = db.lock().map_err(|_| lock_error())?;
    expense_service::add_expense(&db, input)
}

#[tauri::command]
pub fn delete_expense(db: tauri::State<'_, Mutex<Database>>, id: i64) -> AppResult<()> {
    let db = db.lock().map_err(|_| lock_error())?;
    expense_service::delete_expense(&db, id)
}
