use lkjagent_core::workspace_manifest::{
    validate_rebalance_move, RebalanceMove, WorkspaceManifest,
};
use lkjagent_core::workspace_record::{record_fingerprint, record_path_at};
use lkjagent_store::record_rows::{records, RecordRow};
use lkjagent_store::workspace_rows::{upsert_manifest, OperationRevision};
use rusqlite::Connection;
use std::path::Path;
pub fn plan(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    ensure_manifest(conn, data_dir, now)?;
    let moves = planned_moves(conn)?;
    render_plan(&moves, json)
}
pub fn apply(conn: &Connection, data_dir: &Path, json: bool, now: &str) -> Result<String, String> {
    crate::workspace_rebalance_apply::run(conn, data_dir, json, now)
}
pub fn validate(
    conn: &Connection,
    data_dir: &Path,
    json: bool,
    now: &str,
) -> Result<String, String> {
    ensure_manifest(conn, data_dir, now)?;
    let rows = records(conn, None, true).map_err(|error| error.to_string())?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let mut missing = Vec::new();
    let mut stale = Vec::new();
    for row in &rows {
        if !workspace.join(&row.path).exists() {
            missing.push(row.id.clone());
            continue;
        }
        match file_fingerprint(&workspace, &row.path) {
            Ok(found) if found == row.fingerprint => {}
            _ => stale.push(row.id.clone()),
        }
    }
    if json {
        let valid = missing.is_empty() && stale.is_empty();
        return Ok(
            serde_json::json!({"valid": valid, "missing": missing, "stale": stale}).to_string(),
        );
    }
    match (missing.is_empty(), stale.is_empty()) {
        (true, true) => Ok("workspace validate: ok".to_string()),
        (false, true) => Ok(format!("workspace validate: missing {}", missing.join(","))),
        (true, false) => Ok(format!("workspace validate: stale {}", stale.join(","))),
        (false, false) => Ok(format!(
            "workspace validate: missing {} stale {}",
            missing.join(","),
            stale.join(",")
        )),
    }
}
pub(crate) fn ensure_manifest(conn: &Connection, data_dir: &Path, now: &str) -> Result<(), String> {
    let manifest = WorkspaceManifest::default_workspace();
    let workspace = crate::config::workspace_root(data_dir)?;
    let text = serde_json::to_string_pretty(&manifest).map_err(|error| error.to_string())?;
    crate::effect_files::write_bytes(
        &workspace,
        "system/manifests/workspace-manifest.json",
        text.as_bytes(),
    )?;
    upsert_manifest(conn, &manifest, now).map_err(|error| error.to_string())
}
pub(crate) fn planned_moves(conn: &Connection) -> Result<Vec<RebalanceMove>, String> {
    let rows = records(conn, None, false).map_err(|error| error.to_string())?;
    Ok(rows
        .into_iter()
        .filter_map(|row| move_for_row(&row))
        .collect())
}
fn move_for_row(row: &RecordRow) -> Option<RebalanceMove> {
    let new_path = record_path_at(&row.kind, &row.id, &row.updated_at, &row.title, &row.state)
        .unwrap_or_else(|_| format!("records/knowledge/notes/{}/{}.md", row.kind, row.id));
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
    let validation = validate_rebalance_move(&item);
    item.validation = if validation.is_empty() {
        vec!["ok".to_string()]
    } else {
        validation
    };
    Some(item)
}
pub(crate) fn file_fingerprint(workspace: &Path, rel: &str) -> Result<String, String> {
    let text = crate::effect_files::read_text(workspace, rel)?;
    record_fingerprint(&text).map_err(|error| error.message)
}

pub(crate) fn verified_file_bytes(
    workspace: &Path,
    rel: &str,
    expected: &str,
) -> Result<Vec<u8>, String> {
    let bytes = crate::effect_files::read_bytes(workspace, rel)?;
    let text = String::from_utf8(bytes.clone()).map_err(|error| error.to_string())?;
    if record_fingerprint(&text).map_err(|error| error.message)? == expected {
        Ok(bytes)
    } else {
        Err(format!("fingerprint mismatch: {rel}"))
    }
}

#[rustfmt::skip]
pub(crate) fn validated_revisions<'a>(rows: &'a [OperationRevision], item: &RebalanceMove, expected: &str) -> Result<(&'a OperationRevision, &'a OperationRevision), String> {
    let prior = rows.iter().find(|row| row.role == "prior").ok_or_else(|| "rebalance prior revision missing".to_string())?;
    let intended = rows.iter().find(|row| row.role == "intended").ok_or_else(|| "rebalance intended revision missing".to_string())?; if rows.len() != 2 { return Err("rebalance move must have exactly two revisions".to_string()); }
    if prior.path != item.old_path || intended.path != item.new_path { return Err("rebalance revision paths conflict".to_string()); }
    for row in rows {
        let fingerprint = lkjagent_core::runtime_fingerprint::stable_fingerprint(&row.bytes).map_err(|error| error.message)?;
        if fingerprint != row.fingerprint { return Err("rebalance revision fingerprint changed".to_string()); }
    }
    if prior.bytes != intended.bytes || prior.fingerprint != intended.fingerprint { return Err("rebalance move revisions contain different bytes".to_string()); }
    let text = std::str::from_utf8(&prior.bytes).map_err(|error| error.to_string())?; if record_fingerprint(text).map_err(|error| error.message)? != expected { return Err("rebalance revision content conflicts with record preimage".to_string()); }
    Ok((prior, intended))
}

pub(crate) fn render_plan(moves: &[RebalanceMove], json: bool) -> Result<String, String> {
    if json {
        return serde_json::to_string(moves).map_err(|error| error.to_string());
    }
    if moves.is_empty() {
        return Ok("rebalance plan: no moves".to_string());
    }
    Ok(moves.iter().map(move_line).collect::<Vec<_>>().join("\n"))
}
fn move_line(item: &RebalanceMove) -> String {
    format!(
        "move {} {} -> {}",
        item.entity_id, item.old_path, item.new_path
    )
}
