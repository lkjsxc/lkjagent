use std::{fs, path::Path};

use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::{archive_path, parse_record, record_fingerprint};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    compensate_operation, insert_alias_and_audit, operation_for_key, operation_revisions,
    prepare_or_load_operation, remove_alias_and_audit, settle_operation, OperationDraft,
    PathAliasRow,
};
use rusqlite::Connection;

#[rustfmt::skip]
pub fn archive(conn: &Connection, data_dir: &Path, id: &str, now: &str) -> Result<String, String> {
    let row = record(conn, id).map_err(|error| error.to_string())?.ok_or_else(|| format!("record not found: {id}"))?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let old = workspace.join(&row.path);
    let new_rel = archive_path(&row.kind, &row.id)?;
    let new = workspace.join(&new_rel);
    let audit_id = format!("archive-{}", row.id);
    let operation_id = format!("workspace-{audit_id}");
    let key = format!("archive:{}:{}", row.id, row.fingerprint);
    if let Some(operation) = operation_for_key(conn, &key).map_err(|error| error.to_string())? {
        if operation.phase == "settled" {
            let revisions = operation_revisions(conn, &operation.id).map_err(|error| error.to_string())?;
            let intended = revisions.iter().find(|revision| revision.role == "intended")
                .ok_or_else(|| "archive intended revision missing".to_string())?;
            let prior = revisions.iter().find(|revision| revision.role == "prior")
                .ok_or_else(|| "archive prior revision missing".to_string())?;
            if workspace.join(&prior.path) != new && workspace.join(&prior.path).exists() { return Err("settled archive prior path reoccupied".to_string()); }
            if fs::read(&new).map_err(|error| error.to_string())? != intended.bytes { return Err("settled archive target changed".to_string()); }
            return Ok(format!("archived record: {id}"));
        }
        if new.exists() { return resume_moved(conn, data_dir, &row, (&new_rel, &new), (&audit_id, &operation.id, &operation.preimage_json), now); }
    }
    let bytes = crate::record_files::archive_source_bytes(&old, &row.fingerprint)?;
    prepare_or_load_operation(conn, &OperationDraft { id: &operation_id, key: &key, kind: "archive", preimage: &crate::record_files::archive_preimage(&row), intended: &crate::record_files::archive_intended(&row, &new_rel), revisions: &crate::record_files::archive_revisions(&row.path, &new_rel, &bytes)?, now }).map_err(|error| error.to_string())?;
    if let Some(parent) = new.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
    if let Err(error) = crate::record_files::move_if_absent(&old, &new) {
        compensate_operation(conn, &operation_id, &error.to_string(), now).map_err(|error| error.to_string())?;
        return Err(error.to_string());
    }
    let restore = |error| rollback(conn, Rollback { data_dir, old: &old, new: &new, original: &row, audit_id: &audit_id, operation_id: &operation_id, now }, error);
    let text = match fs::read_to_string(&new) { Ok(text) => text, Err(error) => return restore(error.to_string()) };
    let archived = match archived_row(&row, &new_rel, &text, now) { Ok(archived) => archived, Err(error) => return restore(error) };
    if let Err(error) = settle(conn, data_dir, &row, &archived, &audit_id, &operation_id, now) { return restore(error); }
    Ok(format!("archived record: {id}"))
}

#[rustfmt::skip]
fn resume_moved(conn: &Connection, data_dir: &Path, original: &RecordRow, target: (&str, &Path), operation: (&str, &str, &str), now: &str) -> Result<String, String> {
    let (path, target) = target;
    let (audit_id, operation_id, preimage) = operation;
    let original = prepared_original(original, preimage)?;
    let revisions = operation_revisions(conn, operation_id).map_err(|error| error.to_string())?;
    let intended = revisions.iter().find(|revision| revision.role == "intended").ok_or_else(|| "archive intended revision missing".to_string())?;
    let prior = revisions.iter().find(|revision| revision.role == "prior").ok_or_else(|| "archive prior revision missing".to_string())?;
    let bytes = fs::read(target).map_err(|error| error.to_string())?;
    if bytes != intended.bytes { return Err("archive target conflicts with intended revision".to_string()); }
    let source = crate::config::workspace_root(data_dir)?.join(&prior.path);
    if source != target && source.exists() { return Err("archive prior path remains occupied".to_string()); }
    let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
    let archived = archived_row(&original, path, &text, now)?;
    settle(conn, data_dir, &original, &archived, audit_id, operation_id, now)?;
    Ok(format!("archived record: {}", original.id))
}

