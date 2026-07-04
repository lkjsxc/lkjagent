use rusqlite::{params, Connection};

use crate::error::StoreResult;

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
