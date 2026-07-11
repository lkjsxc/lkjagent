use lkjagent_core::runtime_candidate::selected_candidate_at;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};
use lkjagent_core::runtime_state_edge::{StateEdge, StateEdgeRelation, StateRef};

type TestResult = Result<(), String>;

#[test]
fn future_cell_waits_without_consuming_it_and_due_cell_runs() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let mut future = model_cell("future")?;
    future.cooldown_until = Some("2026-07-11T10:00:00.050Z".to_string());
    snapshot.cells.insert(future.key.clone(), future);

    let waiting = selected_candidate_at(&snapshot, "2026-07-11T10:00:00.049Z");
    assert_eq!(waiting.operation.key, "runtime.wait");
    assert_eq!(waiting.state_key, None);
    assert!(snapshot
        .active_cells()
        .iter()
        .any(|cell| cell.key.as_label() == "model:future"));

    let due = selected_candidate_at(&snapshot, "2026-07-11T10:00:00.050Z");
    assert_eq!(due.operation.key, "model.call/future");
    assert_eq!(
        due.state_key.map(|key| key.as_label()).as_deref(),
        Some("model:future")
    );
    Ok(())
}

#[test]
fn future_cell_does_not_hide_runnable_work() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let mut future = model_cell("future")?;
    future.cooldown_until = Some("2026-07-11T10:00:02Z".to_string());
    let current = model_cell("current")?;
    snapshot.cells.insert(future.key.clone(), future);
    snapshot.cells.insert(current.key.clone(), current);

    let selected = selected_candidate_at(&snapshot, "2026-07-11T10:00:00Z");
    assert_eq!(selected.operation.key, "model.call/current");
    Ok(())
}

#[test]
fn malformed_cooldown_fails_closed_as_visible_blocker() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let mut malformed = model_cell("malformed")?;
    malformed.cooldown_until = Some("not-an-instant".to_string());
    snapshot.cells.insert(malformed.key.clone(), malformed);
    let selected = selected_candidate_at(&snapshot, "2026-07-11T10:00:00Z");
    assert_eq!(selected.operation.key, "completion.blocked");
    Ok(())
}

#[test]
fn entirely_edge_blocked_state_selects_visible_blocker() -> TestResult {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    let model = model_cell("blocked")?;
    snapshot.cells.insert(model.key.clone(), model);
    snapshot.edges.insert(
        "edge-1".to_string(),
        StateEdge::active(
            "edge-1",
            "case-1",
            StateRef::new("state", "safety:conflict"),
            StateRef::new("state", "model:blocked"),
            StateEdgeRelation::blocks(),
            "event-1",
        ),
    );

    let selected = selected_candidate_at(&snapshot, "2026-07-11T10:00:00Z");
    assert_eq!(selected.operation.key, "completion.blocked");
    assert_eq!(selected.state_key, None);
    Ok(())
}

fn model_cell(name: &str) -> Result<StateCell, String> {
    let key = StateKey::new("model", name).map_err(|error| error.message)?;
    let mut cell = StateCell::active(key, "event-1");
    cell.payload_json = serde_json::json!({
        "operation_key": format!("model.call/{name}"),
        "expected_envelope": "Message"
    })
    .to_string();
    Ok(cell)
}
