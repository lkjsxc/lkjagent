use lkjagent_core::runtime_graph_query::{blockers, conflicts, lineage, stale_dependencies};
use lkjagent_core::runtime_state::{RuntimeSnapshot, StateCell, StateKey, StateStatus};
use lkjagent_core::runtime_state_edge::{StateEdge, StateEdgeRelation, StateRef};

#[test]
fn graph_queries_report_blockers_conflicts_stale_and_lineage() -> Result<(), String> {
    let target = key("task", "write")?;
    let done = key("record", "old")?;
    let mut snapshot = RuntimeSnapshot::empty("case-1");
    snapshot
        .cells
        .insert(target.clone(), StateCell::active(target.clone(), "event-1"));
    let mut inactive = StateCell::active(done.clone(), "event-2");
    inactive.status = StateStatus::Resolved;
    snapshot.cells.insert(done.clone(), inactive);
    add(
        &mut snapshot,
        "edge-block",
        StateRef::new("state", "policy:approval"),
        StateRef::new("state", target.as_label()),
        StateEdgeRelation::blocks(),
    );
    add(
        &mut snapshot,
        "edge-stale",
        StateRef::new("state", target.as_label()),
        StateRef::new("state", done.as_label()),
        StateEdgeRelation::depends_on(),
    );
    add(
        &mut snapshot,
        "edge-conflict",
        StateRef::new("context", "a"),
        StateRef::new("context", "b"),
        StateEdgeRelation::conflicts_with(),
    );
    add(
        &mut snapshot,
        "edge-lineage",
        StateRef::new("artifact", "unit-1"),
        StateRef::new("artifact", "manifest-1"),
        StateEdgeRelation::derived_from(),
    );

    assert_eq!(blockers(&snapshot, &target.as_label()).len(), 1);
    assert_eq!(stale_dependencies(&snapshot).len(), 1);
    assert_eq!(conflicts(&snapshot).len(), 1);
    assert_eq!(
        lineage(&snapshot, &StateRef::new("artifact", "unit-1")).len(),
        1
    );
    Ok(())
}

fn key(namespace: &str, name: &str) -> Result<StateKey, String> {
    StateKey::new(namespace, name).map_err(|error| error.message)
}

fn add(
    snapshot: &mut RuntimeSnapshot,
    id: &str,
    from: StateRef,
    to: StateRef,
    relation: StateEdgeRelation,
) {
    let edge = StateEdge::active(id, "case-1", from, to, relation, "event");
    snapshot.edges.insert(id.to_string(), edge);
}
