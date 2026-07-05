use lkjagent_core::runtime_state_edge::{StateEdge, StateEdgeRelation, StateEdgeStatus, StateRef};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_edge_rows::{insert_state_edge, state_edges, suppress_state_edge};
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn state_edges_round_trip_and_suppress() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    insert_case(&conn, "case-1", "Persist edge.", "t0")?;
    let edge = StateEdge::active(
        "edge-1",
        "case-1",
        StateRef::new("record", "rec-a"),
        StateRef::new("state", "todo:open/a"),
        StateEdgeRelation::references(),
        "event-1",
    )
    .with_reason("record creates state");

    insert_state_edge(&conn, Some("case-1"), &edge)?;
    assert_eq!(state_edges(&conn, "case-1")?, vec![edge]);

    assert_eq!(suppress_state_edge(&conn, "edge-1", "stale record")?, 1);
    assert!(state_edges(&conn, "case-1")?.is_empty());
    let status: String = conn.query_row(
        "SELECT status FROM state_edges WHERE id = 'edge-1'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(status, format!("{:?}", StateEdgeStatus::Suppressed));
    Ok(())
}
