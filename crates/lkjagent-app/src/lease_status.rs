use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

const OWNER_KEY: &str = "daemon.lock.owner";
const HEARTBEAT_KEY: &str = "daemon.lock.heartbeat";
const STALE_SECONDS: u64 = 300;

pub fn line(conn: &Connection) -> Result<String, String> {
    let owner = config_value(conn, OWNER_KEY)?.unwrap_or_else(|| "unknown".to_string());
    let heartbeat = config_value(conn, HEARTBEAT_KEY)?.unwrap_or_else(|| "unknown".to_string());
    Ok(format!(
        "lease: {} owner={} heartbeat={}",
        lease_state(&heartbeat),
        owner,
        heartbeat
    ))
}

fn lease_state(heartbeat: &str) -> &'static str {
    let Some(previous) = unix_seconds(heartbeat) else {
        return "unknown";
    };
    let Some(current) = now_seconds() else {
        return "unknown";
    };
    if current.saturating_sub(previous) > STALE_SECONDS {
        "stale"
    } else {
        "active"
    }
}

fn now_seconds() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn unix_seconds(value: &str) -> Option<u64> {
    let rest = value.strip_prefix("unix:")?;
    let seconds = rest.split('.').next()?;
    seconds.parse().ok()
}

fn config_value(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    let value = conn.query_row(
        "SELECT value FROM config WHERE key = ?1",
        params![key],
        |row| row.get(0),
    );
    match value {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}
