use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::{validate_rebalance_move, RebalanceMove};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    insert_alias_and_audit, operation_revisions, settle_operation, OperationRow, PathAliasRow,
};
use rusqlite::Connection;

pub fn run(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    crate::workspace_rebalance_group::run(conn, data_dir, json, now)
}

#[rustfmt::skip]
pub fn recover_prepared(conn: &Connection, data_dir: &Path, operation: &OperationRow, now: &str) -> Result<(), String> {
    recover_prepared_mode(conn, data_dir, operation, now, false)
}

#[rustfmt::skip]
pub(crate) fn recover_for_apply(conn: &Connection, data_dir: &Path, operation: &OperationRow, now: &str) -> Result<(), String> {
    recover_prepared_mode(conn, data_dir, operation, now, true)
}

#[rustfmt::skip]
fn recover_prepared_mode(conn: &Connection, data_dir: &Path, operation: &OperationRow,
    now: &str, allow_unstarted: bool) -> Result<(), String> {
    let mut item = intended_move(&operation.intended_json)?
        .ok_or_else(|| "rebalance intended move missing".to_string())?;
    if !validate_rebalance_move(&item).is_empty() { return Err("persisted rebalance move is invalid".to_string()); }
    let workspace = crate::config::workspace_root(data_dir)?;
    let current = record(conn, &item.entity_id).map_err(|error| error.to_string())?.ok_or_else(|| format!("record not found: {}", item.entity_id))?;
    let original = preimage_row(&operation.preimage_json)?;
    let moved = crate::effect_files::path_occupied(&workspace, &item.new_path)?;
    validate_recovery_row(&current, &original, &item, moved)?;
    if !moved {
        if !crate::effect_files::path_occupied(&workspace, &item.old_path)? { return Err("rebalance target and prior paths missing".to_string()); }
        settle_unstarted(conn, &workspace, &item, operation, &original.fingerprint, allow_unstarted)?;
        move_file(&workspace, &item)?;
    }
    crate::record_files::sync_relative_move(&workspace, &item.old_path, &item.new_path)?;
    verify_moved(conn, &workspace, &item, &operation.id, &original.fingerprint)?;
    settle_one(conn, &workspace, &mut item, &original, &operation.id, now)?;
    crate::workspace_index::rebuild(conn, data_dir, now)?;
    settle_operation(conn, &operation.id, now).map_err(|error| error.to_string())
}

#[rustfmt::skip]
fn settle_unstarted(conn: &Connection, workspace: &Path, item: &RebalanceMove,
    operation: &OperationRow, expected: &str, allow_unstarted: bool) -> Result<(), String> {
    let revisions = operation_revisions(conn, &operation.id).map_err(|error| error.to_string())?;
    let (prior, _) = crate::workspace_rebalance::validated_revisions(&revisions, item, expected)?;
    if crate::effect_files::read_bytes(workspace, &prior.path)? != prior.bytes {
        return Err("rebalance prior source changed".to_string());
    }
    if allow_unstarted { Ok(()) } else { Err("rebalance unstarted operation requires explicit apply".to_string()) }
}

#[rustfmt::skip]
fn settle_one(conn: &Connection, workspace: &Path, item: &mut RebalanceMove, original: &RecordRow, operation_id: &str, now: &str) -> Result<(), String> {
    verify_moved(conn, workspace, item, operation_id, &original.fingerprint)?;
    crate::workspace_root::refresh_for_path(workspace, &item.old_path)?;
    crate::workspace_root::refresh_for_path(workspace, &item.new_path)?;
    verify_moved(conn, workspace, item, operation_id, &original.fingerprint)?;
    let current = record(conn, &item.entity_id).map_err(|error| error.to_string())?.ok_or_else(|| format!("record not found: {}", item.entity_id))?;
    if current.path != item.new_path { let mut updated = original.clone(); updated.path = item.new_path.clone(); upsert_record(conn, &updated).map_err(|error| error.to_string())?; }
    let repaired = crate::workspace_root::repair_record_links(conn, workspace, &item.entity_id, &item.old_path, &item.new_path, now)?;
    item.validation.push(format!("links-repaired:{repaired}"));
    insert_alias_and_audit(conn, &alias(item, now), &audit_id(item), item, now).map_err(|error| error.to_string())?;
    Ok(())
}

