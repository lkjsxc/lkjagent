use std::path::Path;

use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::{archive_path, parse_record, record_fingerprint};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    compensate_operation, insert_alias_and_audit, operation_for_key, operation_revisions,
    prepare_or_load_operation, remove_alias_and_audit, settle_operation, transition_operation,
    OperationDraft, OperationRevision, PathAliasRow,
};
use rusqlite::Connection;

#[rustfmt::skip]
pub fn archive(conn: &Connection, data_dir: &Path, id: &str, now: &str) -> Result<String, String> {
    let mut row = record(conn, id).map_err(|error| error.to_string())?.ok_or_else(|| format!("record not found: {id}"))?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let new_rel = archive_path(&row.kind, &row.id)?;
    let audit_id = format!("archive-{}", row.id); let operation_id = format!("workspace-{audit_id}");
    let key = format!("archive:{}:{}", row.id, row.fingerprint);
    if let Some(operation) = operation_for_key(conn, &key).map_err(|error| error.to_string())? {
        if operation.kind != "archive" || operation.id != operation_id { return Err("archive operation identity mismatch".to_string()); }
        validate_intended(&operation.intended_json, &row.id, &new_rel)?;
        let original = prepared_original(&row, &operation.preimage_json, &new_rel)?;
        let revisions = operation_revisions(conn, &operation.id).map_err(|error| error.to_string())?;
        let (prior, intended) = validated_revisions(&revisions, &original, &new_rel)?;
        if operation.phase == "settled" {
            if prior.path != new_rel && crate::effect_files::path_occupied(&workspace, &prior.path)? { return Err("settled archive prior path reoccupied".to_string()); }
            if crate::effect_files::read_bytes(&workspace, &new_rel)? != intended.bytes { return Err("settled archive target changed".to_string()); }
            return Ok(format!("archived record: {id}"));
        }
        let target_exists = crate::effect_files::path_occupied(&workspace, &new_rel)?;
        if target_exists {
            if crate::effect_files::read_bytes(&workspace, &new_rel)? != intended.bytes { return Err("archive target conflicts with intended revision".to_string()); }
            if crate::effect_files::path_occupied(&workspace, &prior.path)? { return Err("archive prior path remains occupied".to_string()); }
        } else if crate::workspace_rebalance::verified_file_bytes(&workspace, &original.path, &original.fingerprint)? != prior.bytes { return Err("archive source conflicts with prepared revision".to_string()); }
        if operation.phase == "compensated" {
            transition_operation(conn, &operation.id, "compensated", "prepared", now).map_err(|error| error.to_string())?;
            transition_operation(conn, &operation.id, "prepared", "moving", now).map_err(|error| error.to_string())?;
        } else if operation.phase == "prepared" { transition_operation(conn, &operation.id, "prepared", "moving", now).map_err(|error| error.to_string())?; }
        else if operation.phase != "moving" { return Err("archive operation phase is invalid".to_string()); }
        if target_exists { return resume_moved(conn, data_dir, &row, &new_rel, (&audit_id, &operation.id, &operation.preimage_json), now); }
        if row != original { return Err("archive source-only row changed".to_string()); }
        row = original;
    }
    let bytes = crate::workspace_rebalance::verified_file_bytes(&workspace, &row.path, &row.fingerprint)?;
    prepare_or_load_operation(conn, &OperationDraft { id: &operation_id, key: &key, kind: "archive", preimage: &crate::record_files::archive_preimage(&row), intended: &crate::record_files::archive_intended(&row, &new_rel), revisions: &crate::record_files::archive_revisions(&row.path, &new_rel, &bytes)?, now }).map_err(|error| error.to_string())?;
    let operation = operation_for_key(conn, &key).map_err(|error| error.to_string())?.ok_or_else(|| "archive operation missing after preparation".to_string())?;
    if operation.kind != "archive" || operation.id != operation_id { return Err("archive operation identity mismatch".to_string()); }
    validate_intended(&operation.intended_json, &row.id, &new_rel)?;
    if prepared_original(&row, &operation.preimage_json, &new_rel)? != row { return Err("archive source-only row changed".to_string()); }
    let revisions = operation_revisions(conn, &operation_id).map_err(|error| error.to_string())?;
    let (prior, _) = validated_revisions(&revisions, &row, &new_rel)?;
    if prior.bytes != bytes { return Err("archive source conflicts with prepared revision".to_string()); }
    if operation.phase == "compensated" { transition_operation(conn, &operation.id, "compensated", "prepared", now).map_err(|error| error.to_string())?; transition_operation(conn, &operation.id, "prepared", "moving", now).map_err(|error| error.to_string())?; }
    else if operation.phase == "prepared" { transition_operation(conn, &operation.id, "prepared", "moving", now).map_err(|error| error.to_string())?; }
    else if operation.phase != "moving" { return Err("archive operation phase is invalid".to_string()); }
    if let Err(error) = crate::record_files::move_relative_if_absent(&workspace, &row.path, &new_rel) {
        if !crate::effect_files::path_occupied(&workspace, &new_rel)? { compensate_operation(conn, &operation_id, &error, now).map_err(|error| error.to_string())?; }
        return Err(error);
    }
    let restore = |error| rollback(conn, Rollback { data_dir, old: &row.path, new: &new_rel, original: &row, audit_id: &audit_id, operation_id: &operation_id, now }, error);
    let text = match crate::effect_files::read_text(&workspace, &new_rel) { Ok(text) => text, Err(error) => return restore(error) };
    let archived = match archived_row(&row, &new_rel, &text, now) { Ok(archived) => archived, Err(error) => return restore(error) };
    if let Err(error) = settle(conn, data_dir, &row, &archived, &audit_id, &operation_id, now) { return restore(error); }
    Ok(format!("archived record: {id}"))
}

