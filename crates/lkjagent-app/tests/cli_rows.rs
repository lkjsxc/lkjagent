use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn status_reports_stale_lease_rows() -> TestResult<()> {
    let data = fixture_root("lease")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute(
        "INSERT INTO config (key, value) VALUES
         ('daemon.lock.owner', 'pid:old'),
         ('daemon.lock.heartbeat', 'unix:1')",
        [],
    )?;
    drop(conn);

    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;

    assert!(status.contains("lease: stale owner=pid:old heartbeat=unix:1"));
    Ok(())
}

#[test]
fn cli_inspection_reads_store_rows() -> TestResult<()> {
    let data = fixture_root("cli")?;
    let sent = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "send",
        "--new",
        "What",
        "now?",
    ])?;
    assert!(sent.contains("new=true"));
    assert!(
        cli::run(["--data", data.to_string_lossy().as_ref(), "queue", "list"])?
            .contains("force_new=true")
    );
    assert!(cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "queue",
        "show",
        "1"
    ])?
    .contains("content=What now?"));

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    enqueue(&conn, "What is an agent?", "now")?;
    conn.execute(
        "INSERT INTO memory (topic, content, created_at) VALUES ('agent', 'row memory', 'now')",
        [],
    )?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>done</message>".to_string()],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 3)?;
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("state: active="));
    assert!(status.contains("decision: case-1-decision-"));
    assert!(status.contains("exchanges: 1"));
    let task_list = cli::run(["--data", data.to_string_lossy().as_ref(), "task", "list"])?;
    assert!(task_list.contains("task 1 closed"));
    let task_show = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "task",
        "show",
        "1",
    ])?;
    assert!(task_show.contains("task 1 Closed"));
    assert!(task_show.contains("state: active="));
    assert!(task_show.contains("decisions:"));
    assert!(task_show.contains("prompt_frames: 1"));
    assert!(task_show.contains("checks: total=0"));
    assert!(task_show.contains("exchanges: 1"));
    let watch = cli::run(["--data", data.to_string_lossy().as_ref(), "watch"])?;
    assert!(watch.contains("== status =="));
    assert!(watch.contains("== recent events =="));
    assert!(watch.contains("task 1 Closed"));
    assert!(watch.contains("proof: prompt_frames="));
    assert!(watch.contains("exchanges=1"));
    assert!(cli::run(["--data", data.to_string_lossy().as_ref(), "log"])?.contains("taskclosed"));
    assert!(
        cli::run(["--data", data.to_string_lossy().as_ref(), "memory", "row"])?
            .contains("row memory")
    );
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-cli-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