#[rustfmt::skip]
fn verify_moved(conn: &Connection, workspace: &Path, item: &RebalanceMove, operation_id: &str, expected: &str) -> Result<(), String> {
    let revisions = operation_revisions(conn, operation_id).map_err(|error| error.to_string())?;
    let (prior, intended) = crate::workspace_rebalance::validated_revisions(&revisions, item, expected)?;
    if crate::effect_files::apply_revision(workspace, &prior.path, &None, &None).is_err() { return Err("rebalance prior path remains occupied".to_string()); }
    if crate::effect_files::read_bytes(workspace, &intended.path).map_err(|_| "rebalance target conflicts with intended revision".to_string())? != intended.bytes { return Err("rebalance target conflicts with intended revision".to_string()); }
    Ok(())
}

#[rustfmt::skip]
fn move_file(workspace: &Path, item: &RebalanceMove) -> Result<(), String> {
    crate::record_files::move_relative_if_absent(workspace, &item.old_path, &item.new_path)
}

#[rustfmt::skip]
fn intended_move(json: &str) -> Result<Option<RebalanceMove>, String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|error| error.to_string())?;
    let Some(item) = value.get("move").cloned().map(serde_json::from_value::<RebalanceMove>).transpose().map_err(|error| error.to_string())? else { return Ok(None); };
    let id = value.get("id").and_then(serde_json::Value::as_str); let path = value.get("path").and_then(serde_json::Value::as_str);
    if id != Some(item.entity_id.as_str()) || path != Some(item.new_path.as_str()) { return Err("rebalance intended envelope conflicts with move".to_string()); }
    Ok(Some(item))
}

#[rustfmt::skip]
fn preimage_row(json: &str) -> Result<RecordRow, String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|error| error.to_string())?;
    let field = |name| value.get(name).and_then(serde_json::Value::as_str).ok_or_else(|| format!("rebalance preimage missing {name}"));
    Ok(RecordRow { id: field("id")?.to_string(), kind: field("kind")?.to_string(),
        title: field("title")?.to_string(), state: field("state")?.to_string(), path: field("path")?.to_string(),
        fingerprint: field("fingerprint")?.to_string(), archived: value.get("archived").and_then(serde_json::Value::as_bool).ok_or_else(|| "rebalance preimage missing archived".to_string())?, updated_at: field("updated_at")?.to_string() })
}

#[rustfmt::skip]
fn validate_recovery_row(current: &RecordRow, original: &RecordRow,
    item: &RebalanceMove, moved: bool) -> Result<(), String> {
    let same = original.id == item.entity_id && original.path == item.old_path
        && current.id == original.id && current.kind == original.kind && current.title == original.title
        && current.state == original.state && current.fingerprint == original.fingerprint
        && current.archived == original.archived && current.updated_at == original.updated_at;
    let path = if moved { current.path == original.path || current.path == item.new_path } else { current.path == original.path };
    if same && path { Ok(()) } else { Err("rebalance record preimage changed".to_string()) }
}

#[rustfmt::skip]
fn alias(item: &RebalanceMove, now: &str) -> PathAliasRow {
    PathAliasRow { old_path: item.old_path.clone(), entity_id: item.entity_id.clone(), entity_kind: item.entity_kind.clone(), new_path: item.new_path.clone(), decision_id: item.decision_id.clone(), created_at: now.to_string() }
}

#[rustfmt::skip]
fn audit_id(item: &RebalanceMove) -> String {
    let identity = format!("{}\0{}\0{}", item.entity_id, item.old_path, item.new_path);
    stable_fingerprint(&identity).map(|value| format!("rebalance-{value}")).unwrap_or_else(|_| format!("rebalance-{}", item.entity_id))
}
