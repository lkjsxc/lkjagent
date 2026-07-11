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
    if held_by.as_deref().is_some_and(|held| held != owner) && !stale(&held_by, &heartbeat, now) {
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

pub fn release(conn: &Connection) -> Result<(), String> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    let held_by: Option<String> = tx
        .query_row(
            "SELECT value FROM config WHERE key = ?1",
            [OWNER_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let owner = owner_id();
    if held_by.as_deref() == Some(owner.as_str()) {
        tx.execute(
            "DELETE FROM config WHERE key IN (?1, ?2)",
            params![OWNER_KEY, HEARTBEAT_KEY],
        )
        .map_err(|error| error.to_string())?;
    }
    tx.commit().map_err(|error| error.to_string())
}

fn stale(owner: &Option<String>, heartbeat: &Option<String>, now: &str) -> bool {
    if let Some(live) = owner.as_deref().and_then(owner_is_live) {
        return !live;
    }
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

fn owner_is_live(owner: &str) -> Option<bool> {
    let mut fields = owner.split(':');
    if fields.next()? != "pid" {
        return None;
    }
    let pid = fields.next()?.parse::<u32>().ok()?;
    if fields.next()? != "start" {
        return None;
    }
    let expected = fields.next()?;
    process_start(pid).map(|found| found == expected)
}

fn process_start(pid: u32) -> Option<String> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let mut fields = stat.get(stat.rfind(") ")? + 2..)?.split_whitespace();
    fields.nth(19).map(str::to_string)
}

fn owner_id() -> String {
    let pid = std::process::id();
    let start = process_start(pid).unwrap_or_else(|| "unknown".to_string());
    format!(
        "pid:{pid}:start:{start}:thread:{:?}",
        std::thread::current().id()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_threads_have_distinct_owners() -> Result<(), String> {
        let current = owner_id();
        let other = std::thread::spawn(owner_id)
            .join()
            .map_err(|_| "owner thread failed".to_string())?;
        if current == other {
            Err("thread owners collided".to_string())
        } else {
            Ok(())
        }
    }

    #[test]
    fn live_process_owner_does_not_expire() -> Result<(), String> {
        let mut conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        conn.execute(
            "CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .map_err(|error| error.to_string())?;
        claim(&mut conn, "unix:1000.0")?;
        if claim_as(&mut conn, "foreign", "unix:2000.0").is_err() {
            Ok(())
        } else {
            Err("live process lock expired".to_string())
        }
    }

    #[test]
    fn current_owner_can_release_lock() -> Result<(), String> {
        let mut conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        conn.execute(
            "CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .map_err(|error| error.to_string())?;
        claim(&mut conn, "unix:1000.0")?;
        release(&conn)?;
        claim_as(&mut conn, "next", "unix:1001.0")
    }

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
