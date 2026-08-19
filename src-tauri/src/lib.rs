mod commands;
mod database;
mod errors;
mod models;
mod services;
mod sync;

use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub db_path: Mutex<Option<String>>,
}

#[tauri::command]
fn app_status() -> &'static str {
    "local"
}

#[tauri::command]
fn initialize_database(state: State<'_, AppState>) -> Result<String, String> {
    let mut path = state.db_path.lock().map_err(|_| "state lock failed")?;
    if path.is_none() {
        *path = Some("local restaurant database".to_string());
    }
    Ok("initialized".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState { db_path: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![app_status, initialize_database])
        .run(tauri::generate_context!())
        .expect("error while running NATRA Restaurant Management");
}
