use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, StepState, TaskState};
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn continuation_turn_attaches_to_open_matter() -> TestResult<()> {
    let data = fixture_root("matter-continuation")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let snapshot = instantiate(1, "create an artifact report");
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    enqueue(&conn, "also add this evidence to this matter", "later")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 0)?;
    assert_eq!(endpoint.index, 0);
    assert!(snapshot.task.brief.contains("owner_update="));
    assert!(snapshot.steps[0].inputs.contains("owner_update="));

    let show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "show",
        "1",
    ])?;
    assert!(show.contains("route=existing_matter durability=matter_update"));
    assert!(show.contains("matter=1"));
    Ok(())
}

#[test]
fn english_inspect_goal_requires_provider_work() -> TestResult<()> {
    let data = fixture_root("queue-inspect-goal")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(
        &conn,
        "Inspect project-orbit and cite current source.",
        "now",
    )?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<plan>\nrespond | Gather source evidence before answering\n</plan>".to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(endpoint.index, 1);
    assert_ne!(snapshot.task.state, TaskState::Closed);
    assert!(!snapshot.task.summary.starts_with("inspection:"));
    Ok(())
}

#[test]
fn six_useful_model_decisions_continue_without_a_turn_cap() -> TestResult<()> {
    let data = fixture_root("six-useful-decisions")?;
    let mut snapshot = instantiate(1, "Answer this matter in six useful stages");
    let base = snapshot.steps[0].clone();
    snapshot.steps = (0..6)
        .map(|index| {
            let mut step = base.clone();
            step.id = index + 1;
            step.ordinal = index as u32 + 1;
            step.kind = StepKind::Respond;
            step.title = format!("Useful stage {}", index + 1);
            step.instruction = format!("Return useful stage {}", index + 1);
            step.state = if index == 0 {
                StepState::Active
            } else {
                StepState::Pending
            };
            step.attempts_used = 0;
            step.actions_used = 0;
            step
        })
        .collect();
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: (1..=6)
            .map(|index| format!("<final><message>useful stage {index}</message></final>"))
            .collect(),
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 12)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(endpoint.index, 6);
    let (decisions, exchanges, blockers): (i64, i64, i64) = conn.query_row(
        "SELECT
        (SELECT COUNT(*) FROM runtime_decisions WHERE operation_key LIKE 'model.call/%'),
        (SELECT COUNT(*) FROM provider_exchanges WHERE finished_at IS NOT NULL),
        (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked')",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!((decisions, exchanges, blockers), (6, 6, 0));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
