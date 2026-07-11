use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

const OWNER_KEY: &str = "daemon.lock.owner";
const HEARTBEAT_KEY: &str = "daemon.lock.heartbeat";
const STALE_SECONDS: u64 = 300;

pub fn claim(conn: &mut Connection, now: &str) -> Result<(), String> {
    claim_as(conn, &owner_id(), now)
}

fn claim_as(conn: &mut Connection, owner: &str, now: &str) -> Result<(), String> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|error| error.to_string())?;
    let held_by: Option<String> = tx
        .query_row(
            "SELECT value FROM config WHERE key = ?1",
            [OWNER_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let heartbeat: Option<String> = tx
        .query_row(
            "SELECT value FROM config WHERE key = ?1",
            [HEARTBEAT_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    if held_by.as_deref().is_some_and(|held| held != owner) && !stale(&heartbeat, now) {
        return Err(format!(
            "daemon lock held by {}",
            held_by.unwrap_or_default()
        ));
    }
    for (key, value) in [(OWNER_KEY, owner), (HEARTBEAT_KEY, now)] {
        tx.execute(
            "INSERT INTO config (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )
        .map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())
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
    rest.split('.').next()?.parse().ok()
}

fn owner_id() -> String {
    format!("pid:{}", std::process::id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_foreign_owner_cannot_replace_lock() -> Result<(), String> {
        let mut conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        conn.execute(
            "CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .map_err(|error| error.to_string())?;
        claim_as(&mut conn, "first", "unix:1000.0")?;
        assert!(claim_as(&mut conn, "second", "unix:1001.0").is_err());
        let owner: String = conn
            .query_row(
                "SELECT value FROM config WHERE key = ?1",
                [OWNER_KEY],
                |row| row.get(0),
            )
            .map_err(|error| error.to_string())?;
        assert_eq!(owner, "first");
        Ok(())
    }
}
