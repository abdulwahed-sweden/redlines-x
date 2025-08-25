// src/core/database.rs
use rusqlite::{Connection, Result};
use crate::core::session::Session;
use serde_json;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("redlines.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                target TEXT NOT NULL,
                cookies TEXT,
                headers TEXT,
                variables TEXT,
                created_at TEXT
            )",
            [],
        )?;

        Ok(Self { conn })
    }
    
    pub fn save_session(&self, session: &Session) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sessions (id, target, cookies, headers, variables, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &session.id,
                &session.target,
                &serde_json::to_string(&session.cookies).unwrap(),
                &serde_json::to_string(&session.headers).unwrap(),
                &serde_json::to_string(&session.variables).unwrap(),
                &session.created_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }
}
