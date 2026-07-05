use lkjagent_core::runtime_event::{apply_patch, reduce_event, RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::RuntimeSnapshot;
use lkjagent_core::runtime_state_edge::{
    active_edges, sorted_edges, EdgeEvidenceRef, StateEdge, StateEdgeRelation, StateEdgeStatus,
    StateRef,
};

#[test]
fn unknown_refs_round_trip_and_sort_deterministically() {
    let late = edge("edge-2", "record", "rec-b", "state", "todo:open/b");
    let early = edge("edge-1", "record", "rec-a", "state", "todo:open/a")
        .with_evidence(vec![EdgeEvidenceRef::new("owner", "msg-1", "fp")]);

    let sorted = sorted_edges(vec![late.clone(), early.clone()]);

    assert_eq!(sorted, vec![early, late]);
    assert_eq!(sorted[0].from_ref.label(), "record:rec-a");
    assert_eq!(sorted[0].to_ref.label(), "state:todo:open/a");
}

#[test]
fn reducer_adds_and_suppresses_state_edges() {
    let initial = RuntimeSnapshot::empty("case-1");
    let upsert = event(
        "event-1",
        RuntimeEventPayload::UpsertEdge(Box::new(edge(
            "edge-1",
            "state",
            "todo:open/a",
            "record",
            "rec-a",
        ))),
    );
    let after_upsert = apply_patch(&initial, &reduce_event(&initial, &upsert));
    let suppress = event(
        "event-2",
        RuntimeEventPayload::SuppressEdge {
            edge_id: "edge-1".to_string(),
            reason: "owner changed record".to_string(),
        },
    );

    let after_suppress = apply_patch(&after_upsert, &reduce_event(&after_upsert, &suppress));
    let edge = after_suppress.edges.get("edge-1");

    assert!(after_upsert
        .active_edges()
        .iter()
        .any(|item| item.id == "edge-1"));
    assert!(after_suppress.active_edges().is_empty());
    assert_eq!(
        edge.map(|item| item.status),
        Some(StateEdgeStatus::Suppressed)
    );
    assert_eq!(
        edge.and_then(|item| item.suppression_reason.as_deref()),
        Some("owner changed record")
    );
}

#[test]
fn active_edge_filter_keeps_suppressed_edges_out_of_selection() {
    let visible = edge("edge-1", "state", "todo:open/a", "record", "rec-a");
    let hidden = edge("edge-2", "state", "todo:open/b", "record", "rec-b").suppress("stale");

    assert_eq!(active_edges(vec![hidden, visible])[0].id, "edge-1");
}

fn edge(id: &str, from_kind: &str, from_id: &str, to_kind: &str, to_id: &str) -> StateEdge {
    StateEdge::active(
        id,
        "case-1",
        StateRef::new(from_kind, from_id),
        StateRef::new(to_kind, to_id),
        StateEdgeRelation::references(),
        "event-1",
    )
    .with_reason("test relation")
}

fn event(id: &str, payload: RuntimeEventPayload) -> RuntimeEvent {
    RuntimeEvent {
        id: id.to_string(),
        case_id: "case-1".to_string(),
        kind: "state.edge".to_string(),
        payload,
        source: "test".to_string(),
        created_at: "now".to_string(),
        decision_id: None,
    }
}
