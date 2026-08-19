use std::sync::Mutex;
use tauri::State;

use crate::database::Database;

#[tauri::command]
pub fn app_initialized(db: State<'_, Mutex<Database>>) -> Result<bool, String> {
    db.lock()
        .map_err(|_| "Database lock failed".to_string())?
        .has_admin()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_admin(
    username: String,
    password: String,
    full_name: String,
    db: State<'_, Mutex<Database>>,
) -> Result<(), String> {
    db.lock()
        .map_err(|_| "Database lock failed".to_string())?
        .create_admin(&username, &password, &full_name)
}

#[tauri::command]
pub fn login(
    username: String,
    password: String,
    db: State<'_, Mutex<Database>>,
) -> Result<bool, String> {
    db.lock()
        .map_err(|_| "Database lock failed".to_string())?
        .verify_login(&username, &password)
        .map_err(|e| e.to_string())
}
