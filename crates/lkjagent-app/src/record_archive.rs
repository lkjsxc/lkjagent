use std::fs;
use std::path::Path;

use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::{archive_path, parse_record, record_fingerprint};
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    compensate_operation, insert_alias_and_audit, prepare_operation, remove_alias_and_audit,
    settle_operation, PathAliasRow,
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
    let audit_id = format!("archive-{}", row.id);
    let operation_id = format!("workspace-{audit_id}");
    prepare_operation(
        conn,
        &operation_id,
        &format!("archive:{}:{}", row.id, row.fingerprint),
        "archive",
        &crate::record_files::archive_preimage(&row),
        &crate::record_files::archive_intended(&row, &new_rel),
        now,
    )
    .map_err(|error| error.to_string())?;
    if let Some(parent) = new.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    if let Err(error) = fs::rename(&old, &new) {
        compensate_operation(conn, &operation_id, &error.to_string(), now)
            .map_err(|error| error.to_string())?;
        return Err(error.to_string());
    }
    let text = match fs::read_to_string(&new) {
        Ok(text) => text,
        Err(error) => {
            return rollback(
                conn,
                Rollback::new(data_dir, &old, &new, &row, &audit_id, &operation_id, now),
                error.to_string(),
            )
        }
    };
    let archived = match archived_row(&row, &new_rel, &text, now) {
        Ok(archived) => archived,
        Err(error) => {
            return rollback(
                conn,
                Rollback::new(data_dir, &old, &new, &row, &audit_id, &operation_id, now),
                error,
            )
        }
    };
    let settled = settle(
        conn,
        data_dir,
        &row,
        &archived,
        &audit_id,
        &operation_id,
        now,
    );
    if let Err(error) = settled {
        return rollback(
            conn,
            Rollback {
                data_dir,
                old: &old,
                new: &new,
                original: &row,
                audit_id: &audit_id,
                operation_id: &operation_id,
                now,
            },
            error,
        );
    }
    Ok(format!("archived record: {id}"))
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
    upsert_record(conn, archived).map_err(|error| error.to_string())?;
    archive_rows(conn, original, &archived.path, audit_id, now)?;
    crate::workspace_index::rebuild(conn, data_dir, now)?;
    crate::record_state::suppress_record_cells(conn, original, now)?;
    settle_operation(conn, operation_id, now).map_err(|error| error.to_string())
}

fn rollback(conn: &Connection, rollback: Rollback<'_>, error: String) -> Result<String, String> {
    remove_alias_and_audit(conn, &rollback.original.path, rollback.audit_id)
        .map_err(|error| error.to_string())?;
    upsert_record(conn, rollback.original).map_err(|error| error.to_string())?;
    fs::rename(rollback.new, rollback.old).map_err(|error| error.to_string())?;
    let text = fs::read_to_string(rollback.old).map_err(|error| error.to_string())?;
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

impl<'a> Rollback<'a> {
    fn new(
        data_dir: &'a Path,
        old: &'a Path,
        new: &'a Path,
        original: &'a RecordRow,
        audit_id: &'a str,
        operation_id: &'a str,
        now: &'a str,
    ) -> Self {
        Self {
            data_dir,
            old,
            new,
            original,
            audit_id,
            operation_id,
            now,
        }
    }
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
