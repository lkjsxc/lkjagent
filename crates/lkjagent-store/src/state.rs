use rusqlite::{params, Connection, OptionalExtension};

use crate::error::StoreResult;

const LOCK_KEY: &str = "daemon lock";

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
    let current = get(conn, LOCK_KEY)?;
    let value = format!("{holder}|{started_at}|{started_at}");
    match current {
        None => {
            set(conn, LOCK_KEY, &value)?;
            Ok(LockDecision::Taken)
        }
        Some(existing) => {
            if lock_is_stale(&existing, stale_before) {
                set(conn, LOCK_KEY, &value)?;
                Ok(LockDecision::Reclaimed { previous: existing })
            } else {
                Ok(LockDecision::Refused { holder: existing })
            }
        }
    }
}

pub fn heartbeat_lock(conn: &Connection, holder: &str, heartbeat_at: &str) -> StoreResult<bool> {
    let Some(existing) = get(conn, LOCK_KEY)? else {
        return Ok(false);
    };
    let Some(lock) = parse_lock(&existing) else {
        return Ok(false);
    };
    if lock.holder != holder {
        return Ok(false);
    }
    set(
        conn,
        LOCK_KEY,
        &format!("{holder}|{}|{heartbeat_at}", lock.started_at),
    )?;
    Ok(true)
}

fn lock_is_stale(existing: &str, stale_before: &str) -> bool {
    parse_lock(existing).is_some_and(|lock| stamp_lt(lock.heartbeat_at, stale_before))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LockValue<'a> {
    holder: &'a str,
    started_at: &'a str,
    heartbeat_at: &'a str,
}

fn parse_lock(value: &str) -> Option<LockValue<'_>> {
    let mut parts = value.split('|');
    let holder = parts.next()?;
    let started_at = parts.next()?;
    let heartbeat_at = parts.next().unwrap_or(started_at);
    Some(LockValue {
        holder,
        started_at,
        heartbeat_at,
    })
}

fn stamp_lt(left: &str, right: &str) -> bool {
    match (left.parse::<u64>(), right.parse::<u64>()) {
        (Ok(left), Ok(right)) => left < right,
        _ => left < right,
    }
}
