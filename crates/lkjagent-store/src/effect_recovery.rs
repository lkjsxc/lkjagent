use std::path::{Component, Path};

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::{params, Connection};

use crate::error::StoreResult;

type Row = (
    String,
    String,
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
);

pub fn recover_unsettled_effects(
    conn: &mut Connection,
    workspace: &Path,
    now: &str,
) -> StoreResult<usize> {
    let rows = rows(conn)?;
    let tx = conn.transaction()?;
    for (id, admission, decision, case, effect, phase, target, prior, intended) in &rows {
        let (state, status, content) = state(workspace, phase, target.as_deref(), prior, intended);
        let observation = format!("{id}-recovery-observation");
        tx.execute(
            "INSERT INTO observations
             (id, case_id, decision_id, admission_id, effect_name, status, content,
              artifact_refs_json, contamination_class, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '[]', 'Clean', ?8)",
            params![
                observation,
                case,
                decision,
                admission,
                effect,
                status,
                content,
                now
            ],
        )?;
        let changed = tx.execute(
            "UPDATE effect_journal SET state = ?2, observation_id = ?3,
             outcome_fingerprint = ?4, updated_at = ?5
             WHERE id = ?1 AND state = ?6 AND observation_id IS NULL",
            params![
                id,
                state,
                observation,
                format!("recovery:{phase}"),
                now,
                phase
            ],
        )?;
        if changed != 1 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }
    }
    tx.commit()?;
    Ok(rows.len())
}

fn rows(conn: &Connection) -> StoreResult<Vec<Row>> {
    let mut statement = conn.prepare(
        "SELECT journal.id, journal.admission_id, journal.decision_id, admissions.case_id,
                journal.effect_name, journal.state, journal.target_path,
                journal.prior_fingerprint, journal.intended_fingerprint
         FROM effect_journal AS journal JOIN tool_admissions AS admissions
         ON admissions.id = journal.admission_id
         WHERE journal.state IN ('prepared', 'applying')",
    )?;
    let mapped = statement.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
        ))
    })?;
    Ok(mapped.collect::<Result<Vec<_>, _>>()?)
}

fn state(
    workspace: &Path,
    phase: &str,
    target: Option<&str>,
    prior: &str,
    intended: &str,
) -> (&'static str, &'static str, String) {
    if phase == "prepared" {
        return (
            "recovered",
            "ok",
            "recovery settled prepared effect without replay".to_string(),
        );
    }
    let Some(target) = target else {
        return (
            "failed",
            "error",
            "recovery cannot inspect applying non-file effect".to_string(),
        );
    };
    match fingerprints(workspace, target) {
        Ok((text, _)) if text == intended => (
            "recovered",
            "ok",
            "recovery observed intended file".to_string(),
        ),
        Ok((_, bytes)) if bytes == prior => (
            "failed",
            "error",
            "recovery observed prior file; effect was not replayed".to_string(),
        ),
        Ok(_) => (
            "failed",
            "error",
            "recovery found conflicting file; effect was not replayed".to_string(),
        ),
        Err(error) => (
            "failed",
            "error",
            format!("recovery could not inspect file: {error}"),
        ),
    }
}

fn fingerprints(workspace: &Path, target: &str) -> Result<(String, String), String> {
    let path = Path::new(target);
    if path.is_absolute()
        || path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err("target escapes workspace".to_string());
    }
    let bytes = std::fs::read(workspace.join(path)).map_err(|error| error.to_string())?;
    let text = String::from_utf8(bytes.clone()).map_err(|error| error.to_string())?;
    let text = stable_fingerprint(&text).map_err(|error| error.message)?;
    let bytes = stable_fingerprint(&Some(bytes)).map_err(|error| error.message)?;
    Ok((text, bytes))
}
