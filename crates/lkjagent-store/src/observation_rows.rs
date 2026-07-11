use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::{params, Connection, OptionalExtension};

use crate::admission_rows::effect_targets;
use crate::artifact_rows::{insert_artifact, refs_json, ArtifactRow};
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
    conn: &Connection,
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
    if !conn.is_autocommit() {
        return settle_rows(conn, journal_id, state, row, &outcome);
    }
    let tx = conn.unchecked_transaction()?;
    settle_rows(&tx, journal_id, state, row, &outcome)?;
    tx.commit()?;
    Ok(())
}

fn settle_rows(
    conn: &Connection,
    journal_id: &str,
    state: &str,
    row: &ObservationRow,
    outcome: &str,
) -> StoreResult<()> {
    if state == "committed" {
        let expected_refs = refs_json(&insert_effect_artifacts(conn, journal_id)?)?;
        if row.artifact_refs_json != expected_refs {
            return Err(StoreError::InvalidState(
                "observation artifact refs do not match durable intents".to_string(),
            ));
        }
    }
    conn.execute(
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
    let changed = conn.execute(
        "UPDATE effect_journal SET state = ?2, observation_id = ?3,
         outcome_fingerprint = ?4, updated_at = ?5
         WHERE id = ?1 AND state = 'applying' AND observation_id IS NULL",
        params![journal_id, state, row.id, outcome, row.created_at],
    )?;
    if changed != 1 {
        return Err(rusqlite::Error::QueryReturnedNoRows.into());
    }
    Ok(())
}

pub fn insert_effect_artifacts(conn: &Connection, journal_id: &str) -> StoreResult<Vec<String>> {
    let mut artifacts = effect_targets(conn, journal_id)?
        .into_iter()
        .flat_map(|target| target.artifacts)
        .collect::<Vec<_>>();
    artifacts.sort_by_key(|row| row.parent_artifact_id.is_some());
    let mut refs = Vec::new();
    for artifact in artifacts {
        insert_exact_artifact(conn, &artifact)?;
        if artifact.parent_artifact_id.is_none() {
            refs.push(artifact.id);
        }
    }
    refs.sort();
    refs.dedup();
    Ok(refs)
}

fn insert_exact_artifact(conn: &Connection, intended: &ArtifactRow) -> StoreResult<()> {
    if let Some(parent) = &intended.parent_artifact_id {
        let case_id = conn
            .query_row(
                "SELECT case_id FROM artifacts WHERE id = ?1",
                [parent],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        if case_id.as_deref() != Some(&intended.case_id) {
            return Err(StoreError::InvalidState(format!(
                "artifact parent is missing or cross-case for {}",
                intended.id
            )));
        }
    }
    let existing = conn
        .query_row(
            "SELECT id, case_id, kind, path, fingerprint, parent_artifact_id,
             metadata_json, created_at FROM artifacts WHERE id = ?1",
            [&intended.id],
            |row| {
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
            },
        )
        .optional()?;
    match existing {
        None => insert_artifact(conn, intended),
        Some(row) if same_artifact(&row, intended) => Ok(()),
        Some(_) => Err(StoreError::InvalidState(format!(
            "artifact intent conflicts for {}",
            intended.id
        ))),
    }
}

pub use crate::row_support::target_fingerprint;

fn same_artifact(left: &ArtifactRow, right: &ArtifactRow) -> bool {
    left.id == right.id
        && left.case_id == right.case_id
        && left.kind == right.kind
        && left.path == right.path
        && left.fingerprint == right.fingerprint
        && left.parent_artifact_id == right.parent_artifact_id
        && left.metadata_json == right.metadata_json
}
