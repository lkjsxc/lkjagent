use std::fs;
use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::{
    canonical_record_path, validate_rebalance_move, RebalanceMove, WorkspaceManifest,
};
use lkjagent_store::record_rows::{records, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    insert_alias, insert_rebalance_audit, upsert_manifest, PathAliasRow,
};
use rusqlite::Connection;

pub fn plan(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    ensure_manifest(conn, data_dir, now)?;
    let moves = planned_moves(conn)?;
    render_plan(&moves, json)
}

pub fn apply(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    ensure_manifest(conn, data_dir, now)?;
    let moves = planned_moves(conn)?;
    let workspace = data_dir.join("workspace");
    for item in &moves {
        if !validate_rebalance_move(item).is_empty() {
            return Err(format!("invalid rebalance move: {}", item.entity_id));
        }
        move_record_file(&workspace, item)?;
        update_record(conn, item, now)?;
        insert_alias(conn, &alias(item, now)).map_err(|error| error.to_string())?;
        insert_rebalance_audit(conn, &audit_id(item), item, now)
            .map_err(|error| error.to_string())?;
    }
    render_plan(&moves, json)
}

pub fn validate(
    conn: &Connection,
    data_dir: &Path,
    json: bool,
    now: &str,
) -> Result<String, String> {
    ensure_manifest(conn, data_dir, now)?;
    let rows = records(conn, None, true).map_err(|error| error.to_string())?;
    let missing = rows
        .iter()
        .filter(|row| !data_dir.join("workspace").join(&row.path).exists())
        .map(|row| row.id.clone())
        .collect::<Vec<_>>();
    if json {
        return Ok(
            serde_json::json!({"valid": missing.is_empty(), "missing": missing}).to_string(),
        );
    }
    if missing.is_empty() {
        Ok("workspace validate: ok".to_string())
    } else {
        Ok(format!("workspace validate: missing {}", missing.join(",")))
    }
}

fn ensure_manifest(conn: &Connection, data_dir: &Path, now: &str) -> Result<(), String> {
    let manifest = WorkspaceManifest::default_workspace();
    let system = data_dir.join("workspace/system");
    fs::create_dir_all(&system).map_err(|error| error.to_string())?;
    let text = serde_json::to_string_pretty(&manifest).map_err(|error| error.to_string())?;
    fs::write(system.join("workspace-manifest.json"), text).map_err(|error| error.to_string())?;
    upsert_manifest(conn, &manifest, now).map_err(|error| error.to_string())
}

fn planned_moves(conn: &Connection) -> Result<Vec<RebalanceMove>, String> {
    let rows = records(conn, None, false).map_err(|error| error.to_string())?;
    Ok(rows
        .into_iter()
        .filter_map(|row| move_for_row(&row))
        .collect())
}

fn move_for_row(row: &RecordRow) -> Option<RebalanceMove> {
    let new_path = canonical_record_path(&row.kind, &row.id);
    if row.path == new_path {
        return None;
    }
    let mut item = RebalanceMove {
        entity_id: row.id.clone(),
        entity_kind: "record".to_string(),
        old_path: row.path.clone(),
        new_path,
        decision_id: "workspace.rebalance".to_string(),
        reason: "canonical record path".to_string(),
        validation: Vec::new(),
    };
    item.validation = if validate_rebalance_move(&item).is_empty() {
        vec!["ok".to_string()]
    } else {
        validate_rebalance_move(&item)
    };
    Some(item)
}

fn move_record_file(workspace: &Path, item: &RebalanceMove) -> Result<(), String> {
    let old = workspace.join(&item.old_path);
    let new = workspace.join(&item.new_path);
    if let Some(parent) = new.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(old, new).map_err(|error| error.to_string())
}

fn update_record(conn: &Connection, item: &RebalanceMove, now: &str) -> Result<(), String> {
    let mut row = lkjagent_store::record_rows::record(conn, &item.entity_id)
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

fn render_plan(moves: &[RebalanceMove], json: bool) -> Result<String, String> {
    if json {
        return serde_json::to_string(moves).map_err(|error| error.to_string());
    }
    if moves.is_empty() {
        return Ok("rebalance plan: no moves".to_string());
    }
    Ok(moves
        .iter()
        .map(|item| {
            format!(
                "move {} {} -> {}",
                item.entity_id, item.old_path, item.new_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n"))
}
