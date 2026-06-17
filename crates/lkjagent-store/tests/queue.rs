mod support;

use lkjagent_store::events::read_events;
use lkjagent_store::queue::{delete, deliver_next, edit, enqueue, list, redeliver};
use support::{memory_store, TestResult};

#[test]
fn queue_delivery_is_exactly_once_and_writes_owner_event() -> TestResult<()> {
    let mut conn = memory_store()?;
    let first = enqueue(&mut conn, "first", "owner-send", "2026-01-01T00:00:00Z")?;
    let second = enqueue(&mut conn, "second", "owner-send", "2026-01-01T00:00:01Z")?;

    let delivered = deliver_next(&mut conn, 7, 3, "2026-01-01T00:00:02Z")?;
    assert_eq!(delivered.map(|row| row.id), Some(first));

    let delivered = deliver_next(&mut conn, 8, 3, "2026-01-01T00:00:03Z")?;
    assert_eq!(delivered.map(|row| row.id), Some(second));

    let delivered = deliver_next(&mut conn, 9, 3, "2026-01-01T00:00:04Z")?;
    assert_eq!(delivered, None);

    let events = read_events(&conn)?;
    assert_eq!(
        events.iter().filter(|event| event.kind == "owner").count(),
        2
    );
    Ok(())
}

#[test]
fn queue_mutations_are_transactional_tombstones_and_redeliveries() -> TestResult<()> {
    let mut conn = memory_store()?;
    let id = enqueue(&mut conn, "draft", "owner-send", "2026-01-01T00:00:00Z")?;
    edit(&mut conn, id, "edited", "fix typo", "2026-01-01T00:00:01Z")?;
    delete(&mut conn, id, "cancel", "2026-01-01T00:00:02Z")?;
    let redelivered = redeliver(
        &mut conn,
        id,
        Some("again"),
        "retry",
        "2026-01-01T00:00:03Z",
    )?;

    let rows = list(&conn)?;
    assert_eq!(
        rows.iter()
            .find(|row| row.id == id)
            .map(|row| row.status.as_str()),
        Some("deleted")
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == redelivered)
            .and_then(|row| row.source_queue_id),
        Some(id)
    );

    let events = read_events(&conn)?;
    assert!(events
        .iter()
        .any(|event| event.content.contains("operation=redeliver")));
    Ok(())
}
