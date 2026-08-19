use std::sync::Mutex;
use tauri::State;

use crate::database::Database;

#[tauri::command]
pub fn dashboard_summary(db: State<'_, Mutex<Database>>) -> Result<serde_json::Value, String> {
    db.lock()
        .map_err(|_| "Database lock failed".to_string())?
        .dashboard_summary()
        .map_err(|e| e.to_string())
}
