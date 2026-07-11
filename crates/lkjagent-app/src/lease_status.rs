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
    value.strip_prefix("unix:")?.split('.').next()?.parse().ok()
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

struct TokenRow {
    total: Option<i64>,
    cached: Option<i64>,
    uncached: Option<i64>,
    output: Option<i64>,
    known: i64,
    unknown: i64,
    provider: i64,
    unsupported: i64,
}

pub fn token_line(conn: &Connection) -> Result<String, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM token_usage", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Ok("unknown".to_string());
    }
    let row = conn
        .query_row(
            "SELECT SUM(input_total_tokens), SUM(input_cached_tokens),
         SUM(input_uncached_tokens), SUM(output_tokens),
         SUM(cache_status = 'known'), SUM(cache_status = 'unknown'),
         SUM(cache_status = 'provider_specific'), SUM(cache_status = 'not_supported')
         FROM token_usage",
            [],
            |row| {
                Ok(TokenRow {
                    total: row.get(0)?,
                    cached: row.get(1)?,
                    uncached: row.get(2)?,
                    output: row.get(3)?,
                    known: row.get(4)?,
                    unknown: row.get(5)?,
                    provider: row.get(6)?,
                    unsupported: row.get(7)?,
                })
            },
        )
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "input_uncached={} input_cached={} input_total={} output={} cache={}",
        fmt_token(row.uncached),
        fmt_token(row.cached),
        fmt_token(row.total),
        fmt_token(row.output),
        cache_label(row.known, row.unknown, row.provider, row.unsupported)
    ))
}

fn fmt_token(value: Option<i64>) -> String {
    value.map_or_else(|| "unknown".to_string(), |value| value.to_string())
}

fn cache_label(known: i64, unknown: i64, provider: i64, unsupported: i64) -> &'static str {
    if provider > 0 {
        "provider_specific"
    } else if unsupported > 0 {
        "not_supported"
    } else if unknown > 0 || known == 0 {
        "unknown"
    } else {
        "known"
    }
}
