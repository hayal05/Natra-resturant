use std::sync::Mutex;
use crate::{database::Database, errors::AppResult, models::Waiter, services::waiter_service};

fn lock_error() -> crate::errors::AppError {
    crate::errors::AppError { message: "Database lock failed".into() }
}

#[tauri::command]
pub fn list_waiters(db: tauri::State<'_, Mutex<Database>>) -> AppResult<Vec<Waiter>> {
    let db = db.lock().map_err(|_| lock_error())?;
    waiter_service::list_active(&db)
}

#[tauri::command]
pub fn add_waiter(db: tauri::State<'_, Mutex<Database>>, name: String) -> AppResult<Waiter> {
    let db = db.lock().map_err(|_| lock_error())?;
    waiter_service::add(&db, &name)
}

#[tauri::command]
pub fn remove_waiter(db: tauri::State<'_, Mutex<Database>>, id: i64) -> AppResult<()> {
    let db = db.lock().map_err(|_| lock_error())?;
    waiter_service::deactivate(&db, id)
}
