use std::fs;
use std::path::Path;

use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::{archive_path, record_fingerprint};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    insert_alias_and_audit, remove_alias_and_audit, PathAliasRow,
};
use rusqlite::Connection;

pub fn archive(conn: &Connection, data_dir: &Path, id: &str, now: &str) -> Result<String, String> {
    let row = record(conn, id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {id}"))?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let old = workspace.join(&row.path);
    let new_rel = archive_path(&row.kind, &row.id)?;
    let new = workspace.join(&new_rel);
    if let Some(parent) = new.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::rename(&old, &new).map_err(|error| error.to_string())?;
    let text = fs::read_to_string(&new).map_err(|error| error.to_string())?;
    let archived = archived_row(&row, &new_rel, &text, now)?;
    let audit_id = format!("archive-{}", row.id);
    let settled = settle(conn, data_dir, &row, &archived, &audit_id, now);
    if let Err(error) = settled {
        return rollback(conn, &old, &new, &row, &audit_id, error);
    }
    Ok(format!("archived record: {id}"))
}

fn settle(
    conn: &Connection,
    data_dir: &Path,
    original: &RecordRow,
    archived: &RecordRow,
    audit_id: &str,
    now: &str,
) -> Result<(), String> {
    upsert_record(conn, archived).map_err(|error| error.to_string())?;
    archive_rows(conn, original, &archived.path, audit_id, now)?;
    crate::workspace_index::rebuild(conn, data_dir, now)?;
    crate::record_state::suppress_record_cells(conn, original, now)
}

fn rollback(
    conn: &Connection,
    old: &Path,
    new: &Path,
    original: &RecordRow,
    audit_id: &str,
    error: String,
) -> Result<String, String> {
    remove_alias_and_audit(conn, &original.path, audit_id).map_err(|error| error.to_string())?;
    upsert_record(conn, original).map_err(|error| error.to_string())?;
    fs::rename(new, old).map_err(|error| error.to_string())?;
    Err(error)
}

fn archived_row(
    original: &RecordRow,
    path: &str,
    text: &str,
    now: &str,
) -> Result<RecordRow, String> {
    Ok(RecordRow {
        id: original.id.clone(),
        kind: original.kind.clone(),
        title: original.title.clone(),
        state: "archived".to_string(),
        path: path.to_string(),
        fingerprint: record_fingerprint(text).map_err(|error| error.message)?,
        archived: true,
        updated_at: now.to_string(),
    })
}

fn archive_rows(
    conn: &Connection,
    row: &RecordRow,
    path: &str,
    audit_id: &str,
    now: &str,
) -> Result<(), String> {
    let item = RebalanceMove {
        entity_id: row.id.clone(),
        entity_kind: "record".to_string(),
        old_path: row.path.clone(),
        new_path: path.to_string(),
        decision_id: "record.archive".to_string(),
        reason: "record archived".to_string(),
        validation: vec!["archive:true".to_string()],
    };
    let alias = PathAliasRow {
        old_path: row.path.clone(),
        entity_id: row.id.clone(),
        entity_kind: "record".to_string(),
        new_path: path.to_string(),
        decision_id: "record.archive".to_string(),
        created_at: now.to_string(),
    };
    insert_alias_and_audit(conn, &alias, audit_id, &item, now).map_err(|error| error.to_string())
}