fn prepared_original(row: &RecordRow, preimage: &str) -> Result<RecordRow, String> {
    let value: serde_json::Value =
        serde_json::from_str(preimage).map_err(|error| error.to_string())?;
    let field = |name| {
        value
            .get(name)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("archive preimage missing {name}"))
    };
    let id = field("id")?;
    if id != row.id {
        return Err("archive preimage record mismatch".to_string());
    }
    Ok(RecordRow {
        id: id.to_string(),
        kind: row.kind.clone(),
        title: row.title.clone(),
        state: field("state")?.to_string(),
        path: field("path")?.to_string(),
        fingerprint: field("fingerprint")?.to_string(),
        archived: value
            .get("archived")
            .and_then(serde_json::Value::as_bool)
            .ok_or_else(|| "archive preimage missing archived".to_string())?,
        updated_at: row.updated_at.clone(),
    })
}

fn settle(
    conn: &Connection,
    data_dir: &Path,
    original: &RecordRow,
    archived: &RecordRow,
    audit_id: &str,
    operation_id: &str,
    now: &str,
) -> Result<(), String> {
    let current = fs::read_to_string(crate::config::workspace_root(data_dir)?.join(&archived.path))
        .map_err(|error| error.to_string())?;
    if record_fingerprint(&current).map_err(|error| error.message)? != archived.fingerprint {
        return Err("archive target changed before settlement".to_string());
    }
    upsert_record(conn, archived).map_err(|error| error.to_string())?;
    archive_rows(conn, original, &archived.path, audit_id, now)?;
    crate::workspace_index::rebuild(conn, data_dir, now)?;
    crate::record_state::suppress_record_cells(conn, original, now)?;
    settle_operation(conn, operation_id, now).map_err(|error| error.to_string())
}

fn rollback(conn: &Connection, rollback: Rollback<'_>, error: String) -> Result<String, String> {
    let text = fs::read_to_string(rollback.new).map_err(|error| error.to_string())?;
    if record_fingerprint(&text).map_err(|error| error.message)? != rollback.original.fingerprint {
        return Err("archive rollback target changed".to_string());
    }
    crate::record_files::move_if_absent(rollback.new, rollback.old)?;
    remove_alias_and_audit(conn, &rollback.original.path, rollback.audit_id)
        .map_err(|error| error.to_string())?;
    upsert_record(conn, rollback.original).map_err(|error| error.to_string())?;
    let record = parse_record(&text)?;
    crate::record_state::upsert_record_cells(
        conn,
        &record,
        &rollback.original.path,
        &rollback.original.fingerprint,
    )?;
    crate::workspace_index::rebuild(conn, rollback.data_dir, rollback.now)?;
    compensate_operation(conn, rollback.operation_id, &error, rollback.now)
        .map_err(|error| error.to_string())?;
    Err(error)
}

struct Rollback<'a> {
    data_dir: &'a Path,
    old: &'a Path,
    new: &'a Path,
    original: &'a RecordRow,
    audit_id: &'a str,
    operation_id: &'a str,
    now: &'a str,
}

#[rustfmt::skip]
fn archived_row(original: &RecordRow, path: &str, text: &str, now: &str) -> Result<RecordRow, String> {
    let fingerprint = record_fingerprint(text).map_err(|error| error.message)?;
    if fingerprint != original.fingerprint { return Err("archive target fingerprint changed".to_string()); }
    Ok(RecordRow { id: original.id.clone(), kind: original.kind.clone(), title: original.title.clone(), state: "archived".to_string(), path: path.to_string(), fingerprint, archived: true, updated_at: now.to_string() })
}

#[rustfmt::skip]
fn archive_rows(conn: &Connection, row: &RecordRow, path: &str, audit_id: &str, now: &str) -> Result<(), String> {
    let item = RebalanceMove { entity_id: row.id.clone(), entity_kind: "record".to_string(), old_path: row.path.clone(), new_path: path.to_string(), decision_id: "record.archive".to_string(), reason: "record archived".to_string(), validation: vec!["archive:true".to_string()] };
    let alias = PathAliasRow { old_path: row.path.clone(), entity_id: row.id.clone(), entity_kind: "record".to_string(), new_path: path.to_string(), decision_id: "record.archive".to_string(), created_at: now.to_string() };
    insert_alias_and_audit(conn, &alias, audit_id, &item, now).map_err(|error| error.to_string())
}
