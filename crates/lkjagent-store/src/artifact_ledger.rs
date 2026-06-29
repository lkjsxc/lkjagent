use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLedgerInput<'a> {
    pub case_id: i64,
    pub artifact_id: &'a str,
    pub root: &'a str,
    pub kind: &'a str,
    pub normalized_topic: &'a str,
    pub requested_scale: &'a str,
    pub profile: &'a str,
    pub lifecycle_state: &'a str,
    pub topology_status: &'a str,
    pub readiness_status: &'a str,
    pub objective_match_status: &'a str,
    pub latest_audit_id: Option<&'a str>,
    pub weak_path_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLedgerRow {
    pub id: i64,
    pub case_id: i64,
    pub artifact_id: String,
    pub root: String,
    pub kind: String,
    pub lifecycle_state: String,
    pub topology_status: String,
    pub readiness_status: String,
    pub weak_path_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeakPathInput<'a> {
    pub artifact_ledger_id: i64,
    pub path: &'a str,
    pub role: &'a str,
    pub missing_requirements: &'a [String],
    pub weak_signals: &'a [String],
    pub semantic_mismatch: &'a str,
    pub retry_count: i64,
    pub updated_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeakPathRow {
    pub path: String,
    pub role: String,
    pub missing_requirements: String,
    pub weak_signals: String,
    pub semantic_mismatch: String,
    pub retry_count: i64,
}

pub fn upsert_artifact(
    conn: &Connection,
    input: &ArtifactLedgerInput<'_>,
    now: &str,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO artifact_ledger
         (case_id, artifact_id, root, kind, normalized_topic, requested_scale,
          profile, lifecycle_state, topology_status, readiness_status,
          objective_match_status, latest_audit_id, weak_path_count, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?14)
         ON CONFLICT(artifact_id) DO UPDATE SET
          root = excluded.root,
          lifecycle_state = excluded.lifecycle_state,
          topology_status = excluded.topology_status,
          readiness_status = excluded.readiness_status,
          objective_match_status = excluded.objective_match_status,
          latest_audit_id = excluded.latest_audit_id,
          weak_path_count = excluded.weak_path_count,
          updated_at = excluded.updated_at",
        params![
            input.case_id,
            input.artifact_id,
            input.root,
            input.kind,
            input.normalized_topic,
            input.requested_scale,
            input.profile,
            input.lifecycle_state,
            input.topology_status,
            input.readiness_status,
            input.objective_match_status,
            input.latest_audit_id,
            input.weak_path_count,
            now,
        ],
    )?;
    artifact_id(conn, input.artifact_id)
}

pub fn record_weak_path(conn: &Connection, input: &WeakPathInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO artifact_weak_paths
         (artifact_ledger_id, path, role, missing_requirements, weak_signals,
          semantic_mismatch, retry_count, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.artifact_ledger_id,
            input.path,
            input.role,
            join(input.missing_requirements),
            join(input.weak_signals),
            input.semantic_mismatch,
            input.retry_count,
            input.updated_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn latest_for_case(conn: &Connection, case_id: i64) -> StoreResult<Option<ArtifactLedgerRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, artifact_id, root, kind, lifecycle_state,
         topology_status, readiness_status, weak_path_count FROM artifact_ledger
         WHERE case_id = ?1 ORDER BY updated_at DESC, id DESC LIMIT 1",
    )?;
    let mut rows = statement.query(params![case_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(ArtifactLedgerRow {
        id: row.get(0)?,
        case_id: row.get(1)?,
        artifact_id: row.get(2)?,
        root: row.get(3)?,
        kind: row.get(4)?,
        lifecycle_state: row.get(5)?,
        topology_status: row.get(6)?,
        readiness_status: row.get(7)?,
        weak_path_count: row.get(8)?,
    }))
}

pub fn weak_paths(conn: &Connection, artifact_ledger_id: i64) -> StoreResult<Vec<WeakPathRow>> {
    let mut statement = conn.prepare(
        "SELECT path, role, missing_requirements, weak_signals,
         semantic_mismatch, retry_count FROM artifact_weak_paths
         WHERE artifact_ledger_id = ?1 ORDER BY id",
    )?;
    let rows = statement.query_map(params![artifact_ledger_id], |row| {
        Ok(WeakPathRow {
            path: row.get(0)?,
            role: row.get(1)?,
            missing_requirements: row.get(2)?,
            weak_signals: row.get(3)?,
            semantic_mismatch: row.get(4)?,
            retry_count: row.get(5)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn artifact_id(conn: &Connection, semantic_id: &str) -> StoreResult<i64> {
    Ok(conn.query_row(
        "SELECT id FROM artifact_ledger WHERE artifact_id = ?1",
        params![semantic_id],
        |row| row.get(0),
    )?)
}

fn join(values: &[String]) -> String {
    values.join(",")
}
