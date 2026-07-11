mod prepare;
mod recover;

use std::collections::BTreeSet;
use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::{validate_rebalance_move, RebalanceMove};
use lkjagent_store::workspace_rows::{
    active_rebalance_groups, prepared_operations, OperationRevision, OperationRow,
};
use rusqlite::Connection;

pub fn run(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    let groups = active_rebalance_groups(conn).map_err(|error| error.to_string())?;
    if groups.len() > 1 {
        return Err("multiple active rebalance groups".to_string());
    }
    crate::workspace_rebalance::ensure_manifest(conn, data_dir, now)?;
    if let Some(group) = groups.first() {
        let moves = prepare::intended_moves(group)?;
        recover::recover(conn, data_dir, group, now, true)?;
        return crate::workspace_rebalance::render_plan(&moves, json);
    }
    for operation in prepared_operations(conn).map_err(|error| error.to_string())? {
        if operation.kind == "rebalance" {
            crate::workspace_rebalance_apply::recover_for_apply(conn, data_dir, &operation, now)?;
        }
    }
    let moves = crate::workspace_rebalance::planned_moves(conn)?;
    if moves.is_empty() {
        return crate::workspace_rebalance::render_plan(&moves, json);
    }
    let group = prepare::prepare(conn, data_dir, moves.clone(), now)?;
    recover::recover(conn, data_dir, &group, now, true)?;
    crate::workspace_rebalance::render_plan(&moves, json)
}

fn group_fingerprint(
    preimage: &str,
    intended: &str,
    revisions: &[OperationRevision],
) -> Result<String, String> {
    let mut rows = revisions.to_vec();
    rows.sort_by(|left, right| left.role.cmp(&right.role));
    let rows = rows
        .iter()
        .map(|row| format!("{}:{}:{}", row.role, row.path, row.fingerprint))
        .collect::<Vec<_>>()
        .join("\0");
    stable_fingerprint(&format!("{preimage}\0{intended}\0{rows}")).map_err(|error| error.message)
}

fn validate_moves(moves: &[RebalanceMove]) -> Result<(), String> {
    if moves.is_empty() {
        return Err("rebalance group is empty".to_string());
    }
    let ids = moves
        .iter()
        .map(|item| item.entity_id.as_str())
        .collect::<BTreeSet<_>>();
    let old = moves
        .iter()
        .map(|item| item.old_path.as_str())
        .collect::<BTreeSet<_>>();
    let new = moves
        .iter()
        .map(|item| item.new_path.as_str())
        .collect::<BTreeSet<_>>();
    let bad = ids.len() != moves.len()
        || old.len() != moves.len()
        || new.len() != moves.len()
        || new.iter().any(|path| old.contains(path))
        || moves
            .iter()
            .any(|item| !validate_rebalance_move(item).is_empty());
    if bad {
        Err("rebalance group moves are invalid or overlap".to_string())
    } else {
        Ok(())
    }
}

pub fn recover_startup(
    conn: &Connection,
    data_dir: &Path,
    operation: &OperationRow,
    now: &str,
) -> Result<(), String> {
    recover::recover(conn, data_dir, operation, now, false)
}
