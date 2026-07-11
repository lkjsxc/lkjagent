use std::fs;

use lkjagent_app::clock::FixedClock;
use lkjagent_app::daemon::{run_until_idle_with_clock, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::TaskState;
use lkjagent_core::runtime_event::{RuntimeEvent, RuntimeEventPayload};
use lkjagent_core::runtime_state::{StateCell, StateKey, StateStatus};
use lkjagent_store::event_rows::append_and_apply_event;
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{hydrate_snapshot, insert_case};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn future_recovery_wait_makes_no_decision_or_model_call_until_due() -> TestResult<()> {
    let data = std::env::temp_dir().join(format!("lkjagent-runtime-wait-{}", std::process::id()));
    if data.exists() {
        fs::remove_dir_all(&data)?;
    }
    fs::create_dir_all(&data)?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What is an agent?");
    insert_task(&conn, &snapshot.task, None, "before")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "before")?;
    }
    tx.commit()?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let step = snapshot.steps[0].id;
    let key = StateKey::new("model", step.to_string())
        .map_err(|error| std::io::Error::other(error.message))?;
    let mut cell = StateCell::active(key.clone(), "event-wait");
    cell.payload_json = serde_json::json!({
        "operation_key": format!("model.call/{step}"), "expected_envelope": "Message"
    })
    .to_string();
    cell.cooldown_until = Some("2026-07-11T10:00:01Z".to_string());
    let event = RuntimeEvent {
        id: "event-wait".to_string(),
        case_id: "1".to_string(),
        kind: "wait.test".to_string(),
        payload: RuntimeEventPayload::UpsertCell(Box::new(cell)),
        source: "test".to_string(),
        created_at: "before".to_string(),
        decision_id: None,
    };
    append_and_apply_event(&conn, &event)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>Agent answer.</message>".to_string()],
        index: 0,
    };
    let mut before = FixedClock::new("2026-07-11T10:00:00Z");
    let waiting = run_until_idle_with_clock(&data, &mut endpoint, 3, &mut before)?;
    assert_eq!(waiting.task.state, TaskState::Open);
    assert_eq!(endpoint.index, 0);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let decisions: i64 = conn.query_row("SELECT COUNT(*) FROM runtime_decisions", [], |row| {
        row.get(0)
    })?;
    assert_eq!(decisions, 0);
    let state = hydrate_snapshot(&conn, "1")?;
    assert_eq!(state.cells[&key].status, StateStatus::Active);
    drop(conn);

    let mut due = FixedClock::new("2026-07-11T10:00:01Z");
    let settled = run_until_idle_with_clock(&data, &mut endpoint, 2, &mut due)?;
    assert_ne!(settled.task.state, TaskState::Open);
    assert_eq!(endpoint.index, 1);
    Ok(())
}
