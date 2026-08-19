use crate::{database::Database, errors::AppResult, models::Waiter, services::waiter_service};

#[tauri::command]
pub fn list_waiters(db: tauri::State<'_, Database>) -> AppResult<Vec<Waiter>> {
    waiter_service::list_active(&db)
}

#[tauri::command]
pub fn add_waiter(db: tauri::State<'_, Database>, name: String) -> AppResult<Waiter> {
    waiter_service::add(&db, &name)
}

#[tauri::command]
pub fn remove_waiter(db: tauri::State<'_, Database>, id: i64) -> AppResult<()> {
    waiter_service::deactivate(&db, id)
}
