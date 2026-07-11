use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationRow {
    pub id: String,
    pub idempotency_key: String,
    pub kind: String,
    pub phase: String,
    pub preimage_json: String,
    pub intended_json: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationRevision {
    pub role: String,
    pub path: String,
    pub bytes: Vec<u8>,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationPreparation {
    Prepared(OperationRow),
    Existing(OperationRow),
}

pub struct OperationDraft<'a> {
    pub id: &'a str,
    pub key: &'a str,
    pub kind: &'a str,
    pub preimage: &'a str,
    pub intended: &'a str,
    pub revisions: &'a [OperationRevision],
    pub now: &'a str,
}

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS workspace_records (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            state TEXT NOT NULL,
            path TEXT NOT NULL,
            fingerprint TEXT NOT NULL,
            archived INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS workspace_record_history (
            id INTEGER PRIMARY KEY,
            record_id TEXT NOT NULL,
            path TEXT NOT NULL,
            fingerprint TEXT NOT NULL,
            state TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

pub fn prepare_or_load_operation(
    conn: &mut Connection,
    draft: &OperationDraft<'_>,
) -> StoreResult<OperationPreparation> {
    let tx = conn.transaction()?;
    let inserted = tx.execute(
        "INSERT OR IGNORE INTO workspace_operations
         (id, idempotency_key, kind, phase, preimage_json, intended_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, 'prepared', ?4, ?5, ?6, ?6)",
        params![
            draft.id,
            draft.key,
            draft.kind,
            draft.preimage,
            draft.intended,
            draft.now
        ],
    )?;
    if inserted == 0 {
        let existing = operation_by_key(&tx, draft.key)?;
        tx.commit()?;
        return Ok(OperationPreparation::Existing(existing));
    }
    for revision in draft.revisions {
        tx.execute(
            "INSERT INTO workspace_operation_revisions
             (operation_id, role, path, bytes, fingerprint) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                draft.id,
                revision.role,
                revision.path,
                revision.bytes,
                revision.fingerprint
            ],
        )?;
    }
    let row = OperationRow {
        id: draft.id.to_string(),
        idempotency_key: draft.key.to_string(),
        kind: draft.kind.to_string(),
        phase: "prepared".to_string(),
        preimage_json: draft.preimage.to_string(),
        intended_json: draft.intended.to_string(),
        error: None,
    };
    tx.commit()?;
    Ok(OperationPreparation::Prepared(row))
}

pub fn prepared_operations(conn: &Connection) -> StoreResult<Vec<OperationRow>> {
    let mut statement = conn.prepare(
        "SELECT id, idempotency_key, kind, phase, preimage_json, intended_json, error
         FROM workspace_operations WHERE phase = 'prepared' ORDER BY created_at, id",
    )?;
    let rows = statement.query_map([], operation_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn operation_revisions(conn: &Connection, id: &str) -> StoreResult<Vec<OperationRevision>> {
    let mut statement = conn.prepare(
        "SELECT role, path, bytes, fingerprint FROM workspace_operation_revisions
         WHERE operation_id = ?1
         ORDER BY CASE role WHEN 'prior' THEN 0 WHEN 'intended' THEN 1 ELSE 2 END",
    )?;
    let rows = statement.query_map([id], |row| {
        Ok(OperationRevision {
            role: row.get(0)?,
            path: row.get(1)?,
            bytes: row.get(2)?,
            fingerprint: row.get(3)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn settle_operation(conn: &Connection, id: &str, now: &str) -> StoreResult<()> {
    set_operation_phase(conn, id, "settled", None, now)
}

pub fn compensate_operation(
    conn: &Connection,
    id: &str,
    error: &str,
    now: &str,
) -> StoreResult<()> {
    set_operation_phase(conn, id, "compensated", Some(error), now)
}

fn operation_by_key(conn: &rusqlite::Transaction<'_>, key: &str) -> StoreResult<OperationRow> {
    Ok(conn.query_row(
        "SELECT id, idempotency_key, kind, phase, preimage_json, intended_json, error
         FROM workspace_operations WHERE idempotency_key = ?1",
        [key],
        operation_row,
    )?)
}

fn operation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<OperationRow> {
    Ok(OperationRow {
        id: row.get(0)?,
        idempotency_key: row.get(1)?,
        kind: row.get(2)?,
        phase: row.get(3)?,
        preimage_json: row.get(4)?,
        intended_json: row.get(5)?,
        error: row.get(6)?,
    })
}

fn set_operation_phase(
    conn: &Connection,
    id: &str,
    phase: &str,
    error: Option<&str>,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE workspace_operations SET phase = ?2, error = ?3, updated_at = ?4 WHERE id = ?1",
        params![id, phase, error, now],
    )?;
    Ok(())
}
