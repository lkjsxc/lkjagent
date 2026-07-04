use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn context_resolve_command_writes_resolution_cell() -> TestResult<()> {
    let data = fixture_root("resolve")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    insert_case(&conn, "1", "Resolve conflict.", "now")?;
    drop(conn);

    let output = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "context",
        "resolve",
        "1",
        "target-root",
        "ctx-a",
    ])?;

    assert!(output.contains("context resolved: target-root -> ctx-a"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let payload: String = conn.query_row(
        "SELECT payload_json FROM state_cells
         WHERE key_label = 'context:resolve/target-root'",
        [],
        |row| row.get(0),
    )?;
    assert!(payload.contains("ctx-a"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-context-cli-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
