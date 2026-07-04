use lkjagent_core::engine::Command;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, TrustClass};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::observation_rows::{insert_observation, ObservationRow};
use rusqlite::Connection;

pub fn persist_observations(
    conn: &Connection,
    decision: &RuntimeDecision,
    snapshot: &TaskSnapshot,
    commands: &[Command],
    now: &str,
) -> Result<(), String> {
    for (index, command) in commands.iter().enumerate() {
        match command {
            Command::RunExplore(action) => {
                insert(conn, decision, index, &action.tool, snapshot, now)?
            }
            Command::WriteFile { path, .. } => insert(conn, decision, index, path, snapshot, now)?,
            Command::RecordAttempt(_)
            | Command::RecordEvent(_)
            | Command::RecordMemory { .. }
            | Command::RecordChecks { .. }
            | Command::AddSteps(_) => {}
        }
    }
    Ok(())
}

fn insert(
    conn: &Connection,
    decision: &RuntimeDecision,
    index: usize,
    effect_name: &str,
    snapshot: &TaskSnapshot,
    now: &str,
) -> Result<(), String> {
    let content = latest_observation(snapshot);
    let status = observation_status(&content);
    let contamination = contamination(status);
    let id = format!("{}-observation-{:04}", decision.id, index + 1);
    insert_observation(
        conn,
        &ObservationRow {
            id: id.clone(),
            case_id: decision.case_id.clone(),
            decision_id: decision.id.clone(),
            admission_id: Some(format!("{}-admission-{:04}", decision.id, index + 1)),
            effect_name: effect_name.to_string(),
            status: status.to_string(),
            content: content.clone(),
            artifact_refs_json: "[]".to_string(),
            contamination_class: format!("{:?}", contamination),
            created_at: now.to_string(),
        },
    )
    .map_err(|error| error.to_string())?;
    if !content.is_empty() {
        insert_context_item(
            conn,
            &decision.case_id,
            &context_item(&id, effect_name, content, contamination, now),
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
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

fn contamination(status: &str) -> ContaminationClass {
    if status == "ok" {
        ContaminationClass::Clean
    } else {
        ContaminationClass::RecoveryOnly
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
