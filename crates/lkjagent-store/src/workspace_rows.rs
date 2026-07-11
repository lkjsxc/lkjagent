use lkjagent_core::workspace_manifest::{RebalanceMove, WorkspaceManifest};
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};

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

pub fn insert_alias(conn: &Connection, row: &PathAliasRow) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO workspace_path_aliases
         (old_path, entity_id, entity_kind, new_path, decision_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            row.old_path,
            row.entity_id,
            row.entity_kind,
            row.new_path,
            row.decision_id,
            row.created_at,
        ],
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

pub fn insert_rebalance_audit(
    conn: &Connection,
    id: &str,
    item: &RebalanceMove,
    created_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO workspace_rebalance_audit
         (id, entity_id, entity_kind, old_path, new_path, decision_id,
          validation_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            id,
            item.entity_id,
            item.entity_kind,
            item.old_path,
            item.new_path,
            item.decision_id,
            json(&item.validation)?,
            created_at,
        ],
    )?;
    Ok(())
}

pub fn setup_operations(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS workspace_operations (
            id TEXT PRIMARY KEY,
            idempotency_key TEXT NOT NULL UNIQUE,
            kind TEXT NOT NULL,
            phase TEXT NOT NULL,
            preimage_json TEXT NOT NULL,
            intended_json TEXT NOT NULL,
            error TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
pub fn prepare_operation(
    conn: &Connection,
    id: &str,
    idempotency_key: &str,
    kind: &str,
    preimage_json: &str,
    intended_json: &str,
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO workspace_operations
         (id, idempotency_key, kind, phase, preimage_json, intended_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, 'prepared', ?4, ?5, ?6, ?6)",
        params![id, idempotency_key, kind, preimage_json, intended_json, now],
    )?;
    Ok(())
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
