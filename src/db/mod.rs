// ============================================================
// src/db/mod.rs
// ============================================================
use rusqlite::Connection;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<Connection>,
}

pub fn init_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}