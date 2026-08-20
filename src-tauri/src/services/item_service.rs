use crate::database::Database;
use crate::models::{Category, Item};

impl Database {
    pub fn list_categories(&self) -> Result<Vec<Category>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id,name,item_type FROM categories ORDER BY name COLLATE NOCASE")?;
        let rows = stmt.query_map([], |row| Ok(Category {
            id: row.get(0)?, name: row.get(1)?, item_type: row.get(2)?
        }))?;
        rows.collect()
    }

    pub fn add_category(&self, name: &str, item_type: &str) -> Result<Category, String> {
        let name = name.trim();
        if name.is_empty() { return Err("Category name is required".into()); }
        if !matches!(item_type, "COOKABLE" | "READY_MADE") { return Err("Invalid category type".into()); }
        self.conn.execute("INSERT INTO categories(name,item_type) VALUES(?,?)", rusqlite::params![name, item_type]).map_err(|e| e.to_string())?;
        let id = self.conn.last_insert_rowid();
        self.conn.query_row("SELECT id,name,item_type FROM categories WHERE id=?", [id], |r| Ok(Category { id:r.get(0)?, name:r.get(1)?, item_type:r.get(2)? })).map_err(|e| e.to_string())
    }

    pub fn remove_category(&self, id: i64) -> Result<(), String> {
        let count: i64 = self.conn.query_row("SELECT COUNT(*) FROM items WHERE category_id=? AND active=1", [id], |r| r.get(0)).map_err(|e| e.to_string())?;
        if count > 0 { return Err("Cannot remove a category that still has active items".into()); }
        self.conn.execute("DELETE FROM categories WHERE id=?", [id]).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_items(&self, item_type: Option<&str>) -> Result<Vec<Item>, rusqlite::Error> {
        let mut sql = String::from("SELECT i.id,i.category_id,c.name,i.name,i.item_type,i.purchase_cost,i.selling_price,i.quantity,i.active FROM items i JOIN categories c ON c.id=i.category_id WHERE i.active=1");
        if item_type.is_some() { sql.push_str(" AND i.item_type=?"); }
        sql.push_str(" ORDER BY c.name COLLATE NOCASE,i.name COLLATE NOCASE");
        let mut stmt = self.conn.prepare(&sql)?;
        let mapper = |row: &rusqlite::Row| Ok(Item { id:row.get(0)?, category_id:row.get(1)?, category_name:row.get(2)?, name:row.get(3)?, item_type:row.get(4)?, purchase_cost:row.get(5)?, selling_price:row.get(6)?, quantity:row.get(7)?, active:row.get::<_,i64>(8)? != 0 });
        let rows = match item_type { Some(t) => stmt.query_map([t], mapper)?, None => stmt.query_map([], mapper)? };
        rows.collect()
    }

    pub fn add_item(&self, category_id: i64, name: &str, purchase_cost: Option<f64>, selling_price: f64, quantity: f64) -> Result<Item, String> {
        let name = name.trim();
        if name.is_empty() { return Err("Item name is required".into()); }
        if selling_price < 0.0 || quantity < 0.0 { return Err("Price and quantity cannot be negative".into()); }
        let category_type: String = self.conn.query_row("SELECT item_type FROM categories WHERE id=?", [category_id], |r| r.get(0)).map_err(|_| "Category not found".to_string())?;
        let cost = if category_type == "READY_MADE" {
            let c = purchase_cost.ok_or_else(|| "Ready-made items require a purchase cost".to_string())?;
            if c < 0.0 { return Err("Purchase cost cannot be negative".into()); }
            Some(c)
        } else { None };
        self.conn.execute("INSERT INTO items(category_id,name,item_type,purchase_cost,selling_price,quantity) VALUES(?,?,?,?,?,?)", rusqlite::params![category_id,name,category_type,cost,selling_price,quantity]).map_err(|e| e.to_string())?;
        let id = self.conn.last_insert_rowid();
        self.conn.query_row("SELECT i.id,i.category_id,c.name,i.name,i.item_type,i.purchase_cost,i.selling_price,i.quantity,i.active FROM items i JOIN categories c ON c.id=i.category_id WHERE i.id=?", [id], |r| Ok(Item { id:r.get(0)?,category_id:r.get(1)?,category_name:r.get(2)?,name:r.get(3)?,item_type:r.get(4)?,purchase_cost:r.get(5)?,selling_price:r.get(6)?,quantity:r.get(7)?,active:r.get::<_,i64>(8)? != 0 })).map_err(|e| e.to_string())
    }

    pub fn remove_item(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.conn.execute("UPDATE items SET active=0 WHERE id=? AND active=1", [id])?;
        Ok(())
    }
}
