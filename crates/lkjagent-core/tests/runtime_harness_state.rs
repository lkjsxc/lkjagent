use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_core::runtime_harness_state::{derive_harness_state, RuntimeHarnessState};
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[test]
fn derives_harness_state_from_decision_authority() {
    assert_eq!(
        derive_harness_state(
            Some("case:owner-intake"),
            "owner.intake",
            OutputEnvelope::None,
            "commit-or-recover"
        ),
        RuntimeHarnessState::Intake
    );
    assert_eq!(
        derive_harness_state(
            Some("recovery:parse/1"),
            "model.call/1",
            OutputEnvelope::Action,
            "retry-same-decision"
        ),
        RuntimeHarnessState::Recover
    );
    assert_eq!(
        derive_harness_state(
            Some("todo:open/1"),
            "todo.review/open/1",
            OutputEnvelope::None,
            "commit-or-recover"
        ),
        RuntimeHarnessState::Record
    );
    assert_eq!(
        derive_harness_state(
            Some("index:workspace"),
            "index.rebuild/workspace",
            OutputEnvelope::None,
            "commit-or-recover"
        ),
        RuntimeHarnessState::Maintain
    );
    assert_eq!(
        derive_harness_state(None, "runtime.idle", OutputEnvelope::None, "none"),
        RuntimeHarnessState::Idle
    );
}

#[test]
fn selector_stores_harness_state_on_persisted_decision() -> Result<(), String> {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let key = StateKey::new("work", "model/7").map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, "event-1");
    cell.payload_json = serde_json::json!({
        "operation_key": "model.call/7",
        "expected_envelope": "Action"
    })
    .to_string();
    snapshot.cells.insert(cell.key.clone(), cell);

    let decision = select_runtime_decision(&snapshot, "decision-1", "ctx-1", &[])
        .map_err(|error| error.message)?;

    assert_eq!(decision.selected_state_key.as_deref(), Some("work:model/7"));
    assert_eq!(decision.harness_state, RuntimeHarnessState::Act);
    Ok(())
}

#[test]
fn unfinished_decision_keeps_recorded_harness_state() -> Result<(), String> {
    let snapshot = RuntimeSnapshot::empty("case-1");
    let mut pending = RuntimeDecision::new(
        "decision-existing",
        "case-1",
        OperationKey("model.call/old".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    pending.harness_state = RuntimeHarnessState::Recover;

    let selected = select_runtime_decision(&snapshot, "decision-new", "ctx-new", &[pending])
        .map_err(|error| error.message)?;

    assert_eq!(selected.id, "decision-existing");
    assert_eq!(selected.harness_state, RuntimeHarnessState::Recover);
    Ok(())
}
