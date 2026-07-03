use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    let task_list = cli::run(["--data", data.to_string_lossy().as_ref(), "task", "list"])?;
    assert!(task_list.contains("task 1 closed"));
    assert!(cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "task",
        "show",
        "1"
    ])?
    .contains("task 1 Closed"));
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
