use serde::Serialize;
#[derive(Debug, Serialize)] pub struct AppError { pub message: String }
impl From<rusqlite::Error> for AppError { fn from(e: rusqlite::Error)->Self{Self{message:e.to_string()}} }
