use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};

const MANIFEST_KEY: &str = "workspace.inventory.manifest";
const CHECKED_KEY: &str = "workspace.inventory.checked_millis";
const RECONCILED_KEY: &str = "workspace.inventory.reconciled_millis";

pub fn rebuild(conn: &Connection, workspace: &Path) -> Result<String, String> {
    let (output, manifest) = stable_rebuild(conn, workspace)?;
    if let Some(manifest) = manifest {
        store_scan(conn, &manifest, now_millis())?;
        Ok(output)
    } else {
        Ok(format!("{output} unstable=1"))
    }
}

pub fn reconcile_entry(
    conn: &Connection,
    workspace: &Path,
    data_dir: &Path,
) -> Result<String, String> {
    let now = now_millis();
    let (debounce, reconcile_seconds) = crate::config::workspace_scan_timing(data_dir)?;
    let checked = value(conn, CHECKED_KEY)?.and_then(|value| value.parse::<u64>().ok());
    if checked.is_some_and(|prior| now.saturating_sub(prior) < debounce) {
        return Ok("workspace reconciliation debounced: interval".to_string());
    }
    let manifest = crate::workspace_search::inventory::source_manifest(workspace)?;
    let prior = value(conn, MANIFEST_KEY)?;
    let reconciled = value(conn, RECONCILED_KEY)?.and_then(|value| value.parse::<u64>().ok());
    let full_due = reconciled
        .is_none_or(|prior| now.saturating_sub(prior) >= reconcile_seconds.saturating_mul(1_000));
    if prior.as_deref() == Some(manifest.as_str()) && !full_due {
        put(conn, CHECKED_KEY, &now.to_string())?;
        return Ok("workspace reconciliation debounced: unchanged".to_string());
    }
    let (output, stable) = stable_rebuild(conn, workspace)?;
    if let Some(stable) = stable {
        store_scan(conn, &stable, now)?;
        Ok(output)
    } else {
        Ok(format!("{output} unstable=1"))
    }
}

fn stable_rebuild(conn: &Connection, workspace: &Path) -> Result<(String, Option<String>), String> {
    let mut output = String::new();
    for _ in 0..3 {
        let before = crate::workspace_search::inventory::source_manifest(workspace)?;
        output = crate::workspace_search::inventory::rebuild(conn, workspace)?;
        let after = crate::workspace_search::inventory::source_manifest(workspace)?;
        if before == after {
            return Ok((output, Some(after)));
        }
    }
    Ok((output, None))
}

fn value(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row("SELECT value FROM config WHERE key = ?1", [key], |row| {
        row.get(0)
    })
    .optional()
    .map_err(|error| error.to_string())
}

fn put(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO config (key, value) VALUES (?1, ?2)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn store_scan(conn: &Connection, manifest: &str, now: u64) -> Result<(), String> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    put(&tx, MANIFEST_KEY, manifest)?;
    put(&tx, CHECKED_KEY, &now.to_string())?;
    put(&tx, RECONCILED_KEY, &now.to_string())?;
    tx.commit().map_err(|error| error.to_string())
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or(0)
}
