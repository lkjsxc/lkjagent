use std::collections::BTreeMap;

use lkjagent_core::engine::Command;
use lkjagent_core::model::{CheckSpec, Step, StepKind, StepState, TaskSnapshot, TaskState};
use lkjagent_core::parse::Action;
use lkjagent_core::render::max_tokens;
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;
use lkjagent_store::admission_rows::insert_tool_admission;
use lkjagent_store::decision_rows::{
    insert_runtime_decision, next_decision_id, settle_decision, unfinished_decisions,
};
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case, upsert_state_cell};
use rusqlite::Connection;

pub fn prepare_runtime_decision(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    now: &str,
) -> Result<RuntimeDecision, String> {
    let case_id = snapshot.task.id.to_string();
    insert_case(conn, &case_id, &snapshot.task.objective, now)
        .map_err(|error| error.to_string())?;
    let cell = next_work_cell(snapshot, now)?;
    upsert_state_cell(conn, &case_id, &cell).map_err(|error| error.to_string())?;
    let state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    let unfinished = unfinished_decisions(conn, &case_id).map_err(|error| error.to_string())?;
    if let Some(decision) = unfinished.first() {
        return Ok(decision.clone());
    }
    let id = next_decision_id(conn, &case_id).map_err(|error| error.to_string())?;
    let decision =
        select_runtime_decision(&state_snapshot, &id, &[]).map_err(|error| error.message)?;
    insert_runtime_decision(conn, &decision, "pending", now).map_err(|error| error.to_string())?;
    Ok(decision)
}

pub fn settle_runtime_decision(
    conn: &Connection,
    decision: &RuntimeDecision,
    status: &str,
    now: &str,
) -> Result<(), String> {
    settle_decision(conn, &decision.id, status, now)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

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

fn next_work_cell(snapshot: &TaskSnapshot, now: &str) -> Result<StateCell, String> {
    let mut cell = StateCell::active(
        key("runtime", "next-work")?,
        format!("task-{}", snapshot.task.id),
    );
    cell.payload_schema = "plan-bridge.next-work.v1".to_string();
    cell.payload_json = payload(snapshot).to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "task".to_string(),
        source_id: snapshot.task.id.to_string(),
        fingerprint: format!("budget-{}", snapshot.task.budget_used),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    Ok(cell)
}

fn payload(snapshot: &TaskSnapshot) -> serde_json::Value {
    match snapshot.task.state {
        TaskState::Waiting => serde_json::json!({"operation": "owner.answer"}),
        TaskState::Blocked | TaskState::Closed => serde_json::json!({"operation": "runtime.idle"}),
        TaskState::Open => open_payload(snapshot),
    }
}

fn open_payload(snapshot: &TaskSnapshot) -> serde_json::Value {
    let Some(step) = snapshot
        .steps
        .iter()
        .find(|step| matches!(step.state, StepState::Pending | StepState::Active))
    else {
        return serde_json::json!({"operation": "completion.close"});
    };
    if step.kind == StepKind::Verify && step.checks.iter().all(deterministic) {
        return serde_json::json!({"operation": "check.run", "step_id": step.id});
    }
    serde_json::json!({
        "operation": "model.call",
        "step_id": step.id,
        "expected_envelope": envelope(step.kind),
        "model_budget_tokens": max_tokens(step.kind),
        "tool_view": tool_view(step),
    })
}

fn envelope(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Write | StepKind::Revise => "Content",
        StepKind::Plan => "Plan",
        StepKind::Explore => "Action",
        StepKind::Respond | StepKind::Ask => "Message",
        StepKind::Verify => "Verdict",
    }
}

fn tool_view(step: &Step) -> Vec<serde_json::Value> {
    if step.kind != StepKind::Explore {
        return Vec::new();
    }
    explore_tool_view()
        .entries
        .into_iter()
        .map(|entry| {
            serde_json::json!({
                "name": entry.name,
                "purpose": entry.purpose,
                "required_params": entry.required_params,
                "optional_params": entry.optional_params,
            })
        })
        .collect()
}

fn deterministic(spec: &CheckSpec) -> bool {
    !matches!(spec, CheckSpec::Judged { .. })
}

fn key(namespace: &str, name: &str) -> Result<StateKey, String> {
    StateKey::new(namespace, name).map_err(|error| error.message)
}
