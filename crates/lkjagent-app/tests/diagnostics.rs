use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use rusqlite::Connection;
use serde_json::Value;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn doctor_reports_separate_roots_without_creating_workspace() -> TestResult<()> {
    let data = fixture_root("doctor")?;

    let output = cli::run(["--data", data.to_string_lossy().as_ref(), "doctor"])?;

    assert!(output.contains("schema=native-18 tables=18 missing=0"));
    assert!(output.contains("endpoint: url="));
    assert!(output.contains("roots: data="));
    assert!(output.contains("workspace="));
    assert!(output.contains("workspace_present=false"));
    assert!(output.contains("matters=0 unfinished_decisions=0"));
    let connection = Connection::open(data.join("lkjagent.sqlite3"))?;
    let tables: i64 = connection.query_row(
        "SELECT count(*) FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(tables, 18);
    assert_eq!(
        connection.query_row(
            "SELECT count(*) FROM sqlite_schema WHERE name='tasks'",
            [],
            |row| row.get::<_, i64>(0)
        )?,
        0
    );
    Ok(())
}

#[test]
fn workspace_reports_rows_and_json_shape() -> TestResult<()> {
    let data = fixture_root("workspace")?;
    cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "todo",
        "Buy",
        "milk",
    ])?;

    let rebuilt = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "workspace",
        "--rebuild",
    ])?;
    assert!(rebuilt.contains("records: total=1 archived=0"));
    assert!(rebuilt.contains("artifacts: total=7"));
    assert!(rebuilt.contains("indexes: files=7"));
    assert!(rebuilt.contains("present: true"));
    let workspace = lkjagent_app::config::workspace_root(&data)?;
    let open_todos = fs::read_to_string(workspace.join("indexes/open-todos.md"))?;
    assert!(open_todos.contains("Buy milk"));

    let json = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "workspace",
        "--json",
    ])?;
    let value: Value = serde_json::from_str(&json)?;
    assert_eq!(value["records"]["total"], 1);
    assert_eq!(value["artifacts"], 7);
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let parent = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
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
