use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[test]
fn selector_reuses_unfinished_decision_before_new_work() {
    let snapshot = snapshot_with(cell("model", "call/7"));
    let unfinished = RuntimeDecision::new(
        "decision-existing",
        "case-1",
        OperationKey("model.call/7".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );

    let selected = select(&snapshot, "decision-new", &[unfinished]);

    assert_eq!(selected.id, "decision-existing");
}

#[test]
fn selector_prefers_bridge_next_work_and_preserves_tool_view() {
    let snapshot = snapshot_with(bridge_cell());

    let selected = select(&snapshot, "decision-1", &[]);

    assert_eq!(selected.operation.0, "model.call/42");
    assert_eq!(selected.expected_envelope, OutputEnvelope::Action);
    assert_eq!(selected.model_budget_tokens, Some(512));
    assert_eq!(selected.tool_view.tool_names(), vec!["fs.read"]);
    assert!(!selected.snapshot_fingerprint.is_empty());
    assert_eq!(
        selected.snapshot_fingerprint,
        selected.state_vector_fingerprint
    );
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

fn snapshot_with(cell: StateCell) -> RuntimeSnapshot {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot.cells.insert(cell.key.clone(), cell);
    snapshot
}

fn cell(namespace: &str, name: &str) -> StateCell {
    StateCell::active(key(namespace, name), "event-1")
}

fn bridge_cell() -> StateCell {
    let mut cell = cell("runtime", "next-work");
    cell.payload_schema = "plan-bridge.next-work.v1".to_string();
    cell.payload_json = serde_json::json!({
        "operation": "model.call",
        "step_id": 42,
        "expected_envelope": "Action",
        "model_budget_tokens": 512,
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
    match select_runtime_decision(snapshot, id, unfinished) {
        Ok(decision) => decision,
        Err(_) => RuntimeDecision::new(
            "error",
            "case-1",
            OperationKey("error".to_string()),
            ToolSetView::new(vec![ToolViewEntry::new("error", "error")]),
            OutputEnvelope::None,
        ),
    }
}
