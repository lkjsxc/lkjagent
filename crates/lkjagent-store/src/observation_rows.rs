use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub struct ObservationRow {
    pub id: String,
    pub case_id: String,
    pub decision_id: String,
    pub admission_id: Option<String>,
    pub effect_name: String,
    pub status: String,
    pub content: String,
    pub artifact_refs_json: String,
    pub contamination_class: String,
    pub created_at: String,
}

pub fn insert_observation(conn: &Connection, row: &ObservationRow) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO observations
         (id, case_id, decision_id, admission_id, effect_name, status, content,
          artifact_refs_json, contamination_class, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            row.id,
            row.case_id,
            row.decision_id,
            row.admission_id,
            row.effect_name,
            row.status,
            row.content,
            row.artifact_refs_json,
            row.contamination_class,
            row.created_at,
        ],
    )?;
    Ok(())
}
