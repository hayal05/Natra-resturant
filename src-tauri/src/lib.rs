mod commands;
mod database;
mod errors;
mod models;
mod services;
mod sync;

use std::sync::Mutex;
use database::Database;

#[tauri::command]
fn app_initialized(db: tauri::State<Mutex<Database>>) -> Result<bool, String> { db.lock().map_err(|_| "Database lock failed".to_string()).and_then(|d| d.has_admin().map_err(Into::into)) }

#[tauri::command]
fn create_admin(username: String, password: String, full_name: String, db: tauri::State<Mutex<Database>>) -> Result<(), String> { db.lock().map_err(|_| "Database lock failed".to_string())?.create_admin(&username, &password, &full_name).map_err(Into::into) }

#[tauri::command]
fn login(username: String, password: String, db: tauri::State<Mutex<Database>>) -> Result<bool, String> { db.lock().map_err(|_| "Database lock failed".to_string())?.verify_login(&username, &password).map_err(Into::into) }

#[tauri::command]
fn dashboard_summary(db: tauri::State<Mutex<Database>>) -> Result<serde_json::Value, String> { db.lock().map_err(|_| "Database lock failed".to_string())?.dashboard_summary().map_err(Into::into) }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = Database::open().expect("failed to initialize local database");
    tauri::Builder::default()
        .manage(Mutex::new(database))
        .invoke_handler(tauri::generate_handler![app_initialized, create_admin, login, dashboard_summary])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
