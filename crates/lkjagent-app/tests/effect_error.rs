use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::{AttemptOutcome, StepState};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn failed_write_effect_records_error_without_done_state() -> TestResult<()> {
    let data = fixture_root("effect-error")?;
    fs::create_dir_all(data.join("workspace"))?;
    fs::write(data.join("workspace/journal"), "not a directory")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Add a journal note about the release.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<content>release note</content>".to_string()],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_ne!(snapshot.steps[0].state, StepState::Done);
    assert!(snapshot
        .attempts
        .iter()
        .any(|attempt| attempt.outcome == AttemptOutcome::EffectError));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let state: String = conn.query_row("SELECT state FROM steps LIMIT 1", [], |row| row.get(0))?;
    assert_ne!(state, "done");
    let outcome: String =
        conn.query_row("SELECT outcome FROM attempts LIMIT 1", [], |row| row.get(0))?;
    assert_eq!(outcome, "effecterror");
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-effect-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
