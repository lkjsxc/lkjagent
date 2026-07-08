use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::{CheckResult, Event, EventKind, StepKind, TaskState};
use lkjagent_store::memory::search_memory;
use lkjagent_store::plan_access::{
    attach_answer, deliver_next, enqueue, enqueue_with_force, insert_step_tx, insert_task,
    next_pending, set_task_state,
};
use lkjagent_store::plan_commit::commit_turn;
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
fn setup_adds_force_new_to_legacy_queue() -> TestResult<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE queue (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        INSERT INTO queue (content, state, created_at) VALUES ('old', 'pending', 'then');",
    )?;

    setup(&conn)?;
    let row = match next_pending(&conn)? {
        Some(row) => row,
        None => return Err("missing migrated queue row".into()),
    };
    let id = enqueue_with_force(&conn, "new", true, "now")?;
    let forced: i64 = conn.query_row("SELECT force_new FROM queue WHERE id = ?1", [id], |row| {
        row.get(0)
    })?;

    assert_eq!(row.content, "old");
    assert!(!row.force_new);
    assert_eq!(forced, 1);
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
fn task_closed_event_updates_task_row() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(10, "What is here?");
    insert_task(&conn, &snapshot.task, None, "now")?;
    commit_commands(
        &mut conn,
        10,
        &[Command::RecordEvent(Event {
            kind: EventKind::TaskClosed,
            content: "done".to_string(),
        })],
        "later",
    )?;
    let state: String = conn.query_row("SELECT state FROM tasks WHERE id = 10", [], |row| {
        row.get(0)
    })?;
    assert_eq!(state, "closed");
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

#[test]
fn memory_writes_are_deduplicated_and_searchable() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(12, "Remember release facts.");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let memory = Command::RecordMemory {
        topic: "probe".to_string(),
        content: "release memory survives".to_string(),
    };
    commit_turn(&mut conn, &snapshot, std::slice::from_ref(&memory), "later")?;
    commit_turn(&mut conn, &snapshot, &[memory], "again")?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM memory", [], |row| row.get(0))?;
    assert_eq!(count, 1);
    let rows = search_memory(&conn, "release", 10)?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].content, "release memory survives");
    Ok(())
}

#[test]
fn turn_commit_stores_check_params_with_step_id() -> TestResult<()> {
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let snapshot = instantiate(11, "Write notes/out.md with setup notes.");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    let step_id = snapshot
        .steps
        .iter()
        .find(|step| step.kind == StepKind::Verify)
        .map(|step| step.id)
        .unwrap_or(2);
    commit_turn(
        &mut conn,
        &snapshot,
        &[Command::RecordChecks {
            step_id,
            decision_id: Some("decision-check".to_string()),
            results: vec![CheckResult {
                name: "file_exists".to_string(),
                params: None,
                decision_id: None,
                evidence_fingerprint: Some("evidence-fp".to_string()),
                artifact_refs: Vec::new(),
                passed: true,
                measured: "true".to_string(),
            }],
        }],
        "later",
    )?;
    let row: (i64, String, String, String) = conn.query_row(
        "SELECT step_id, params_json, decision_id, evidence_fingerprint FROM check_results LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    let state_cell: (String, String, String) = conn.query_row(
        "SELECT key_label, evidence_json, payload_json FROM state_cells LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!(row.0, step_id as i64);
    assert!(row.1.contains("notes/out.md"));
    assert_eq!(row.2, "decision-check");
    assert_eq!(row.3, "evidence-fp");
    assert!(state_cell.0.starts_with("completion:check-passed/"));
    assert!(state_cell.1.contains("check_result") && state_cell.2.contains("file_exists"));
    Ok(())
}
fn event(content: &str) -> Command {
    Command::RecordEvent(Event {
        kind: EventKind::Notice,
        content: content.to_string(),
    })
}
