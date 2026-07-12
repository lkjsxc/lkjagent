use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::clock::FixedClock;
use lkjagent_app::daemon::{run_until_idle, run_until_idle_with_clock, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn help_matches_documented_command_tree() -> TestResult<()> {
    let output = cli::run(["help"])?;
    assert!(output.contains("send TEXT [--new]"));
    assert!(output.contains("workbench"));
    assert!(output.contains("matter list | matter show REF"));
    Ok(())
}

#[test]
fn non_actions_advance_recovery_without_closing_work() -> TestResult<()> {
    let data = fixture_root("daemon")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<final><message>wrong</message></final>".to_string(),
            "<final><message>done exploring</message></final>".to_string(),
        ],
        index: 0,
    };
    let recovered = run_until_idle(&data, &mut endpoint, 4)?;
    assert_eq!(recovered.task.state, TaskState::Open);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let strategies: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'recovery.failure'",
        [],
        |row| row.get(0),
    )?;
    assert!(strategies >= 2);
    Ok(())
}

#[test]
fn daemon_uses_injected_clock_for_durable_rows() -> TestResult<()> {
    let data = fixture_root("clock")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "queued")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>An agent follows checks.</message></final>".to_string()],
        index: 0,
    };
    let mut clock = FixedClock::new("fixed-utc");
    let snapshot = run_until_idle_with_clock(&data, &mut endpoint, 2, &mut clock)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (String, String) = conn.query_row(
        "SELECT created_at, updated_at FROM tasks WHERE id = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row, ("fixed-utc".to_string(), "fixed-utc".to_string()));
    let heartbeat: String = conn.query_row(
        "SELECT value FROM config WHERE key = 'daemon.lock.heartbeat'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(heartbeat, "fixed-utc");
    Ok(())
}

#[test]
fn task_intake_admits_bounded_memory_facts() -> TestResult<()> {
    let data = fixture_root("memory-admission")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    conn.execute(
        "INSERT INTO memory (topic, content, created_at)
         VALUES ('release', 'row memory fact', 'seed')",
        [],
    )?;
    enqueue(&conn, "Report release details.", "queued")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    let mut clock = FixedClock::new("fixed-utc");
    let snapshot = run_until_idle_with_clock(&data, &mut endpoint, 0, &mut clock)?;
    assert!(snapshot.task.brief.contains("memory_facts"));
    assert!(snapshot.task.brief.contains("row memory fact"));
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
fn typed_templates_close_with_fake_endpoint() -> TestResult<()> {
    run_scripted(
        "question",
        "What is an agent?",
        vec!["<final><message>An agent follows a loop.</message></final>".to_string()],
        None,
    )?;
    run_scripted(
        "journal",
        "Add a journal note about the release.",
        vec![
            "<content># Release\n\nShipped notes.</content>".to_string(),
            "<final><message>journal updated</message></final>".to_string(),
        ],
        Some("journal/today.md"),
    )?;
    run_scripted(
        "filework",
        "Write notes/out.md with setup notes.",
        vec![
            "<plan>write notes/out.md | draft | words=1</plan>".to_string(),
            "<content>setup notes</content>".to_string(),
            "<final><message>wrote notes</message></final>".to_string(),
        ],
        Some("notes/out.md"),
    )?;
    Ok(())
}

fn run_scripted(
    name: &str,
    objective: &str,
    outputs: Vec<String>,
    expected_file: Option<&str>,
) -> TestResult<()> {
    let data = fixture_root(name)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };
    let snapshot = run_until_idle(&data, &mut endpoint, 8)?;
    assert_eq!(snapshot.task.state, TaskState::Closed, "{name}");
    if let Some(path) = expected_file {
        let workspace = config_workspace(&data)?;
        assert!(workspace.join(path).exists(), "{path}");
    }
    Ok(())
}

fn config_workspace(data: &std::path::Path) -> TestResult<PathBuf> {
    Ok(lkjagent_app::config::workspace_root(data)?)
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let parent = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if parent.exists() {
        fs::remove_dir_all(&parent)?;
    }
    let data = parent.join("data");
    fs::create_dir_all(&data)?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"workspace_root\":\"../workspace\"}",
    )?;
    Ok(data)
}
