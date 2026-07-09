use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_core::runtime_selector::{candidates, select_runtime_decision};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
use lkjagent_core::runtime_state_edge::{StateEdge, StateEdgeRelation, StateRef};

#[test]
fn selector_reuses_unfinished_decision_before_new_work() {
    let snapshot = snapshot_with(cell("model", "call/7"));
    let mut unfinished = RuntimeDecision::new(
        "decision-existing",
        "case-1",
        OperationKey("model.call/7".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    unfinished.context_frame_fingerprint = "recorded-context".to_string();

    let selected = select(&snapshot, "decision-new", &[unfinished]);

    assert_eq!(selected.id, "decision-existing");
    assert_eq!(selected.context_frame_fingerprint, "recorded-context");
}

#[test]
fn selector_reads_model_cells_and_preserves_tool_view() {
    let snapshot = snapshot_with(model_cell());

    let selected = select(&snapshot, "decision-1", &[]);

    assert_eq!(selected.selected_state_key.as_deref(), Some("model:42"));
    assert_eq!(selected.operation.0, "model.call/42");
    assert_eq!(selected.expected_envelope, OutputEnvelope::Action);
    assert_eq!(selected.model_budget_tokens, Some(512));
    assert_eq!(selected.tool_view.tool_names(), vec!["fs.read"]);
    assert!(!selected.snapshot_fingerprint.is_empty());
    let mut exhausted = model_cell();
    exhausted.payload_json = exhausted
        .payload_json
        .replace("\"tool_budget_remaining\":1", "\"tool_budget_remaining\":0");
    let selected = select(&snapshot_with(exhausted), "decision-2", &[]);
    assert!(selected.tool_view.entries.is_empty());
    assert!(selected
        .evidence_requirements
        .contains(&String::from("tool-budget:suppressed")));
}

#[test]
fn selector_orders_recovery_before_model_cells() {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let model = cell("model", "call/1");
    let recovery = cell("recovery", "endpoint-loss");
    snapshot.cells.insert(model.key.clone(), model);
    snapshot.cells.insert(recovery.key.clone(), recovery);

    let selected = select(&snapshot, "decision-1", &[]);

    assert_eq!(selected.operation.0, "recovery.handle/endpoint-loss");
    assert_eq!(selected.expected_envelope, OutputEnvelope::None);
}

#[test]
fn selector_uses_idle_when_no_active_cells_exist() {
    let snapshot = RuntimeSnapshot::empty("case-1");

    let selected = select(&snapshot, "decision-1", &[]);

    assert_eq!(selected.operation.0, "runtime.idle");
    assert_eq!(selected.expected_envelope, OutputEnvelope::None);
}

#[test]
fn selector_skips_candidates_blocked_by_state_edges() {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let model = cell("model", "1");
    let check = cell("check", "1");
    snapshot.cells.insert(model.key.clone(), model);
    snapshot.cells.insert(check.key.clone(), check);
    snapshot.edges.insert(
        "edge-1".to_string(),
        StateEdge::active(
            "edge-1",
            "case-1",
            StateRef::new("state", "context:conflict/root"),
            StateRef::new("state", "model:1"),
            StateEdgeRelation::blocks(),
            "event-1",
        ),
    );

    let items = candidates(&snapshot);
    let blocked = items
        .iter()
        .find(|item| item.operation.key == "model.call/1");
    assert_eq!(
        blocked.map(|item| item.blocked_by.clone()),
        Some(vec!["edge-1".to_string()])
    );
    let selected = select(&snapshot, "decision-1", &[]);
    assert_eq!(selected.operation.0, "check.run/1");
}

#[test]
fn payload_operation_selects_unknown_state_namespace() {
    let mut custom = cell("calendar", "due/today");
    custom.payload_json = serde_json::json!({
        "operation_key": "model.call/42",
        "expected_envelope": "Message",
        "selector_tier": 25,
        "evidence_requirements": ["record:todo-1"],
        "effect_command": {"name":"workspace.write_text","path":"native.md","content":"body"}
    })
    .to_string();
    let snapshot = snapshot_with(custom);

    let selected = select(&snapshot, "decision-1", &[]);

    assert_eq!(
        selected.selected_state_key.as_deref(),
        Some("calendar:due/today")
    );
    assert_eq!(selected.operation.0, "model.call/42");
    assert_eq!(selected.expected_envelope, OutputEnvelope::Message);
    assert!(selected.effect_command.is_none());
    assert_eq!(
        selected.evidence_requirements,
        vec!["selector:payload", "record:todo-1"]
    );
}

#[test]
fn payload_model_free_operation_carries_effect_command() {
    let mut custom = cell("effect", "write/native");
    custom.payload_json = serde_json::json!({
        "operation_key": "effect.workspace.write",
        "effect_command": {"name":"workspace.write_text","path":"native.md","content":"body"}
    })
    .to_string();
    let selected = select(&snapshot_with(custom), "decision-1", &[]);

    assert_eq!(selected.operation.0, "effect.workspace.write");
    assert!(selected.effect_command.is_some());
    let Some(effect) = selected.effect_command else {
        return;
    };
    assert_eq!(effect.name, "workspace.write_text");
    assert_eq!(effect.path.as_deref(), Some("native.md"));
    assert_eq!(effect.content.as_deref(), Some("body"));
}

fn snapshot_with(cell: StateCell) -> RuntimeSnapshot {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot.cells.insert(cell.key.clone(), cell);
    snapshot
}

fn cell(namespace: &str, name: &str) -> StateCell {
    StateCell::active(key(namespace, name), "event-1")
}

fn model_cell() -> StateCell {
    let mut cell = cell("model", "42");
    cell.payload_schema = "state.model".to_string();
    cell.payload_json = serde_json::json!({
        "expected_envelope": "Action",
        "model_budget_tokens": 512,
        "tool_budget_remaining": 1,
        "tool_view": [{
            "name": "fs.read",
            "purpose": "read a workspace file",
            "required_params": ["path"],
            "optional_params": ["count"]
        }]
    })
    .to_string();
    cell
}

fn key(namespace: &str, name: &str) -> StateKey {
    match StateKey::new(namespace, name) {
        Ok(key) => key,
        Err(_) => StateKey {
            namespace: namespace.to_string(),
            name: name.to_string(),
        },
    }
}

fn select(snapshot: &RuntimeSnapshot, id: &str, unfinished: &[RuntimeDecision]) -> RuntimeDecision {
    match select_runtime_decision(snapshot, id, "prepared-context", unfinished) {
        Ok(decision) => decision,
        Err(_) => RuntimeDecision::new(
            "error",
            "case-1",
            OperationKey("error".to_string()),
            ToolSetView::empty(),
            OutputEnvelope::None,
        ),
    }
}
