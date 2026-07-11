use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::{params, Connection};

use crate::error::{StoreError, StoreResult};

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

pub fn settle_effect_observation(
    conn: &mut Connection,
    journal_id: &str,
    state: &str,
    row: &ObservationRow,
) -> StoreResult<()> {
    if !matches!(state, "committed" | "failed") {
        return Err(StoreError::InvalidState(format!(
            "invalid journal settlement {state}"
        )));
    }
    let expected: (String, String, String, String) = conn.query_row(
        "SELECT journal.admission_id, journal.decision_id, admissions.case_id, journal.effect_name
         FROM effect_journal AS journal JOIN tool_admissions AS admissions
         ON admissions.id = journal.admission_id WHERE journal.id = ?1",
        [journal_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    if row.admission_id.as_deref() != Some(&expected.0)
        || row.decision_id != expected.1
        || row.case_id != expected.2
        || row.effect_name != expected.3
    {
        return Err(StoreError::InvalidState(
            "observation does not match journal".to_string(),
        ));
    }
    let outcome = stable_fingerprint(&(state, &row.status, &row.content))
        .map_err(|error| StoreError::InvalidState(error.message))?;
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO observations
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
            row.created_at
        ],
    )?;
    let changed = tx.execute(
        "UPDATE effect_journal SET state = ?2, observation_id = ?3,
         outcome_fingerprint = ?4, updated_at = ?5
         WHERE id = ?1 AND state = 'applying' AND observation_id IS NULL",
        params![journal_id, state, row.id, outcome, row.created_at],
    )?;
    if changed != 1 {
        return Err(rusqlite::Error::QueryReturnedNoRows.into());
    }
    tx.commit()?;
    Ok(())
}
