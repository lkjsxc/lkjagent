use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn archive_restores_file_and_row_when_audit_fails() -> TestResult<()> {
    let data = fixture_root("archive-compensation")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Archive",
        "Compensation",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let workspace = data.join("workspace");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute(
        "CREATE TRIGGER fail_archive_audit
         BEFORE INSERT ON workspace_rebalance_audit
         BEGIN SELECT RAISE(FAIL, 'audit blocked'); END",
        [],
    )?;
    drop(conn);

    let result = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id]);

    assert!(result.is_err());
    assert!(workspace.join(&old_path).exists());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (String, i64) = conn.query_row(
        "SELECT path, archived FROM workspace_records WHERE id = ?1",
        [&id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row.0, old_path);
    assert_eq!(row.1, 0);
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