#[rustfmt::skip]
fn resume_moved(conn: &Connection, data_dir: &Path, current: &RecordRow, path: &str, operation: (&str, &str, &str), now: &str) -> Result<String, String> {
    let (audit_id, operation_id, preimage) = operation;
    let original = prepared_original(current, preimage, path)?;
    let revisions = operation_revisions(conn, operation_id).map_err(|error| error.to_string())?;
    let (prior, intended) = validated_revisions(&revisions, &original, path)?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let bytes = crate::effect_files::read_bytes(&workspace, path)?;
    if bytes != intended.bytes { return Err("archive target conflicts with intended revision".to_string()); }
    if prior.path != path && crate::effect_files::path_occupied(&workspace, &prior.path)? { return Err("archive prior path remains occupied".to_string()); }
    crate::record_files::sync_relative_move(&workspace, &prior.path, path)?;
    let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
    let archived = archived_row(&original, path, &text, now)?;
    settle(conn, data_dir, &original, &archived, audit_id, operation_id, now)?;
    Ok(format!("archived record: {}", original.id))
}

#[rustfmt::skip]
fn prepared_original(row: &RecordRow, preimage: &str, target: &str) -> Result<RecordRow, String> {
    let value: serde_json::Value = serde_json::from_str(preimage).map_err(|error| error.to_string())?;
    let field = |name| value.get(name).and_then(serde_json::Value::as_str).ok_or_else(|| format!("archive preimage missing {name}"));
    let original = RecordRow { id: field("id")?.to_string(), kind: field("kind")?.to_string(),
        title: field("title")?.to_string(), state: field("state")?.to_string(),
        path: field("path")?.to_string(), fingerprint: field("fingerprint")?.to_string(),
        archived: value.get("archived").and_then(serde_json::Value::as_bool).ok_or_else(|| "archive preimage missing archived".to_string())?,
        updated_at: field("updated_at")?.to_string() };
    let projected = row.id == original.id && row.kind == original.kind && row.title == original.title
        && row.fingerprint == original.fingerprint && row.archived && row.state == "archived" && row.path == target;
    if row == &original || projected { Ok(original) } else { Err("archive record preimage changed".to_string()) }
}

fn validate_intended(json: &str, id: &str, target: &str) -> Result<(), String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|error| error.to_string())?;
    let matches = value.get("id").and_then(serde_json::Value::as_str) == Some(id)
        && value.get("path").and_then(serde_json::Value::as_str) == Some(target)
        && value.get("state").and_then(serde_json::Value::as_str) == Some("archived");
    if matches {
        Ok(())
    } else {
        Err("archive intended descriptor conflicts".to_string())
    }
}

#[rustfmt::skip]
fn validated_revisions<'a>(rows: &'a [OperationRevision], original: &RecordRow,
    target: &str) -> Result<(&'a OperationRevision, &'a OperationRevision), String> {
    let prior = rows.iter().find(|row| row.role == "prior").ok_or_else(|| "archive prior revision missing".to_string())?;
    let intended = rows.iter().find(|row| row.role == "intended").ok_or_else(|| "archive intended revision missing".to_string())?;
    if rows.len() != 2 || prior.path != original.path || intended.path != target { return Err("archive revision membership conflicts".to_string()); }
    for row in rows { let fingerprint = lkjagent_core::runtime_fingerprint::stable_fingerprint(&row.bytes).map_err(|error| error.message)?; if fingerprint != row.fingerprint { return Err("archive revision fingerprint changed".to_string()); } }
    if prior.bytes != intended.bytes || prior.fingerprint != intended.fingerprint { return Err("archive revisions contain different bytes".to_string()); }
    let text = std::str::from_utf8(&prior.bytes).map_err(|error| error.to_string())?;
    if record_fingerprint(text).map_err(|error| error.message)? != original.fingerprint { return Err("archive revision conflicts with preimage".to_string()); }
    Ok((prior, intended))
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
    let workspace = crate::config::workspace_root(data_dir)?;
    let current = crate::effect_files::read_text(&workspace, &archived.path)?;
    if record_fingerprint(&current).map_err(|error| error.message)? != archived.fingerprint {
        return Err("archive target changed before settlement".to_string());
    }
    let row = record(conn, &archived.id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {}", archived.id))?;
    if row.path != archived.path || row.state != archived.state || !row.archived {
        upsert_record(conn, archived).map_err(|error| error.to_string())?;
    }
    archive_rows(conn, original, &archived.path, audit_id, now)?;
    crate::workspace_index::rebuild(conn, data_dir, now)?;
    crate::record_state::suppress_record_cells(conn, original, now)?;
    settle_operation(conn, operation_id, now).map_err(|error| error.to_string())
}

fn rollback(conn: &Connection, rollback: Rollback<'_>, error: String) -> Result<String, String> {
    let workspace = crate::config::workspace_root(rollback.data_dir)?;
    let text = crate::effect_files::read_text(&workspace, rollback.new)?;
    if record_fingerprint(&text).map_err(|error| error.message)? != rollback.original.fingerprint {
        return Err("archive rollback target changed".to_string());
    }
    crate::record_files::move_relative_if_absent(&workspace, rollback.new, rollback.old)?;
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
    old: &'a str,
    new: &'a str,
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
