use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::{Event, EventKind, TaskState};
use lkjagent_store::plan_access::{
    attach_answer, deliver_next, enqueue, insert_task, set_task_state,
};
use lkjagent_store::plan_inspect::application_tables;
use lkjagent_store::plan_schema::{setup, APPLICATION_TABLES};
use lkjagent_store::plan_turn::{commit_commands, events, orphan_exchanges};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn fresh_schema_has_declared_tables() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let tables = application_tables(&conn)?;
    for table in APPLICATION_TABLES {
        assert!(tables.contains(*table), "missing {table}");
    }
    assert_eq!(tables.len(), APPLICATION_TABLES.len());
    Ok(())
}

#[test]
fn fifo_queue_and_waiting_answer_routing() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let first = enqueue(&conn, "first", "now")?;
    let second = enqueue(&conn, "second", "now")?;
    let snapshot = instantiate(7, "What is here?");
    insert_task(&conn, &snapshot.task, Some(first), "now")?;
    let delivered = deliver_next(&conn, 7, "later")?;
    assert!(matches!(delivered, Some(row) if row.id == first));
    let delivered = deliver_next(&conn, 8, "later")?;
    assert!(matches!(delivered, Some(row) if row.id == second));
    set_task_state(&conn, 7, TaskState::Waiting, "wait")?;
    let answer = attach_answer(&conn, 7, "answer", "answer-time")?;
    assert!(answer > second);
    Ok(())
}

#[test]
fn turn_transaction_rolls_back_uncommitted_rows() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(9, "Survey and report.");
    insert_task(&conn, &snapshot.task, None, "now")?;
    commit_commands(&mut conn, 9, &[event("one")], "t1")?;
    commit_commands(&mut conn, 9, &[event("two")], "t2")?;
    {
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT INTO events (task_id, kind, content, created_at) VALUES (9, 'notice', 'lost', 't3')",
            [],
        )?;
    }
    let rows = events(&conn)?;
    assert_eq!(rows.len(), 2);
    let orphans = orphan_exchanges(&["a".to_string(), "b".to_string()], &["a".to_string()]);
    assert_eq!(orphans.len(), 1);
    Ok(())
}

fn event(content: &str) -> Command {
    Command::RecordEvent(Event {
        kind: EventKind::Notice,
        content: content.to_string(),
    })
}
