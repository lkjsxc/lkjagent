use lkjagent_core::model::{CheckResult, CheckSpec};
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRow {
    pub id: String,
    pub case_id: String,
    pub kind: String,
    pub path: String,
    pub fingerprint: String,
    pub parent_artifact_id: Option<String>,
    pub metadata_json: String,
    pub created_at: String,
}

pub fn insert_artifact(conn: &Connection, row: &ArtifactRow) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO artifacts
         (id, case_id, kind, path, fingerprint, parent_artifact_id,
          metadata_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            row.id,
            row.case_id,
            row.kind,
            row.path,
            row.fingerprint,
            row.parent_artifact_id,
            row.metadata_json,
            row.created_at,
        ],
    )?;
    if row.parent_artifact_id.is_none() {
        suppress_superseded_edges(conn, row)?;
    }
    Ok(())
}

fn suppress_superseded_edges(conn: &Connection, row: &ArtifactRow) -> StoreResult<()> {
    conn.execute(
        "UPDATE state_edges SET status = 'Suppressed', suppression_reason = ?1
         WHERE case_id = ?2 AND status = 'Active' AND to_ref_kind = 'artifact'
         AND to_ref_id IN (
             SELECT id FROM artifacts WHERE case_id = ?2 AND path = ?3
             AND parent_artifact_id IS NULL AND id != ?4
         )",
        params![
            "artifact superseded by newer fingerprint",
            row.case_id,
            row.path,
            row.id
        ],
    )?;
    Ok(())
}

pub fn artifacts(conn: &Connection, case_id: &str) -> StoreResult<Vec<ArtifactRow>> {
    let mut statement = conn.prepare(
        "SELECT id, case_id, kind, path, fingerprint, parent_artifact_id,
         metadata_json, created_at FROM artifacts WHERE case_id = ?1 ORDER BY id",
    )?;
    let rows = statement.query_map([case_id], |row| {
        Ok(ArtifactRow {
            id: row.get(0)?,
            case_id: row.get(1)?,
            kind: row.get(2)?,
            path: row.get(3)?,
            fingerprint: row.get(4)?,
            parent_artifact_id: row.get(5)?,
            metadata_json: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;
    let mut output = Vec::new();
    for row in rows {
        output.push(row?);
    }
    Ok(output)
}

#[derive(Debug, Clone)]
struct RefRow {
    id: String,
    path: String,
}

pub fn refs_json(refs: &[String]) -> StoreResult<String> {
    serde_json::to_string(refs).map_err(|error| StoreError::Sql(error.to_string()))
}

pub fn parse_refs(text: &str) -> Vec<String> {
    serde_json::from_str(text).unwrap_or_default()
}

pub fn refs_for_spec(
    conn: &Connection,
    task_id: i64,
    spec: &CheckSpec,
) -> StoreResult<Vec<String>> {
    let mut refs = latest_refs(conn, task_id)?
        .into_iter()
        .filter(|row| matches_spec(spec, &row.path))
        .map(|row| row.id)
        .collect::<Vec<_>>();
    refs.sort();
    Ok(refs)
}

pub fn suppress_stale_passed(
    conn: &Connection,
    task_id: i64,
    results: Vec<CheckResult>,
) -> StoreResult<Vec<CheckResult>> {
    let mut output = Vec::new();
    for result in results {
        if !stale_passed(conn, task_id, &result)? {
            output.push(result);
        }
    }
    Ok(output)
}

fn stale_passed(conn: &Connection, task_id: i64, result: &CheckResult) -> StoreResult<bool> {
    if !result.passed || result.artifact_refs.is_empty() {
        return Ok(false);
    }
    let Some(spec) = result.params.as_ref() else {
        return Ok(false);
    };
    Ok(result.artifact_refs != refs_for_spec(conn, task_id, spec)?)
}

fn latest_refs(conn: &Connection, task_id: i64) -> StoreResult<Vec<RefRow>> {
    let mut statement = conn.prepare(
        "SELECT id, path FROM artifacts WHERE case_id = ?1 AND parent_artifact_id IS NULL
         ORDER BY path, created_at DESC, id DESC",
    )?;
    let rows = statement.query_map(params![task_id.to_string()], |row| {
        Ok(RefRow {
            id: row.get(0)?,
            path: row.get(1)?,
        })
    })?;
    let mut output = Vec::new();
    for row in rows {
        let row = row?;
        if !output.iter().any(|item: &RefRow| item.path == row.path) {
            output.push(row);
        }
    }
    Ok(output)
}

fn matches_spec(spec: &CheckSpec, path: &str) -> bool {
    match spec {
        CheckSpec::FileExists { path: target }
        | CheckSpec::MinWords { path: target, .. }
        | CheckSpec::MaxLines { path: target, .. }
        | CheckSpec::Contains { path: target, .. }
        | CheckSpec::Absent { path: target, .. }
        | CheckSpec::Judged { path: target, .. } => path == target,
        CheckSpec::MinWordsTotal { glob, .. } | CheckSpec::FileCount { glob, .. } => {
            glob_match(glob, path)
        }
        CheckSpec::ReadmeCoverage { root } | CheckSpec::LinksResolve { root } => {
            in_root(root, path)
        }
        CheckSpec::Command { .. } => false,
    }
}

fn in_root(root: &str, path: &str) -> bool {
    root == "." || path == root || path.starts_with(&format!("{}/", root.trim_end_matches('/')))
}

fn glob_match(glob: &str, path: &str) -> bool {
    if let Some((prefix, suffix)) = glob.split_once('*') {
        path.starts_with(prefix) && path.ends_with(suffix)
    } else {
        path == glob
    }
}
