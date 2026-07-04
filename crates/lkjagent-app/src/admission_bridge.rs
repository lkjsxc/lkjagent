use std::collections::BTreeMap;

use lkjagent_core::engine::Command;
use lkjagent_core::parse::Action;
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::admission_rows::insert_tool_admission;
use rusqlite::Connection;

pub fn persist_tool_admissions(
    conn: &Connection,
    decision: &RuntimeDecision,
    commands: &[Command],
    now: &str,
) -> Result<(), String> {
    for (index, command) in commands.iter().enumerate() {
        let Command::RunExplore(action) = command else {
            continue;
        };
        let model_action = model_action(action);
        let admission = admit_action(decision, &model_action).map_err(|error| error.message)?;
        let id = format!("{}-admission-{:04}", decision.id, index + 1);
        let parsed = serde_json::to_string(&model_action).map_err(|error| error.to_string())?;
        insert_tool_admission(conn, &id, &decision.case_id, &admission, &parsed, now)
            .map_err(|error| error.to_string())?;
        if admission.status == AdmissionStatus::Rejected {
            return Err(format!("admission rejected: {}", admission.reason));
        }
    }
    Ok(())
}

fn model_action(action: &Action) -> ModelAction {
    ModelAction {
        tool: action.tool.clone(),
        params: action
            .params
            .iter()
            .filter(|(name, _)| name != "tool")
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<BTreeMap<_, _>>(),
    }
}
