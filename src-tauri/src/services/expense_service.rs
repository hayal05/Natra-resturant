use crate::{database::Database, errors::AppResult, models::{Expense, NewExpense}};

fn validation_error(message: &str) -> crate::errors::AppError {
    message.into()
}

pub fn list_expenses(db: &Database) -> AppResult<Vec<Expense>> {
    let conn = &db.conn;
    let mut stmt = conn.prepare(
        "SELECT id, description, category, amount, created_at
         FROM expenses
         ORDER BY created_at DESC, id DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Expense {
            id: row.get(0)?,
            description: row.get(1)?,
            category: row.get(2)?,
            amount: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn add_expense(db: &Database, input: NewExpense) -> AppResult<Expense> {
    if input.description.trim().is_empty() {
        return Err(validation_error("Expense description is required"));
    }
    if input.category.trim().is_empty() {
        return Err(validation_error("Expense category is required"));
    }
    if input.amount <= 0.0 {
        return Err(validation_error("Expense amount must be greater than zero"));
    }

    let conn = &db.conn;
    conn.execute(
        "INSERT INTO expenses (description, category, amount, note)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            input.description.trim(),
            input.category.trim(),
            input.amount,
            input.note.as_deref().map(str::trim).filter(|s| !s.is_empty())
        ],
    )?;

    let id = conn.last_insert_rowid();
    Ok(conn.query_row(
        "SELECT id, description, category, amount, created_at
         FROM expenses WHERE id = ?1",
        [id],
        |row| {
            Ok(Expense {
                id: row.get(0)?,
                description: row.get(1)?,
                category: row.get(2)?,
                amount: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    )?)
}

pub fn delete_expense(db: &Database, id: i64) -> AppResult<()> {
    let conn = &db.conn;
    let affected = conn.execute("DELETE FROM expenses WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(validation_error("Expense not found"));
    }
    Ok(())
}
