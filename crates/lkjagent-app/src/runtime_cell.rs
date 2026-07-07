use lkjagent_core::engine::completion_blocker_reason;
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
        TaskState::Waiting => CellParts::new("case", "waiting-answer", "plan-bridge.waiting"),
        TaskState::Blocked | TaskState::Closed => {
            CellParts::new("runtime", "idle", "plan-bridge.idle")
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
        if let Some(reason) = completion_blocker_reason(snapshot) {
            return CellParts::with_payload(
                "completion",
                "blocked",
                "plan-bridge.blocked",
                serde_json::json!({"reason": reason}),
            );
        }
        return CellParts::new("completion", "close-candidate", "plan-bridge.completion");
    };
    if step.kind == StepKind::Verify && step.checks.iter().all(deterministic) {
        return CellParts::with_payload(
            "check",
            &step.id.to_string(),
            "plan-bridge.check",
            serde_json::json!({"step_id": step.id}),
        );
    }
    CellParts::with_payload(
        "model",
        &step.id.to_string(),
        "plan-bridge.model",
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

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_core::classify::instantiate;
    use lkjagent_core::model::StepState;

    #[test]
    fn blocked_step_projects_completion_blocked() {
        let mut snapshot = instantiate(1, "Create something to read");
        snapshot.steps[0].state = StepState::Blocked;
        for step in snapshot.steps.iter_mut().skip(1) {
            step.state = StepState::Done;
        }

        let cell = test_cell(&snapshot);

        assert_eq!(cell.key.namespace, "completion");
        assert_eq!(cell.key.name, "blocked");
        assert_eq!(cell.payload_schema, "plan-bridge.blocked");
        assert!(cell.payload_json.contains("blocked"));
    }

    #[test]
    fn all_done_bridge_projects_close_candidate() {
        let mut snapshot = instantiate(2, "are you ok?");
        for step in &mut snapshot.steps {
            step.state = StepState::Done;
        }

        let cell = test_cell(&snapshot);

        assert_eq!(cell.key.namespace, "completion");
        assert_eq!(cell.key.name, "close-candidate");
    }

    fn test_cell(snapshot: &TaskSnapshot) -> StateCell {
        let result = projected_cell(snapshot, "now");
        assert!(result.is_ok());
        match result {
            Ok(cell) => cell,
            Err(_) => StateCell::active(
                StateKey {
                    namespace: "invalid".to_string(),
                    name: "invalid".to_string(),
                },
                "test",
            ),
        }
    }
}
