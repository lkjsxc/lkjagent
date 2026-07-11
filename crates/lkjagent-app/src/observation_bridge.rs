use std::path::Path;

use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_context::{
    contamination_for_observation, redact_sensitive_owner_data, ContaminationClass,
};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{mark_journal, PreparedEffect};
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::observation_rows::{settle_effect_observation, ObservationRow};
use rusqlite::Connection;

use crate::context_bridge::observation_context_item;
use crate::effect_dispatch::DispatchFailure;

pub fn persist_observations(
    conn: &mut Connection,
    workspace: &Path,
    decision: &RuntimeDecision,
    snapshot: &TaskSnapshot,
    effects: &[PreparedEffect],
    now: &str,
) -> Result<(), String> {
    for (index, effect) in effects.iter().enumerate() {
        insert(conn, workspace, decision, index, effect, snapshot, now)?;
    }
    Ok(())
}

pub fn settle_dispatch_failure(
    conn: &mut Connection,
    workspace: &Path,
    decision: &RuntimeDecision,
    snapshot: &TaskSnapshot,
    effects: &[PreparedEffect],
    failure: &DispatchFailure,
    now: &str,
) -> Result<(), String> {
    let succeeded = failure.completed;
    let completed = effects
        .get(..succeeded)
        .ok_or_else(|| "dispatch success count is invalid".to_string())?;
    persist_observations(conn, workspace, decision, snapshot, completed, now)?;
    let pending = effects
        .get(succeeded..)
        .ok_or_else(|| "dispatch pending range is invalid".to_string())?;
    persist_failed_observations(conn, decision, pending, failure, now)
}

#[rustfmt::skip]
fn persist_failed_observations(conn: &mut Connection, decision: &RuntimeDecision,
    effects: &[PreparedEffect], failure: &DispatchFailure, now: &str) -> Result<(), String> {
    let attempted_count = usize::from(failure.failed_current && !failure.recovery_required);
    let attempted = effects.get(..attempted_count).ok_or_else(|| "dispatch attempted too many effects".to_string())?;
    for (index, effect) in attempted.iter().enumerate() {
        let row = observation(decision, failure.completed + index, effect, "error", &failure.error, "Clean", now)?;
        settle_effect_observation(conn, &effect.journal_id, "failed", &row).map_err(|error| error.to_string())?;
    }
    let preserved = usize::from(failure.failed_current && failure.recovery_required);
    for effect in effects.get(attempted.len() + preserved..).ok_or_else(|| "dispatch effect range is invalid".to_string())? {
        mark_journal(conn, &effect.journal_id, "compensated", now).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn insert(
    conn: &mut Connection,
    workspace: &Path,
    decision: &RuntimeDecision,
    index: usize,
    effect: &PreparedEffect,
    snapshot: &TaskSnapshot,
    now: &str,
) -> Result<(), String> {
    let (status, raw_content) = effect_outcome(workspace, effect, snapshot);
    let contamination = contamination_for_observation(&effect.effect_name, status, &raw_content);
    let content = stored_content(&raw_content, contamination);
    let row = observation(
        decision,
        index,
        effect,
        status,
        &content,
        &format!("{:?}", contamination),
        now,
    )?;
    let state = if status == "ok" {
        "committed"
    } else {
        "failed"
    };
    settle_effect_observation(conn, &effect.journal_id, state, &row)
        .map_err(|error| error.to_string())?;
    if !content.is_empty() {
        insert_context_item(
            conn,
            &decision.case_id,
            &observation_context_item(&row.id, &effect.effect_name, content, contamination, now),
        )
        .map_err(|error| error.to_string())?;
    }
    if status == "ok" {
        Ok(())
    } else {
        Err(format!("effect postcondition failed: {raw_content}"))
    }
}

#[rustfmt::skip]
fn observation(decision: &RuntimeDecision, index: usize, effect: &PreparedEffect, status: &str,
    content: &str, contamination: &str, now: &str) -> Result<ObservationRow, String> {
    let artifact_refs_json = if status == "ok" { artifact_refs(effect)? } else { "[]".to_string() };
    Ok(ObservationRow { id: format!("{}-observation-{:04}", decision.id, index + 1),
        case_id: decision.case_id.clone(), decision_id: decision.id.clone(),
        admission_id: Some(effect.admission_id.clone()), effect_name: effect.effect_name.clone(),
        status: status.to_string(), content: content.to_string(), artifact_refs_json,
        contamination_class: contamination.to_string(), created_at: now.to_string() })
}

fn effect_outcome(
    workspace: &Path,
    effect: &PreparedEffect,
    snapshot: &TaskSnapshot,
) -> (&'static str, String) {
    if !effect.targets.is_empty() {
        return match verify_targets(workspace, effect) {
            Ok(content) => ("ok", content),
            Err(content) => ("error", content),
        };
    }
    let Some(path) = effect.target_path.as_deref() else {
        let content = latest_observation(snapshot);
        return (observation_status(&content), content);
    };
    match lkjagent_effects::workspace::resolve(workspace, path)
        .map_err(|error| error.to_string())
        .and_then(|full| std::fs::read_to_string(full).map_err(|error| error.to_string()))
        .and_then(|body| stable_fingerprint(&body).map_err(|error| error.message))
    {
        Ok(fingerprint) if fingerprint == effect.intended_fingerprint => {
            ("ok", format!("path={path}\nfingerprint={fingerprint}"))
        }
        Ok(fingerprint) => ("error", format!("postcondition mismatch: {fingerprint}")),
        Err(error) => ("error", format!("postcondition unavailable: {error}")),
    }
}

#[rustfmt::skip]
fn verify_targets(workspace: &Path, effect: &PreparedEffect) -> Result<String, String> {
    for target in &effect.targets {
        let fingerprint = lkjagent_store::observation_rows::target_fingerprint(workspace, target)
            .map_err(|error| error.to_string())?;
        if fingerprint != target.intended_fingerprint { return Err(format!("postcondition mismatch: {} {fingerprint}", target.path)); }
    }
    let path = effect.target_path.as_deref().unwrap_or("bundle");
    Ok(format!("path={path}\ntargets={} bundle_fingerprint={}", effect.targets.len(), effect.intended_fingerprint))
}

#[rustfmt::skip]
fn artifact_refs(effect: &PreparedEffect) -> Result<String, String> {
    let mut refs = effect.targets.iter().flat_map(|target| &target.artifacts)
        .filter(|artifact| artifact.parent_artifact_id.is_none()).map(|artifact| artifact.id.clone()).collect::<Vec<_>>();
    refs.sort(); refs.dedup();
    lkjagent_store::artifact_rows::refs_json(&refs).map_err(|error| error.to_string())
}

fn stored_content(content: &str, contamination: ContaminationClass) -> String {
    if contamination == ContaminationClass::SensitiveOwnerData {
        redact_sensitive_owner_data(content)
    } else {
        content.to_string()
    }
}

fn latest_observation(snapshot: &TaskSnapshot) -> String {
    snapshot
        .steps
        .iter()
        .find(|step| step.inputs.contains("latest_observation="))
        .map_or_else(String::new, |step| step.inputs.clone())
}

fn observation_status(content: &str) -> &'static str {
    if content.contains("<status>error</status>") {
        "error"
    } else {
        "ok"
    }
}
