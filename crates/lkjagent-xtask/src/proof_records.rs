use std::fs;
use std::path::Path;

use lkjagent_core::runtime_selector::candidates;
use lkjagent_store::record_rows::records;
use lkjagent_store::state_rows::hydrate_snapshot;
use rusqlite::Connection;

pub fn write_record_selector_bundle(conn: &Connection, out_dir: &Path) -> Result<(), String> {
    write(out_dir, "records.md", &record_rows(conn)?)?;
    write(out_dir, "selector-candidates.md", &selector_rows(conn)?)
}

fn record_rows(conn: &Connection) -> Result<String, String> {
    let rows = records(conn, None, true).map_err(|error| error.to_string())?;
    let mut lines = vec!["# Workspace Records".to_string(), String::new()];
    for row in rows {
        lines.push(format!(
            "- {} kind={} state={} archived={} path={} fp={}",
            row.id, row.kind, row.state, row.archived, row.path, row.fingerprint
        ));
    }
    if lines.len() == 2 {
        lines.push("none".to_string());
    }
    Ok(lines.join("\n"))
}

fn selector_rows(conn: &Connection) -> Result<String, String> {
    let mut lines = vec!["# Selector Candidates".to_string(), String::new()];
    for case_id in case_ids(conn)? {
        let snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
        for candidate in candidates(&snapshot) {
            let state = candidate
                .state_key
                .as_ref()
                .map_or_else(|| "none".to_string(), |key| key.as_label());
            lines.push(format!(
                "- case={} op={} state={} reason={} blocked_by={}",
                case_id,
                candidate.operation.key,
                state,
                candidate.reason,
                candidate.blocked_by.join(",")
            ));
        }
    }
    if lines.len() == 2 {
        lines.push("none".to_string());
    }
    Ok(lines.join("\n"))
}

fn case_ids(conn: &Connection) -> Result<Vec<String>, String> {
    let mut statement = conn
        .prepare("SELECT id FROM cases ORDER BY id")
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?;
    let mut output = Vec::new();
    for row in rows {
        output.push(row.map_err(|error| error.to_string())?);
    }
    Ok(output)
}

fn write(dir: &Path, name: &str, body: &str) -> Result<(), String> {
    fs::write(dir.join(name), body).map_err(|error| error.to_string())
}
