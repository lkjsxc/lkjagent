use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::cli;
use lkjagent_core::workspace_record::{parse_record, record_fingerprint, render_record};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn open_todos_excludes_closed_rows_and_rebuild_bytes_are_stable() -> TestResult<()> {
    let data = fixture_root("index-predicates")?;
    let data_arg = data.to_string_lossy();
    let open = add(&data_arg, "Open item")?;
    let closed = add(&data_arg, "Closed item")?;
    set_state(
        &data,
        &field(&closed, "record: ")?,
        &field(&closed, "path=")?,
        "closed",
    )?;
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;
    let index = data.join("workspace/indexes/open-todos.md");
    let first = fs::read_to_string(&index)?;
    assert!(first.contains(&field(&open, "record: ")?));
    assert!(!first.contains(&field(&closed, "record: ")?));
    assert!(!first.contains("generated_at:"));
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;
    assert_eq!(first, fs::read_to_string(index)?);
    Ok(())
}

fn add(data: &str, title: &str) -> Result<String, String> {
    cli::run([
        "--data", data, "record", "add", "todo", title, "--body", title,
    ])
}

fn set_state(data: &Path, id: &str, path: &str, state: &str) -> TestResult<()> {
    let full = data.join("workspace").join(path);
    let text = fs::read_to_string(&full)?;
    let mut parsed = parse_record(&text)?;
    parsed.state = state.to_string();
    let text = render_record(&parsed);
    fs::write(full, &text)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let original = record(&conn, id)?.ok_or("record missing")?;
    upsert_record(
        &conn,
        &RecordRow {
            state: state.to_string(),
            fingerprint: record_fingerprint(&text).map_err(|error| error.message)?,
            ..original
        },
    )?;
    Ok(())
}

fn field(output: &str, marker: &str) -> Result<String, String> {
    output
        .split(marker)
        .nth(1)
        .and_then(|value| value.split_whitespace().next())
        .map(str::to_string)
        .ok_or_else(|| format!("missing {marker} in {output}"))
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
