use std::path::{Component, Path, PathBuf};

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::{params, Connection};

use crate::admission_rows::{effect_targets, EffectTargetRevision};
use crate::artifact_rows::refs_json;
use crate::error::{StoreError, StoreResult};
use crate::observation_rows::{insert_effect_artifacts, target_fingerprint};

struct JournalRow {
    id: String,
    admission: String,
    decision: String,
    case_id: String,
    effect: String,
    phase: String,
    target: Option<String>,
    prior: String,
    intended: String,
}

type Outcome = (&'static str, &'static str, String);

pub fn recover_unsettled_effects(
    conn: &mut Connection,
    workspace: &Path,
    now: &str,
) -> StoreResult<usize> {
    let rows = rows(conn)?;
    let mut planned = Vec::new();
    for row in rows {
        let targets = effect_targets(conn, &row.id)?;
        planned.push((row, targets));
    }
    let count = planned.len();
    let tx = conn.transaction()?;
    for (row, targets) in planned {
        let (state, status, content) = state(workspace, &row, &targets);
        if state == "recovery_required" {
            return Err(StoreError::InvalidState(content));
        }
        let artifact_refs_json =
            if state == "recovered" && row.phase == "applying" && !targets.is_empty() {
                refs_json(&insert_effect_artifacts(&tx, &row.id)?)?
            } else {
                "[]".to_string()
            };
        let observation = format!("{}-recovery-observation", row.id);
        tx.execute(
            "INSERT INTO observations
             (id, case_id, decision_id, admission_id, effect_name, status, content,
              artifact_refs_json, contamination_class, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'Clean', ?9)",
            params![
                observation,
                row.case_id,
                row.decision,
                row.admission,
                row.effect,
                status,
                content,
                artifact_refs_json,
                now
            ],
        )?;
        let changed = tx.execute(
            "UPDATE effect_journal SET state = ?2, observation_id = ?3,
             outcome_fingerprint = ?4, updated_at = ?5
             WHERE id = ?1 AND state = ?6 AND observation_id IS NULL",
            params![
                row.id,
                state,
                observation,
                format!("recovery:{}", row.phase),
                now,
                row.phase
            ],
        )?;
        if changed != 1 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }
    }
    tx.commit()?;
    Ok(count)
}

#[rustfmt::skip]
fn rows(conn: &Connection) -> StoreResult<Vec<JournalRow>> {
    let mut statement = conn.prepare(
        "SELECT journal.id, journal.admission_id, journal.decision_id, admissions.case_id,
                journal.effect_name, journal.state, journal.target_path,
                journal.prior_fingerprint, journal.intended_fingerprint
         FROM effect_journal AS journal JOIN tool_admissions AS admissions
         ON admissions.id = journal.admission_id
         WHERE journal.state IN ('prepared', 'applying')")?;
    let mapped = statement.query_map([], |row| Ok(JournalRow {
        id: row.get(0)?, admission: row.get(1)?, decision: row.get(2)?, case_id: row.get(3)?,
        effect: row.get(4)?, phase: row.get(5)?, target: row.get(6)?, prior: row.get(7)?, intended: row.get(8)? }))?;
    Ok(mapped.collect::<Result<Vec<_>, _>>()?)
}

fn state(workspace: &Path, row: &JournalRow, targets: &[EffectTargetRevision]) -> Outcome {
    if row.phase == "prepared" {
        return (
            "failed",
            "error",
            "recovery failed prepared effect without replay".to_string(),
        );
    }
    if !targets.is_empty() {
        return target_state(workspace, targets);
    }
    let Some(target) = row.target.as_deref() else {
        return (
            "failed",
            "error",
            "recovery cannot inspect applying non-file effect".to_string(),
        );
    };
    match legacy_fingerprints(workspace, target) {
        Ok((text, _)) if text == row.intended => (
            "recovered",
            "ok",
            "recovery observed intended file".to_string(),
        ),
        Ok((_, bytes)) if bytes == row.prior => (
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

#[rustfmt::skip]
fn target_state(workspace: &Path, targets: &[EffectTargetRevision]) -> Outcome {
    let mut prior = 0;
    for target in targets {
        let actual = match target_fingerprint(workspace, target) {
            Ok(actual) => actual,
            Err(error) => return ("recovery_required", "error", format!("bundle target unavailable {}: {error}", target.path)),
        };
        if actual == target.intended_fingerprint { continue; }
        if actual == target.prior_fingerprint { prior += 1; continue; }
        return ("recovery_required", "error", format!("bundle target conflicts: {}", target.path));
    }
    if prior == 0 { ("recovered", "ok", format!("recovery observed {} intended bundle targets", targets.len())) }
    else if prior == targets.len() { ("failed", "error", "recovery observed the complete prior bundle".to_string()) }
    else { ("recovery_required", "error", format!("recovery found partial bundle: {prior}/{} targets remain prior", targets.len())) }
}

fn legacy_fingerprints(workspace: &Path, target: &str) -> Result<(String, String), String> {
    let bytes =
        std::fs::read(resolve_target(workspace, target)?).map_err(|error| error.to_string())?;
    let text = String::from_utf8(bytes.clone()).map_err(|error| error.to_string())?;
    let text = stable_fingerprint(&text).map_err(|error| error.message)?;
    let bytes = stable_fingerprint(&Some(bytes)).map_err(|error| error.message)?;
    Ok((text, bytes))
}

fn resolve_target(workspace: &Path, target: &str) -> Result<PathBuf, String> {
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
    let joined = workspace.join(path);
    if joined.exists() {
        let root = workspace
            .canonicalize()
            .map_err(|error| error.to_string())?;
        let resolved = joined.canonicalize().map_err(|error| error.to_string())?;
        if !resolved.starts_with(root) {
            return Err("target resolves outside workspace".to_string());
        }
    }
    Ok(joined)
}
