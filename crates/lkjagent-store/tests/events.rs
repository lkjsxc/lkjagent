mod support;

use lkjagent_store::events::{append_event, read_events, EventKind};
use support::{memory_store, TestResult};

#[test]
fn events_append_and_read_in_order() -> TestResult<()> {
    let conn = memory_store()?;
    append_event(
        &conn,
        Some(1),
        EventKind::Action,
        "act",
        4,
        "2026-01-01T00:00:00Z",
    )?;
    append_event(
        &conn,
        Some(1),
        EventKind::Observation,
        "obs",
        4,
        "2026-01-01T00:00:01Z",
    )?;
    let events = read_events(&conn)?;
    assert_eq!(
        events
            .iter()
            .map(|event| event.kind.as_str())
            .collect::<Vec<_>>(),
        vec!["action", "observation"]
    );
    Ok(())
}
