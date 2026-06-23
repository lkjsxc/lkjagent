use std::collections::BTreeSet;

use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchCursorInput<'a> {
    pub artifact_ledger_id: i64,
    pub root: &'a str,
    pub planned_paths: &'a [String],
    pub completed_paths: &'a [String],
    pub failed_paths: &'a [String],
    pub current_index: i64,
    pub last_valid_example: &'a str,
    pub retry_counts: &'a str,
    pub fallback_mode: &'a str,
    pub updated_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchCursorRow {
    pub artifact_ledger_id: i64,
    pub root: String,
    pub planned_paths: String,
    pub completed_paths: String,
    pub failed_paths: String,
    pub current_index: i64,
    pub last_valid_example: String,
    pub retry_counts: String,
    pub fallback_mode: String,
}

pub fn upsert_batch_cursor(conn: &Connection, input: &BatchCursorInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "DELETE FROM artifact_batch_cursors WHERE artifact_ledger_id = ?1 AND root = ?2",
        params![input.artifact_ledger_id, input.root],
    )?;
    conn.execute(
        "INSERT INTO artifact_batch_cursors
         (artifact_ledger_id, root, planned_paths, completed_paths, failed_paths,
          current_index, last_valid_example, retry_counts, fallback_mode, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            input.artifact_ledger_id,
            input.root,
            join(input.planned_paths),
            join(input.completed_paths),
            join(input.failed_paths),
            input.current_index,
            input.last_valid_example,
            input.retry_counts,
            input.fallback_mode,
            input.updated_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn latest_batch_cursor(
    conn: &Connection,
    artifact_ledger_id: i64,
) -> StoreResult<Option<BatchCursorRow>> {
    let mut statement = conn.prepare(
        "SELECT artifact_ledger_id, root, planned_paths, completed_paths, failed_paths,
         current_index, last_valid_example, retry_counts, fallback_mode
         FROM artifact_batch_cursors WHERE artifact_ledger_id = ?1
         ORDER BY updated_at DESC, id DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![artifact_ledger_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(BatchCursorRow {
        artifact_ledger_id: row.get(0)?,
        root: row.get(1)?,
        planned_paths: row.get(2)?,
        completed_paths: row.get(3)?,
        failed_paths: row.get(4)?,
        current_index: row.get(5)?,
        last_valid_example: row.get(6)?,
        retry_counts: row.get(7)?,
        fallback_mode: row.get(8)?,
    }))
}

pub fn delete_batch_cursor(
    conn: &Connection,
    artifact_ledger_id: i64,
    root: &str,
) -> StoreResult<()> {
    conn.execute(
        "DELETE FROM artifact_batch_cursors WHERE artifact_ledger_id = ?1 AND root = ?2",
        params![artifact_ledger_id, root],
    )?;
    Ok(())
}

pub fn mark_paths_completed(conn: &Connection, paths: &[String], now: &str) -> StoreResult<usize> {
    let touched = paths.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let mut changed = 0usize;
    for row in mutable_cursors(conn)? {
        let planned = split(&row.planned_paths);
        let completed = split(&row.completed_paths);
        let additions = planned
            .iter()
            .filter(|path| touched.contains(path.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        if additions.is_empty() {
            continue;
        }
        let merged = merge(completed, additions);
        conn.execute(
            "UPDATE artifact_batch_cursors SET completed_paths = ?1, updated_at = ?2 WHERE id = ?3",
            params![join(&merged), now, row.id],
        )?;
        changed = changed.saturating_add(1);
    }
    Ok(changed)
}

struct MutableCursor {
    id: i64,
    planned_paths: String,
    completed_paths: String,
}

fn mutable_cursors(conn: &Connection) -> StoreResult<Vec<MutableCursor>> {
    let mut statement = conn.prepare(
        "SELECT id, planned_paths, completed_paths FROM artifact_batch_cursors ORDER BY id",
    )?;
    let rows = statement.query_map([], |row| {
        Ok(MutableCursor {
            id: row.get(0)?,
            planned_paths: row.get(1)?,
            completed_paths: row.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn merge(existing: Vec<String>, additions: Vec<String>) -> Vec<String> {
    let mut seen = existing.into_iter().collect::<BTreeSet<_>>();
    seen.extend(additions);
    seen.into_iter().collect()
}

fn split(values: &str) -> Vec<String> {
    values
        .lines()
        .map(str::to_string)
        .filter(|path| !path.is_empty())
        .collect()
}

fn join(values: &[String]) -> String {
    values.join("\n")
}
