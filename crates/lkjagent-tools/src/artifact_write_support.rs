use std::collections::BTreeSet;

use lkjagent_store::artifact_cursor::{mark_paths_completed, mark_paths_failed};
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};

pub fn validate_paths_against_contract(conn: &Connection, paths: &[String]) -> ToolResult<()> {
    for row in cursor_rows(conn)? {
        let planned = split(&row.planned_paths)
            .into_iter()
            .collect::<BTreeSet<_>>();
        let completed = split(&row.completed_paths)
            .into_iter()
            .collect::<BTreeSet<_>>();
        let scoped = paths
            .iter()
            .filter(|path| in_root(path, &row.root))
            .collect::<Vec<_>>();
        if scoped.is_empty() {
            continue;
        }
        for path in scoped {
            if !planned.contains(path.as_str()) {
                return Err(ToolError::invalid(format!(
                    "fs.batch_write path outside stored write contract: {path}"
                )));
            }
            if completed.contains(path.as_str()) {
                return Err(ToolError::invalid(format!(
                    "fs.batch_write path already completed by stored contract: {path}"
                )));
            }
        }
    }
    Ok(())
}

pub fn record_written_paths(conn: &Connection, paths: &[String], now: &str) -> ToolResult<()> {
    mark_paths_completed(conn, paths, now)?;
    Ok(())
}

pub fn record_failed_paths(conn: &Connection, paths: &[String], now: &str) -> ToolResult<()> {
    mark_paths_failed(conn, paths, now)?;
    Ok(())
}

struct CursorRow {
    root: String,
    planned_paths: String,
    completed_paths: String,
}

fn cursor_rows(conn: &Connection) -> ToolResult<Vec<CursorRow>> {
    let mut statement = conn
        .prepare(
            "SELECT root, planned_paths, completed_paths FROM artifact_batch_cursors ORDER BY id",
        )
        .map_err(|error| ToolError::Store(error.to_string()))?;
    let rows = statement
        .query_map([], |row| {
            Ok(CursorRow {
                root: row.get(0)?,
                planned_paths: row.get(1)?,
                completed_paths: row.get(2)?,
            })
        })
        .map_err(|error| ToolError::Store(error.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|error| ToolError::Store(error.to_string()))?);
    }
    Ok(out)
}

fn in_root(path: &str, root: &str) -> bool {
    let root = root.trim_end_matches('/');
    path == root || path.starts_with(&format!("{root}/"))
}

fn split(values: &str) -> Vec<&str> {
    values.lines().filter(|value| !value.is_empty()).collect()
}
