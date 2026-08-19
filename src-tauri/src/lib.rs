mod commands;
mod database;
mod errors;
mod models;
mod services;
mod sync;

use database::Database;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = Database::open().expect("failed to initialize local database");

    tauri::Builder::default()
        .manage(Mutex::new(database))
        .invoke_handler(tauri::generate_handler![
            commands::auth::app_initialized,
            commands::auth::create_admin,
            commands::auth::login,
            commands::dashboard::dashboard_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
