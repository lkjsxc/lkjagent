use std::path::Path;

use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_context::{
    contamination_for_observation, redact_sensitive_owner_data, ContaminationClass, ContextItem,
    TrustClass,
};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::PreparedEffect;
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::observation_rows::{settle_effect_observation, ObservationRow};
use rusqlite::Connection;

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

pub fn persist_failed_observations(
    conn: &mut Connection,
    decision: &RuntimeDecision,
    effects: &[PreparedEffect],
    error: &str,
    now: &str,
) -> Result<(), String> {
    for (index, effect) in effects.iter().enumerate() {
        let row = observation(decision, index, effect, "error", error, "Clean", now);
        settle_effect_observation(conn, &effect.journal_id, "failed", &row)
            .map_err(|error| error.to_string())?;
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
    );
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
            &context_item(&row.id, &effect.effect_name, content, contamination, now),
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn observation(
    decision: &RuntimeDecision,
    index: usize,
    effect: &PreparedEffect,
    status: &str,
    content: &str,
    contamination: &str,
    now: &str,
) -> ObservationRow {
    ObservationRow {
        id: format!("{}-observation-{:04}", decision.id, index + 1),
        case_id: decision.case_id.clone(),
        decision_id: decision.id.clone(),
        admission_id: Some(effect.admission_id.clone()),
        effect_name: effect.effect_name.clone(),
        status: status.to_string(),
        content: content.to_string(),
        artifact_refs_json: "[]".to_string(),
        contamination_class: contamination.to_string(),
        created_at: now.to_string(),
    }
}

fn effect_outcome(
    workspace: &Path,
    effect: &PreparedEffect,
    snapshot: &TaskSnapshot,
) -> (&'static str, String) {
    let Some(path) = effect.target_path.as_deref() else {
        let content = latest_observation(snapshot);
        return (observation_status(&content), content);
    };
    match lkjagent_effects::workspace::resolve(workspace, path)
        .map_err(|error| error.to_string())
        .and_then(|full| std::fs::read(full).map_err(|error| error.to_string()))
        .and_then(|bytes| stable_fingerprint(&bytes).map_err(|error| error.message))
    {
        Ok(fingerprint) => ("ok", format!("path={path}\nfingerprint={fingerprint}")),
        Err(error) => ("error", format!("postcondition unavailable: {error}")),
    }
}

fn stored_content(content: &str, contamination: ContaminationClass) -> String {
    if contamination == ContaminationClass::SensitiveOwnerData {
        redact_sensitive_owner_data(content)
    } else {
        content.to_string()
    }
}

fn context_item(
    id: &str,
    effect_name: &str,
    content: String,
    contamination: ContaminationClass,
    now: &str,
) -> ContextItem {
    let mut item = ContextItem::clean_fact(
        format!("context-{id}"),
        format!("observation/{effect_name}"),
        content,
    );
    item.source_type = "observation".to_string();
    item.source_id = id.to_string();
    item.source_fingerprint = format!("observation:{id}");
    item.trust = TrustClass::Measured;
    item.contamination = contamination;
    item.decision_id = id.split("-observation-").next().map(str::to_string);
    item.created_at = now.to_string();
    item
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
