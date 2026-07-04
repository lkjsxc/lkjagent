use lkjagent_core::model::{CheckSpec, Step, StepKind, StepState, TaskSnapshot, TaskState};
use lkjagent_core::render::max_tokens;
use lkjagent_core::runtime_state::{EvidenceRef, StateCell, StateKey};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

pub fn projected_cell(snapshot: &TaskSnapshot, now: &str) -> Result<StateCell, String> {
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
        return CellParts::with_payload(
            "check",
            &step.id.to_string(),
            "plan-bridge.check.v1",
            serde_json::json!({"step_id": step.id}),
        );
    }
    CellParts::with_payload(
        "model",
        &step.id.to_string(),
        "plan-bridge.model.v1",
        serde_json::json!({
            "step_id": step.id,
            "expected_envelope": envelope(step.kind),
            "model_budget_tokens": max_tokens(step.kind),
            "tool_view": tool_view(step),
        }),
    )
}

struct CellParts {
    namespace: String,
    name: String,
    schema: String,
    payload: serde_json::Value,
}

impl CellParts {
    fn new(namespace: &str, name: &str, schema: &str) -> Self {
        Self::with_payload(namespace, name, schema, serde_json::json!({}))
    }

    fn with_payload(namespace: &str, name: &str, schema: &str, payload: serde_json::Value) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            schema: schema.to_string(),
            payload,
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
