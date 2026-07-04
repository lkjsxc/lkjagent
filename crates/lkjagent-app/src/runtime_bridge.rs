use lkjagent_core::model::{CheckSpec, Step, StepKind, StepState, TaskSnapshot, TaskState};
use lkjagent_core::render::max_tokens;
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;
use lkjagent_store::decision_rows::{
    insert_runtime_decision, next_decision_id, settle_decision, unfinished_decisions,
};
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case, upsert_state_cell};
use rusqlite::Connection;

use crate::recovery_bridge::recover_or_reuse;

pub fn prepare_runtime_decision(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    context_frame_fingerprint: &str,
    now: &str,
) -> Result<RuntimeDecision, String> {
    let case_id = snapshot.task.id.to_string();
    insert_case(conn, &case_id, &snapshot.task.objective, now)
        .map_err(|error| error.to_string())?;
    suppress_bridge_cells(conn, &case_id)?;
    let cell = next_work_cell(snapshot, now)?;
    upsert_state_cell(conn, &case_id, &cell).map_err(|error| error.to_string())?;
    let state_snapshot = hydrate_snapshot(conn, &case_id).map_err(|error| error.to_string())?;
    let unfinished = unfinished_decisions(conn, &case_id).map_err(|error| error.to_string())?;
    if let Some(decision) = recover_or_reuse(conn, &unfinished, now)? {
        return Ok(decision);
    }
    let id = next_decision_id(conn, &case_id).map_err(|error| error.to_string())?;
    let mut decision =
        select_runtime_decision(&state_snapshot, &id, &[]).map_err(|error| error.message)?;
    decision.context_frame_fingerprint = context_frame_fingerprint.to_string();
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

fn next_work_cell(snapshot: &TaskSnapshot, now: &str) -> Result<StateCell, String> {
    let parts = cell_parts(snapshot);
    let mut cell = StateCell::active(
        key(&parts.namespace, &parts.name)?,
        format!("task-{}", snapshot.task.id),
    );
    cell.payload_schema = parts.schema;
    cell.payload_json = parts.payload.to_string();
    cell.evidence_refs = vec![EvidenceRef {
        source_type: "task".to_string(),
        source_id: snapshot.task.id.to_string(),
        fingerprint: format!("budget-{}", snapshot.task.budget_used),
    }];
    cell.created_at = now.to_string();
    cell.updated_at = now.to_string();
    Ok(cell)
}

fn suppress_bridge_cells(conn: &Connection, case_id: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE state_cells SET status = 'Suppressed'
         WHERE case_id = ?1 AND payload_schema LIKE 'plan-bridge.%'",
        [case_id],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn cell_parts(snapshot: &TaskSnapshot) -> CellParts {
    match snapshot.task.state {
        TaskState::Waiting => CellParts::new("case", "waiting-answer", "plan-bridge.waiting.v1"),
        TaskState::Blocked | TaskState::Closed => {
            CellParts::new("runtime", "idle", "plan-bridge.idle.v1")
        }
        TaskState::Open => open_parts(snapshot),
    }
}

fn open_parts(snapshot: &TaskSnapshot) -> CellParts {
    let Some(step) = snapshot
        .steps
        .iter()
        .find(|step| matches!(step.state, StepState::Pending | StepState::Active))
    else {
        return CellParts::new("completion", "close-candidate", "plan-bridge.completion.v1");
    };
    if step.kind == StepKind::Verify && step.checks.iter().all(deterministic) {
        return CellParts {
            namespace: "check".to_string(),
            name: step.id.to_string(),
            schema: "plan-bridge.check.v1".to_string(),
            payload: serde_json::json!({"step_id": step.id}),
        };
    }
    CellParts {
        namespace: "model".to_string(),
        name: step.id.to_string(),
        schema: "plan-bridge.model.v1".to_string(),
        payload: serde_json::json!({
            "step_id": step.id,
            "expected_envelope": envelope(step.kind),
            "model_budget_tokens": max_tokens(step.kind),
            "tool_view": tool_view(step),
        }),
    }
}

struct CellParts {
    namespace: String,
    name: String,
    schema: String,
    payload: serde_json::Value,
}

impl CellParts {
    fn new(namespace: &str, name: &str, schema: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            schema: schema.to_string(),
            payload: serde_json::json!({}),
        }
    }
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
