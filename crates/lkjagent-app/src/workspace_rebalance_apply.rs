use std::fs;
use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::{validate_rebalance_move, RebalanceMove};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    insert_alias_and_audit, remove_alias_and_audit, PathAliasRow,
};
use rusqlite::Connection;

use crate::workspace_rebalance::IndexSnapshot;

pub fn run(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    crate::workspace_rebalance::ensure_manifest(conn, data_dir, now)?;
    let moves = crate::workspace_rebalance::planned_moves(conn)?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let snapshot = IndexSnapshot::capture(&workspace)?;
    let mut applied = Vec::new();
    for item in &moves {
        match apply_one(conn, &workspace, item.clone(), now) {
            Ok(move_) => applied.push(move_),
            Err(error) => return restore(conn, &workspace, &applied, &snapshot, now, error),
        }
    }
    if !moves.is_empty() {
        if let Err(error) = crate::workspace_index::rebuild(conn, data_dir, now) {
            return restore(conn, &workspace, &applied, &snapshot, now, error);
        }
    }
    crate::workspace_rebalance::render_plan(&moves, json)
}

fn apply_one(
    conn: &Connection,
    workspace: &Path,
    mut item: RebalanceMove,
    now: &str,
) -> Result<Applied, String> {
    if !validate_rebalance_move(&item).is_empty() {
        return Err(format!("invalid rebalance move: {}", item.entity_id));
    }
    let original = record(conn, &item.entity_id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {}", item.entity_id))?;
    let fingerprint = crate::workspace_rebalance::file_fingerprint(workspace, &item.old_path)?;
    if fingerprint != original.fingerprint {
        return Err(format!("fingerprint mismatch: {}", item.entity_id));
    }
    item.validation
        .push(format!("fingerprint-before:{fingerprint}"));
    move_file(workspace, &item)?;
    match crate::workspace_rebalance::file_fingerprint(workspace, &item.new_path) {
        Ok(found) if found == fingerprint => {}
        Ok(_) => {
            restore_one(conn, workspace, &item, &original, now)?;
            return Err(format!("moved fingerprint mismatch: {}", item.entity_id));
        }
        Err(error) => {
            restore_one(conn, workspace, &item, &original, now)?;
            return Err(error);
        }
    }
    if let Err(error) = crate::workspace_scaffold::refresh_for_path(workspace, &item.new_path) {
        restore_one(conn, workspace, &item, &original, now)?;
        return Err(error);
    }
    if let Err(error) = update(conn, &item, now) {
        restore_one(conn, workspace, &item, &original, now)?;
        return Err(error);
    }
    let repaired = crate::workspace_scaffold::repair_record_links(
        conn,
        workspace,
        &item.entity_id,
        &item.old_path,
        &item.new_path,
        now,
    );
    item.validation.push(format!("links-repaired:{repaired}"));
    if let Err(error) =
        insert_alias_and_audit(conn, &alias(&item, now), &audit_id(&item), &item, now)
            .map_err(|error| error.to_string())
    {
        restore_one(conn, workspace, &item, &original, now)?;
        return Err(error);
    }
    Ok(Applied { item, original })
}

fn restore(
    conn: &Connection,
    workspace: &Path,
    applied: &[Applied],
    snapshot: &IndexSnapshot,
    now: &str,
    error: String,
) -> Result<String, String> {
    for move_ in applied.iter().rev() {
        restore_one(conn, workspace, &move_.item, &move_.original, now)?;
    }
    crate::workspace_search::rebuild(conn, workspace)?;
    snapshot.restore()?;
    Err(error)
}

fn restore_one(
    conn: &Connection,
    workspace: &Path,
    item: &RebalanceMove,
    original: &RecordRow,
    now: &str,
) -> Result<(), String> {
    let _ = crate::workspace_scaffold::repair_record_links(
        conn,
        workspace,
        &item.entity_id,
        &item.new_path,
        &item.old_path,
        now,
    );
    remove_alias_and_audit(conn, &item.old_path, &audit_id(item))
        .map_err(|value| value.to_string())?;
    let new = workspace.join(&item.new_path);
    if new.exists() {
        fs::rename(new, workspace.join(&item.old_path)).map_err(|value| value.to_string())?;
    }
    upsert_record(conn, original).map_err(|value| value.to_string())?;
    Ok(())
}

fn move_file(workspace: &Path, item: &RebalanceMove) -> Result<(), String> {
    let old = workspace.join(&item.old_path);
    let new = workspace.join(&item.new_path);
    if let Some(parent) = new.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(old, new).map_err(|error| error.to_string())
}

fn update(conn: &Connection, item: &RebalanceMove, now: &str) -> Result<(), String> {
    let mut row = record(conn, &item.entity_id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {}", item.entity_id))?;
    row.path = item.new_path.clone();
    row.updated_at = now.to_string();
    upsert_record(conn, &row).map_err(|error| error.to_string())
}

fn alias(item: &RebalanceMove, now: &str) -> PathAliasRow {
    PathAliasRow {
        old_path: item.old_path.clone(),
        entity_id: item.entity_id.clone(),
        entity_kind: item.entity_kind.clone(),
        new_path: item.new_path.clone(),
        decision_id: item.decision_id.clone(),
        created_at: now.to_string(),
    }
}

fn audit_id(item: &RebalanceMove) -> String {
    stable_fingerprint(item)
        .map(|value| format!("rebalance-{value}"))
        .unwrap_or_else(|_| format!("rebalance-{}", item.entity_id))
}

struct Applied {
    item: RebalanceMove,
    original: RecordRow,
}
