use rusqlite::{params, Connection, OptionalExtension};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockDecision {
    Taken,
    Refused { holder: String },
    Reclaimed { previous: String },
}

pub fn get(conn: &Connection, key: &str) -> StoreResult<Option<String>> {
    let value = conn
        .query_row(
            "SELECT value FROM state WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()?;
    Ok(value)
}

pub fn set(conn: &Connection, key: &str, value: &str) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO state (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, key: &str) -> StoreResult<()> {
    conn.execute("DELETE FROM state WHERE key = ?1", params![key])?;
    Ok(())
}

pub fn maintenance_stamp(conn: &Connection, directive: &str) -> StoreResult<Option<String>> {
    get(conn, &maintenance_stamp_key(directive))
}

pub fn set_maintenance_stamp(conn: &Connection, directive: &str, value: &str) -> StoreResult<()> {
    set(conn, &maintenance_stamp_key(directive), value)
}

pub fn maintenance_stamp_key(directive: &str) -> String {
    format!("maintenance last-run {directive}")
}

pub fn take_lock(
    conn: &Connection,
    holder: &str,
    started_at: &str,
    stale_before: &str,
) -> StoreResult<LockDecision> {
    let current = get(conn, "daemon lock")?;
    let value = format!("{holder}|{started_at}");
    match current {
        None => {
            set(conn, "daemon lock", &value)?;
            Ok(LockDecision::Taken)
        }
        Some(existing) => {
            let stale = existing
                .split('|')
                .nth(1)
                .is_some_and(|stamp| stamp < stale_before);
            if stale {
                set(conn, "daemon lock", &value)?;
                Ok(LockDecision::Reclaimed { previous: existing })
            } else {
                Ok(LockDecision::Refused { holder: existing })
            }
        }
    }
}
