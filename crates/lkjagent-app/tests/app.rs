use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_app::status::render_status;
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn help_matches_documented_command_tree() -> TestResult<()> {
    let output = cli::run(["help"])?;
    assert!(output.contains("send TEXT [--new]"));
    assert!(output.contains("task list | task show ID"));
    Ok(())
}

#[test]
fn fake_endpoint_task_closes_and_resumes_from_store() -> TestResult<()> {
    let data = fixture_root("daemon")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);

    let mut first = ScriptedEndpoint {
        outputs: vec![
            "<message>wrong</message>".to_string(),
            "<finish>done exploring</finish>".to_string(),
        ],
        index: 0,
    };
    let partial = run_until_idle(&data, &mut first, 2)?;
    assert_eq!(partial.task.state, TaskState::Open);

    let mut second = ScriptedEndpoint {
        outputs: vec!["<message>Survey complete.</message>".to_string()],
        index: 0,
    };
    let closed = run_until_idle(&data, &mut second, 4)?;
    assert_eq!(closed.task.state, TaskState::Closed);
    assert!(render_status(&closed).contains("daemon: stopped"));
    Ok(())
}

#[test]
fn status_snapshot_contains_documented_fields() -> TestResult<()> {
    let data = fixture_root("status")?;
    let sent = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "send",
        "What",
        "now?",
    ])?;
    assert!(sent.contains("queue:"));
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("daemon:"));
    assert!(status.contains("tokens:"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
