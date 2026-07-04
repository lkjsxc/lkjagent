use lkjagent_core::engine::Command;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_decision::RuntimeDecision;
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
    insert_observation(
        conn,
        &ObservationRow {
            id: format!("{}-observation-{:04}", decision.id, index + 1),
            case_id: decision.case_id.clone(),
            decision_id: decision.id.clone(),
            admission_id: Some(format!("{}-admission-{:04}", decision.id, index + 1)),
            effect_name: effect_name.to_string(),
            status: observation_status(&content).to_string(),
            content,
            artifact_refs_json: "[]".to_string(),
            contamination_class: "Clean".to_string(),
            created_at: now.to_string(),
        },
    )
    .map_err(|error| error.to_string())
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
