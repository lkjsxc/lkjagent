use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_core::workspace_record::archive_path;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn archive_restores_file_and_row_when_audit_fails() -> TestResult<()> {
    let data = fixture_root("archive-compensation")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run(["--data", data_arg.as_ref(), "record", "add", "custom", "Archive", "Compensation", "--body", "body"])?;
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
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE kind = 'archive'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "compensated");
    conn.execute("DROP TRIGGER fail_archive_audit", [])?; drop(conn);
    cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row("SELECT phase FROM workspace_operations WHERE kind = 'archive'", [], |row| row.get(0))?;
    let operations: i64 = conn.query_row("SELECT COUNT(*) FROM workspace_operations WHERE kind = 'archive'", [], |row| row.get(0))?;
    assert_eq!(phase, "settled"); assert_eq!(operations, 1);
    assert!(!workspace.join(&old_path).exists());
    assert!(workspace.join(archive_path("custom", &id)?).exists());
    Ok(())
}

#[test]
fn archive_rejects_changed_source_bytes() -> TestResult<()> {
    let data = fixture_root("archive-stale-source")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Archive",
        "Stale",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    fs::write(data.join("workspace").join(&old_path), "owner changed")?;
    let result = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id]);
    assert!(result
        .as_deref()
        .is_err_and(|error| error.contains("fingerprint")));
    assert!(data.join("workspace").join(old_path).exists());
    Ok(())
}

#[test]
fn archive_preserves_occupied_destination() -> TestResult<()> {
    let data = fixture_root("archive-owner-destination")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Archive",
        "Destination",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let destination = archive_path("custom", &id)?;
    let full = data.join("workspace").join(&destination);
    fs::create_dir_all(
        full.parent()
            .ok_or_else(|| std::io::Error::other("missing destination parent"))?,
    )?;
    fs::write(&full, "owner destination")?;
    assert!(cli::run(["--data", data_arg.as_ref(), "record", "archive", &id]).is_err());
    assert_eq!(fs::read_to_string(full)?, "owner destination");
    assert!(data.join("workspace").join(old_path).exists());
    Ok(())
}

#[test]
fn archive_reuses_settled_operation_without_second_move() -> TestResult<()> {
    let data = fixture_root("archive-idempotent")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Archive",
        "Again",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let first = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?;
    let second = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?;
    assert_eq!(first, second);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let rows: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_operations WHERE kind = 'archive'",
        [],
        |row| row.get(0),
    )?;
    let revisions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_operation_revisions",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(rows, 1);
    assert_eq!(revisions, 2);
    Ok(())
}

#[test]
fn archive_restores_indexes_when_state_suppression_fails() -> TestResult<()> {
    let data = fixture_root("archive-index-compensation")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "todo",
        "Restore",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_archive_suppress
         BEFORE INSERT ON runtime_events
         WHEN NEW.source = 'workspace-record' AND NEW.kind = 'state.cell.suppress'
         BEGIN SELECT RAISE(FAIL, 'suppress blocked'); END;",
    )?;
    drop(conn);

    let result = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id]);

    assert!(result.is_err());
    let index = fs::read_to_string(data.join("workspace/indexes/open-todos.md"))?;
    assert!(index.contains(&id));
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
