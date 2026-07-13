use lkjagent_core::runtime_event::{reduce, ReduceFault, RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_operation::{BlockReason, RuntimePolicy, Selection, WakeCondition};
use lkjagent_core::runtime_selector::select;
use lkjagent_core::runtime_state::{
    CurrentTime, RuntimeSnapshot, RuntimeState, StateCell, StateKey, StateStatus,
};

type TestResult = Result<(), String>;
const NOW: &str = "2026-07-12T10:00:00Z";

#[test]
fn reducer_validates_identity_and_sequence() -> TestResult {
    let snapshot = RuntimeSnapshot::empty("matter-1");
    let wrong = event("event-1", "other", cell("matter", "opened", "event-1")?);
    assert!(matches!(
        reduce(snapshot.clone(), wrong, now()),
        Err(ReduceFault::MatterIdentity { .. })
    ));
    let skipped = event("event-2", "matter-1", cell("matter", "opened", "event-2")?);
    assert!(matches!(
        reduce(snapshot, skipped, now()),
        Err(ReduceFault::CausalSequence { expected: 1, .. })
    ));
    Ok(())
}

#[test]
fn reducer_preserves_unknown_and_invalidates_old_revision() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("matter-1");
    let mut check = cell("check", "current-passed", "event-1")?;
    check.payload_json = r#"{"revision":"old"}"#.into();
    let unknown = cell("future", "opaque", "event-1")?;
    snapshot.cells.insert(check.key.clone(), check);
    snapshot.cells.insert(unknown.key.clone(), unknown.clone());
    let mut source = cell("source", "current", "event-2")?;
    source.payload_json = r#"{"revision":"new"}"#.into();
    let state = reduce(snapshot, event("event-2", "matter-1", source), now())
        .map_err(|fault| format!("{fault:?}"))?;
    assert_eq!(state.snapshot.cells.get(&unknown.key), Some(&unknown));
    assert_eq!(
        state
            .snapshot
            .cells
            .get(&key("check", "current-passed")?)
            .map(|cell| cell.status),
        Some(StateStatus::Suppressed)
    );
    Ok(())
}

#[test]
fn direct_transitions_do_not_add_model_review() -> TestResult {
    assert_decision(
        state_with(cell("matter", "opened", "event-1")?),
        "orient.matter",
        true,
    );
    assert_decision(
        state_with(cell("source", "current", "event-1")?),
        "modify.source",
        true,
    );
    assert_decision(
        state_with(cell("edit", "committed", "event-1")?),
        "check.run/current",
        false,
    );
    assert_decision(
        state_with(cell("check", "failed", "event-1")?),
        "recovery.modify",
        true,
    );
    assert_decision(
        state_with(cell("report", "pending", "event-1")?),
        "modify.report",
        true,
    );
    Ok(())
}

#[test]
fn missing_fact_waits_on_owner_and_cooldown_has_typed_wake() -> TestResult {
    let waiting = select(
        state_with(cell("need", "owner-fact", "event-1")?),
        RuntimePolicy::default(),
        now(),
    );
    assert!(
        matches!(waiting, Selection::Wait(wait) if matches!(wait.wake, WakeCondition::OwnerInput { .. }))
    );
    let mut source = cell("source", "current", "event-1")?;
    source.cooldown_until = Some("2026-07-12T10:00:01Z".into());
    let cooling = select(state_with(source), RuntimePolicy::default(), now());
    assert!(
        matches!(cooling, Selection::Wait(wait) if matches!(wait.wake, WakeCondition::Time { .. }))
    );
    Ok(())
}

#[test]
fn current_checks_respond_without_tools_and_guards_close() -> TestResult {
    let responding = select(
        state_with(cell("check", "current-passed", "event-1")?),
        RuntimePolicy::default(),
        now(),
    );
    assert!(
        matches!(responding, Selection::Decision(spec) if spec.operation_key == "respond.final" && spec.tool_view.entries.is_empty())
    );
    let mut closed = RuntimeSnapshot::empty("matter-1");
    for cell in [
        cell("check", "current-passed", "event-1")?,
        cell("response", "final-persisted", "event-2")?,
    ] {
        closed.cells.insert(cell.key.clone(), cell);
    }
    assert_eq!(
        select(
            RuntimeState::from_snapshot(closed),
            RuntimePolicy::default(),
            now()
        ),
        Selection::Idle
    );
    Ok(())
}

#[test]
fn unsettled_effects_and_stale_checks_prevent_closure() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("matter-1");
    for cell in [
        cell("response", "final-persisted", "event-1")?,
        cell("check", "current-passed", "event-2")?,
        cell("effect", "pending", "event-3")?,
    ] {
        snapshot.cells.insert(cell.key.clone(), cell);
    }
    assert!(
        matches!(select(RuntimeState::from_snapshot(snapshot), RuntimePolicy::default(), now()),
        Selection::Block(block) if matches!(block.reason, BlockReason::UnsettledEffects(_)))
    );
    Ok(())
}

#[test]
fn conflicts_and_exhausted_equal_progress_block() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("matter-1");
    for name in ["a", "b"] {
        let mut source = cell("source", name, "event-1")?;
        source.conflict_group = Some("revision".into());
        snapshot.cells.insert(source.key.clone(), source);
    }
    assert!(
        matches!(select(RuntimeState::from_snapshot(snapshot), RuntimePolicy::default(), now()),
        Selection::Block(block) if matches!(block.reason, BlockReason::Conflict(_)))
    );
    let policy = RuntimePolicy {
        prior_progress_fingerprint: Some("same".into()),
        current_progress_fingerprint: Some("same".into()),
        recovery_attempt: 5,
        ..RuntimePolicy::default()
    };
    assert!(
        matches!(select(state_with(cell("source", "current", "event-1")?), policy, now()),
        Selection::Block(block) if block.reason == BlockReason::Stasis)
    );
    Ok(())
}

fn assert_decision(state: RuntimeState, operation: &str, model: bool) {
    assert!(matches!(select(state, RuntimePolicy::default(), now()),
        Selection::Decision(spec) if spec.operation_key == operation && spec.model_required == model));
}
fn state_with(cell: StateCell) -> RuntimeState {
    let mut snapshot = RuntimeSnapshot::empty("matter-1");
    snapshot.cells.insert(cell.key.clone(), cell);
    RuntimeState::from_snapshot(snapshot)
}
fn event(id: &str, matter: &str, cell: StateCell) -> RuntimeEvent {
    RuntimeEvent {
        id: id.into(),
        case_id: matter.into(),
        kind: "state".into(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "test".into(),
        created_at: NOW.into(),
        decision_id: None,
    }
}
fn cell(namespace: &str, name: &str, event: &str) -> Result<StateCell, String> {
    Ok(StateCell::active(key(namespace, name)?, event))
}
fn key(namespace: &str, name: &str) -> Result<StateKey, String> {
    StateKey::new(namespace, name).map_err(|error| error.message)
}
fn now() -> CurrentTime {
    CurrentTime::new(NOW)
}
