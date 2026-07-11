use lkjagent_core::workspace_manifest::{RebalanceMove, WorkspaceManifest};
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};
pub use crate::record_schema::{
    compensate_operation, operation_for_key, operation_revisions, prepare_or_load_operation,
    prepared_operations, settle_operation, OperationDraft, OperationPreparation, OperationRevision,
    OperationRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathAliasRow {
    pub old_path: String,
    pub entity_id: String,
    pub entity_kind: String,
    pub new_path: String,
    pub decision_id: String,
    pub created_at: String,
}

pub fn upsert_manifest(
    conn: &Connection,
    manifest: &WorkspaceManifest,
    updated_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO workspace_manifest
         (id, schema_version, root_policy_json, manifest_json, updated_at)
         VALUES ('default', ?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET schema_version=excluded.schema_version,
         root_policy_json=excluded.root_policy_json,
         manifest_json=excluded.manifest_json, updated_at=excluded.updated_at",
        params![
            manifest.schema_version,
            json(&manifest.root_policy)?,
            json(manifest)?,
            updated_at,
        ],
    )?;
    Ok(())
}

#[rustfmt::skip]
pub fn insert_alias(conn: &Connection, row: &PathAliasRow) -> StoreResult<()> {
    let changed = conn.execute(
        "INSERT INTO workspace_path_aliases
         (old_path, entity_id, entity_kind, new_path, decision_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(old_path) DO UPDATE SET old_path=excluded.old_path
         WHERE entity_id=excluded.entity_id AND entity_kind=excluded.entity_kind
         AND new_path=excluded.new_path AND decision_id=excluded.decision_id",
        params![row.old_path, row.entity_id, row.entity_kind, row.new_path, row.decision_id, row.created_at])?;
    if changed == 1 { Ok(()) } else { Err(StoreError::InvalidState("path alias conflicts".to_string())) }
}

pub fn setup_operations(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS workspace_operations (
            id TEXT PRIMARY KEY, idempotency_key TEXT NOT NULL UNIQUE, kind TEXT NOT NULL,
            phase TEXT NOT NULL, preimage_json TEXT NOT NULL, intended_json TEXT NOT NULL,
            error TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS workspace_operation_revisions (
            operation_id TEXT NOT NULL, role TEXT NOT NULL, path TEXT NOT NULL,
            bytes BLOB NOT NULL, fingerprint TEXT NOT NULL,
            PRIMARY KEY(operation_id, role), FOREIGN KEY(operation_id) REFERENCES workspace_operations(id)
        );
        ",
    )?;
    Ok(())
}

#[rustfmt::skip]
pub fn active_rebalance_groups(conn: &Connection) -> StoreResult<Vec<OperationRow>> {
    let mut statement = conn.prepare(
        "SELECT id, idempotency_key, kind, phase, preimage_json, intended_json, error
         FROM workspace_operations WHERE kind = 'rebalance-group'
         AND phase NOT IN ('settled', 'compensated') ORDER BY created_at, id")?;
    let rows = statement.query_map([], |row| Ok(OperationRow { id: row.get(0)?,
        idempotency_key: row.get(1)?, kind: row.get(2)?, phase: row.get(3)?,
        preimage_json: row.get(4)?, intended_json: row.get(5)?, error: row.get(6)? }))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[rustfmt::skip]
pub fn transition_operation(conn: &Connection, id: &str, from: &str, to: &str,
    now: &str) -> StoreResult<()> {
    let changed = conn.execute("UPDATE workspace_operations SET phase = ?3, error = NULL,
        updated_at = ?4 WHERE id = ?1 AND phase = ?2", params![id, from, to, now])?;
    if changed == 1 { Ok(()) } else { Err(StoreError::InvalidState(format!("operation phase changed: {id}"))) }
}

pub fn update_operation_error(
    conn: &Connection,
    id: &str,
    error: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "UPDATE workspace_operations SET error = ?2, updated_at = ?3 WHERE id = ?1",
        params![id, error, now],
    )?;
    Ok(())
}

pub fn remove_alias_and_audit(
    conn: &Connection,
    old_path: &str,
    audit_id: &str,
) -> StoreResult<()> {
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "DELETE FROM workspace_path_aliases WHERE old_path = ?1",
        [old_path],
    )?;
    tx.execute(
        "DELETE FROM workspace_rebalance_audit WHERE id = ?1",
        [audit_id],
    )?;
    tx.commit()?;
    Ok(())
}

pub fn insert_alias_and_audit(
    conn: &Connection,
    alias: &PathAliasRow,
    audit_id: &str,
    item: &RebalanceMove,
    created_at: &str,
) -> StoreResult<()> {
    let tx = conn.unchecked_transaction()?;
    insert_alias(&tx, alias)?;
    insert_rebalance_audit(&tx, audit_id, item, created_at)?;
    tx.commit()?;
    Ok(())
}

pub fn resolve_alias(conn: &Connection, old_path: &str) -> StoreResult<Option<PathAliasRow>> {
    let row = conn.query_row(
        "SELECT old_path, entity_id, entity_kind, new_path, decision_id, created_at
         FROM workspace_path_aliases WHERE old_path = ?1",
        [old_path],
        alias_row,
    );
    match row {
        Ok(row) => Ok(Some(row)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[rustfmt::skip]
pub fn insert_rebalance_audit(conn: &Connection, id: &str, item: &RebalanceMove,
    created_at: &str) -> StoreResult<()> {
    let changed = conn.execute(
        "INSERT INTO workspace_rebalance_audit
         (id, entity_id, entity_kind, old_path, new_path, decision_id,
          validation_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET id=excluded.id
         WHERE entity_id=excluded.entity_id AND entity_kind=excluded.entity_kind
         AND old_path=excluded.old_path AND new_path=excluded.new_path
         AND decision_id=excluded.decision_id",
        params![id, item.entity_id, item.entity_kind, item.old_path, item.new_path,
            item.decision_id, json(&item.validation)?, created_at])?;
    if changed == 1 { Ok(()) } else { Err(StoreError::InvalidState("rebalance audit conflicts".to_string())) }
}

fn json<T: serde::Serialize>(value: &T) -> StoreResult<String> {
    serde_json::to_string(value).map_err(|error| StoreError::InvalidState(error.to_string()))
}

fn alias_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PathAliasRow> {
    Ok(PathAliasRow {
        old_path: row.get(0)?,
        entity_id: row.get(1)?,
        entity_kind: row.get(2)?,
        new_path: row.get(3)?,
        decision_id: row.get(4)?,
        created_at: row.get(5)?,
    })
}
