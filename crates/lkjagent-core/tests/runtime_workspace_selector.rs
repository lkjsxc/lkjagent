use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_core::runtime_selector::select_runtime_decision;
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey};

#[test]
fn selector_maps_workspace_record_families() -> Result<(), String> {
    for case in [
        (
            "todo",
            "open/rec_1",
            "todo.review/open/rec_1",
            "todo:open/rec_1",
        ),
        (
            "finance",
            "review/rec_3",
            "finance.review/review/rec_3",
            "finance:review/rec_3",
        ),
    ] {
        let selected = select(snapshot_with(cell(case.0, case.1)?))?;
        assert_eq!(selected.operation.0, case.2);
        assert_eq!(
            selected.evidence_requirements,
            vec![format!("selector:{}", case.0), case.3.to_string()]
        );
    }
    Ok(())
}

#[test]
fn selector_orders_workspace_family_priority() -> Result<(), String> {
    let mut low = cell("todo", "open/low")?;
    let mut high = cell("todo", "open/high")?;
    low.priority = 1;
    high.priority = 7;
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot.cells.insert(low.key.clone(), low);
    snapshot.cells.insert(high.key.clone(), high);

    let selected = select(snapshot)?;

    assert_eq!(selected.operation.0, "todo.review/open/high");
    Ok(())
}

fn snapshot_with(cell: StateCell) -> RuntimeSnapshot {
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot.cells.insert(cell.key.clone(), cell);
    snapshot
}

fn cell(namespace: &str, name: &str) -> Result<StateCell, String> {
    let key = StateKey::new(namespace, name).map_err(|error| error.message)?;
    Ok(StateCell::active(key, "event-1"))
}

fn select(snapshot: RuntimeSnapshot) -> Result<RuntimeDecision, String> {
    select_runtime_decision(&snapshot, "decision-1", "prepared-context", &[])
        .map_err(|error| error.message)
}
