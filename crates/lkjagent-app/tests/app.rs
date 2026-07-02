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

#[test]
fn simple_templates_close_with_fake_endpoint() -> TestResult<()> {
    run_scripted(
        "generic",
        "Survey the repository and report.",
        vec!["<finish>found facts</finish>", "<message>done</message>"],
        None,
    )?;
    run_scripted(
        "question",
        "What is an agent?",
        vec!["<message>An agent follows a loop.</message>"],
        None,
    )?;
    run_scripted(
        "journal",
        "Add a journal note about the release.",
        vec![
            "<content># Release\n\nShipped notes.</content>",
            "<message>journal updated</message>",
        ],
        Some("journal/today.md"),
    )?;
    run_scripted(
        "filework",
        "Write notes/out.md with setup notes.",
        vec![
            "<plan>write notes/out.md | draft | words=1\nrespond | summarize file work</plan>",
            "<content>setup notes</content>",
            "<message>wrote notes</message>",
        ],
        Some("notes/out.md"),
    )?;
    Ok(())
}

#[test]
fn waiting_question_resumes_from_queued_answer() -> TestResult<()> {
    let data = fixture_root("waiting")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is in the workspace?", "now")?;
    drop(conn);
    let mut first = ScriptedEndpoint {
        outputs: vec!["<ask>Which file?</ask>".to_string()],
        index: 0,
    };
    let waiting = run_until_idle(&data, &mut first, 2)?;
    assert_eq!(waiting.task.state, TaskState::Waiting);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "README.md", "now")?;
    drop(conn);
    let mut second = ScriptedEndpoint {
        outputs: vec![
            "<finish>README.md observed</finish>".to_string(),
            "<message>It exists.</message>".to_string(),
        ],
        index: 0,
    };
    let closed = run_until_idle(&data, &mut second, 4)?;
    assert_eq!(closed.task.state, TaskState::Closed);
    assert!(closed
        .events
        .iter()
        .any(|event| event.content == "README.md"));
    Ok(())
}

fn run_scripted(
    name: &str,
    objective: &str,
    outputs: Vec<&str>,
    expected_file: Option<&str>,
) -> TestResult<()> {
    let data = fixture_root(name)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: outputs.into_iter().map(str::to_string).collect(),
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 8)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    if let Some(path) = expected_file {
        assert!(data.join("workspace").join(path).exists(), "{path}");
    }
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
