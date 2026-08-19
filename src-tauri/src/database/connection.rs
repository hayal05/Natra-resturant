use rusqlite::{params, Connection};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use std::path::PathBuf;
use crate::models::Waiter;

pub struct Database { conn: Connection }

impl Database {
    pub fn open() -> Result<Self, rusqlite::Error> {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("natra_restaurant.db");
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(include_str!("../../../migrations/001_initial.sql"))?;
        Ok(())
    }

    pub fn has_admin(&self) -> Result<bool, rusqlite::Error> {
        self.conn.query_row("SELECT EXISTS(SELECT 1 FROM users WHERE is_admin=1)", [], |r| r.get(0))
    }

    pub fn create_admin(&self, u: &str, p: &str, n: &str) -> Result<(), String> {
        if self.has_admin().map_err(|e| e.to_string())? {
            return Err("Administrator account already exists".into());
        }
        if u.trim().len() < 3 || p.len() < 8 {
            return Err("Username must be 3+ characters and password 8+ characters".into());
        }
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default().hash_password(p.as_bytes(), &salt).map_err(|e| e.to_string())?.to_string();
        self.conn.execute(
            "INSERT INTO users(username,full_name,password_hash,is_admin) VALUES(?,?,?,1)",
            params![u.trim(), n.trim(), hash],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn verify_login(&self, u: &str, p: &str) -> Result<bool, rusqlite::Error> {
        let hash = self.conn.query_row("SELECT password_hash FROM users WHERE username=?", [u], |r| r.get::<_, String>(0));
        match hash {
            Ok(h) => Ok(PasswordHash::new(&h).map(|x| Argon2::default().verify_password(p.as_bytes(), &x).is_ok()).unwrap_or(false)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn list_active_waiters(&self) -> Result<Vec<Waiter>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT w.id, w.name, w.active, w.created_at,
                    COALESCE(SUM(CASE WHEN date(s.created_at)=date('now','localtime') THEN s.total ELSE 0 END),0)
             FROM waiters w
             LEFT JOIN sales s ON s.waiter_id=w.id
             WHERE w.active=1
             GROUP BY w.id, w.name, w.active, w.created_at
             ORDER BY w.name COLLATE NOCASE"
        )?;
        let rows = stmt.query_map([], |row| Ok(Waiter {
            id: row.get(0)?,
            name: row.get(1)?,
            active: row.get::<_, i64>(2)? != 0,
            created_at: row.get(3)?,
            today_receivable: row.get(4)?,
        }))?;
        rows.collect()
    }

    pub fn add_waiter(&self, name: &str) -> Result<Waiter, rusqlite::Error> {
        let name = name.trim();
        if name.is_empty() {
            return Err(rusqlite::Error::InvalidParameterName("Waiter name is required".into()));
        }
        self.conn.execute("INSERT INTO waiters(name) VALUES(?)", [name])?;
        let id = self.conn.last_insert_rowid();
        self.conn.query_row(
            "SELECT id,name,active,created_at FROM waiters WHERE id=?",
            [id],
            |row| Ok(Waiter {
                id: row.get(0)?,
                name: row.get(1)?,
                active: row.get::<_, i64>(2)? != 0,
                created_at: row.get(3)?,
                today_receivable: 0.0,
            }),
        )
    }

    pub fn deactivate_waiter(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.conn.execute("UPDATE waiters SET active=0 WHERE id=? AND active=1", [id])?;
        Ok(())
    }

    pub fn dashboard_summary(&self) -> Result<serde_json::Value, rusqlite::Error> {
        let sales: f64 = self.conn.query_row("SELECT COALESCE(SUM(total),0) FROM sales", [], |r| r.get(0))?;
        let ready: f64 = self.conn.query_row("SELECT COALESCE(SUM(cost),0) FROM sales", [], |r| r.get(0))?;
        let raw: f64 = self.conn.query_row("SELECT COALESCE(SUM(cost),0) FROM raw_material_transactions WHERE type='PURCHASE'", [], |r| r.get(0))?;
        let exp: f64 = self.conn.query_row("SELECT COALESCE(SUM(amount),0) FROM expenses", [], |r| r.get(0))?;
        Ok(serde_json::json!({"sales":sales,"ready_made_cost":ready,"raw_material_cost":raw,"other_expenses":exp,"total_costs":ready+raw+exp,"net_profit":sales-ready-raw-exp}))
    }
}
