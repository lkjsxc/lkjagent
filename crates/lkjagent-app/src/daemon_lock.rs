use rusqlite::{params, Connection};

const OWNER_KEY: &str = "daemon.lock.owner";
const HEARTBEAT_KEY: &str = "daemon.lock.heartbeat";
const STALE_SECONDS: u64 = 300;

pub fn claim(conn: &Connection, now: &str) -> Result<(), String> {
    let owner = owner_id();
    let held_by = config_value(conn, OWNER_KEY)?;
    let heartbeat = config_value(conn, HEARTBEAT_KEY)?;
    if held_by
        .as_deref()
        .is_some_and(|held| held != owner.as_str())
        && !stale(&heartbeat, now)
    {
        return Err(format!(
            "daemon lock held by {}",
            held_by.unwrap_or_default()
        ));
    }
    set_config(conn, OWNER_KEY, &owner)?;
    set_config(conn, HEARTBEAT_KEY, now)
}

fn stale(heartbeat: &Option<String>, now: &str) -> bool {
    let Some(heartbeat) = heartbeat else {
        return true;
    };
    let Some(previous) = unix_seconds(heartbeat) else {
        return true;
    };
    let Some(current) = unix_seconds(now) else {
        return true;
    };
    current.saturating_sub(previous) > STALE_SECONDS
}

fn unix_seconds(value: &str) -> Option<u64> {
    let rest = value.strip_prefix("unix:")?;
    let seconds = rest.split('.').next()?;
    seconds.parse().ok()
}

fn owner_id() -> String {
    format!("pid:{}", std::process::id())
}

fn config_value(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    let mut statement = conn
        .prepare("SELECT value FROM config WHERE key = ?1")
        .map_err(|error| error.to_string())?;
    let value = statement.query_row(params![key], |row| row.get(0));
    match value {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

fn set_config(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO config (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}
