use std::collections::BTreeSet;

use lkjagent_store::artifact_cursor::{mark_paths_completed, mark_paths_failed};
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};

pub fn validate_paths_against_contract(conn: &Connection, paths: &[String]) -> ToolResult<()> {
    validate_active_atom_contracts(conn, paths)?;
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
    update_active_contracts(conn, paths, now, "written", "satisfied")?;
    Ok(())
}

pub fn record_failed_paths(conn: &Connection, paths: &[String], now: &str) -> ToolResult<()> {
    mark_paths_failed(conn, paths, now)?;
    update_active_contracts(conn, paths, now, "blocked", "failed")?;
    Ok(())
}

fn validate_active_atom_contracts(conn: &Connection, paths: &[String]) -> ToolResult<()> {
    let contracts = lkjagent_store::artifact_graph::active_contracts(conn)?;
    if contracts.is_empty() {
        return Ok(());
    }
    let admitted = contracts
        .iter()
        .flat_map(|contract| split_owned(&contract.exact_paths))
        .collect::<BTreeSet<_>>();
    for path in paths {
        if !admitted.contains(path) {
            return Err(ToolError::invalid(format!(
                "fs.batch_write path outside active artifact contract: {path}"
            )));
        }
    }
    Ok(())
}

fn update_active_contracts(
    conn: &Connection,
    paths: &[String],
    now: &str,
    atom_status: &str,
    contract_status: &str,
) -> ToolResult<()> {
    let touched = paths.iter().cloned().collect::<BTreeSet<_>>();
    for contract in lkjagent_store::artifact_graph::active_contracts(conn)? {
        let exact = split_owned(&contract.exact_paths);
        if !exact.iter().any(|path| touched.contains(path)) {
            continue;
        }
        let weak = split_owned(&contract.forbidden_weak_classes);
        for atom_id in split_owned(&contract.atom_ids) {
            lkjagent_store::artifact_graph::update_atom_status(
                conn,
                &atom_id,
                atom_status,
                0,
                &weak,
                now,
            )?;
        }
        lkjagent_store::artifact_graph::set_contract_status(
            conn,
            &contract.contract_id,
            contract_status,
            now,
        )?;
    }
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

fn split_owned(values: &str) -> Vec<String> {
    values
        .lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}
