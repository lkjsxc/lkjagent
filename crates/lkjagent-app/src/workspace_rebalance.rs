use lkjagent_core::workspace_manifest::{
    validate_rebalance_move, RebalanceMove, WorkspaceManifest,
};
use lkjagent_core::workspace_record::{record_fingerprint, record_path_at};
use lkjagent_store::record_rows::{records, RecordRow};
use lkjagent_store::workspace_rows::{upsert_manifest, OperationRevision};
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
};
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
    let manifests = crate::config::workspace_root(data_dir)?.join("system/manifests");
    fs::create_dir_all(&manifests).map_err(|error| error.to_string())?;
    let text = serde_json::to_string_pretty(&manifest).map_err(|error| error.to_string())?;
    fs::write(manifests.join("workspace-manifest.json"), text)
        .map_err(|error| error.to_string())?;
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
    let text = fs::read_to_string(workspace.join(rel)).map_err(|error| error.to_string())?;
    record_fingerprint(&text).map_err(|error| error.message)
}

pub(crate) fn verified_file_bytes(
    workspace: &Path,
    rel: &str,
    expected: &str,
) -> Result<Vec<u8>, String> {
    let bytes = fs::read(workspace.join(rel)).map_err(|error| error.to_string())?;
    let text = String::from_utf8(bytes.clone()).map_err(|error| error.to_string())?;
    if record_fingerprint(&text).map_err(|error| error.message)? == expected {
        Ok(bytes)
    } else {
        Err(format!("fingerprint mismatch: {rel}"))
    }
}

#[rustfmt::skip]
pub(crate) fn operation_revisions(item: &RebalanceMove, bytes: &[u8]) -> Result<Vec<OperationRevision>, String> {
    let fingerprint = lkjagent_core::runtime_fingerprint::stable_fingerprint(&bytes).map_err(|error| error.message)?;
    let row = |role: &str, path: &str| OperationRevision { role: role.to_string(), path: path.to_string(), bytes: bytes.to_vec(), fingerprint: fingerprint.clone() };
    Ok(vec![row("prior", &item.old_path), row("intended", &item.new_path)])
}

#[rustfmt::skip]
pub(crate) fn operation_preimage(row: &RecordRow) -> String {
    serde_json::json!({"id": row.id, "kind": row.kind, "title": row.title, "state": row.state, "path": row.path, "fingerprint": row.fingerprint, "archived": row.archived}).to_string()
}

pub(crate) fn operation_intended(item: &RebalanceMove) -> String {
    serde_json::json!({"id": item.entity_id, "path": item.new_path, "move": item}).to_string()
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

const INDEXES: &[&str] = &[
    "today",
    "agenda",
    "open-todos",
    "budget-month",
    "active-projects",
    "proof-runs",
    "experiments",
];

pub(crate) struct IndexSnapshot(Vec<(PathBuf, Option<Vec<u8>>)>);

impl IndexSnapshot {
    pub(crate) fn capture(workspace: &Path) -> Result<Self, String> {
        let mut files = Vec::new();
        for name in INDEXES {
            let path = workspace.join("indexes").join(format!("{name}.md"));
            let bytes = match fs::read(&path) {
                Ok(bytes) => Some(bytes),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => return Err(error.to_string()),
            };
            files.push((path, bytes));
        }
        Ok(Self(files))
    }

    pub(crate) fn restore(&self) -> Result<(), String> {
        for (path, bytes) in &self.0 {
            match bytes {
                Some(bytes) => fs::write(path, bytes).map_err(|error| error.to_string())?,
                None if path.exists() => {
                    fs::remove_file(path).map_err(|error| error.to_string())?
                }
                None => {}
            }
        }
        Ok(())
    }
}
