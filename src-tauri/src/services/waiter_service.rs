use crate::{database::Database, errors::AppResult, models::Waiter};

pub fn list_active(db: &Database) -> AppResult<Vec<Waiter>> {
    db.list_active_waiters().map_err(Into::into)
}

pub fn add(db: &Database, name: &str) -> AppResult<Waiter> {
    db.add_waiter(name).map_err(Into::into)
}

pub fn deactivate(db: &Database, id: i64) -> AppResult<()> {
    db.deactivate_waiter(id).map_err(Into::into)
}
