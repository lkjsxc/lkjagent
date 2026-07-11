use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::cli;
use lkjagent_core::workspace_record::{parse_record, record_fingerprint};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{record, upsert_record, RecordRow};
use lkjagent_store::workspace_search::canonical_rows;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn workspace_search_discovers_body_filters_and_rejects_drift() -> TestResult<()> {
    let data = fixture_root("search")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "todo",
        "Plan",
        "--body",
        "aurora needle appears only in this body",
    ])?;
    let path = field(&added, "path=")?;
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;

    let found = cli::run([
        "--data",
        data_arg.as_ref(),
        "workspace",
        "search",
        "aurora",
        "--kind",
        "todo",
        "--state",
        "open",
    ])?;
    assert!(found.contains("aurora needle"));
    assert!(found.contains("kind=todo"));
    let filtered = cli::run([
        "--data",
        data_arg.as_ref(),
        "workspace",
        "search",
        "aurora",
        "--kind",
        "note",
    ])?;
    assert_eq!(filtered, "no matches");

    fs::write(data.join("workspace").join(path), "owner changed bytes")?;
    let drift = cli::run(["--data", data_arg.as_ref(), "workspace", "search", "aurora"])?;
    assert!(drift.contains("excluded_drifted=1"));
    Ok(())
}

#[test]
fn search_uses_trigram_project_date_and_stable_rebuild() -> TestResult<()> {
    let data = fixture_root("search-modes")?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "todo",
        "Lantern",
        "--body",
        "starlight-lantern illuminates a Japanese excerpt: こんにちは",
    ])?;
    let id = field(&added, "record: ")?;
    let path = field(&added, "path=")?;
    let date = tag_project(&data, &id, &path, "alpha")?;
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;

    let found = cli::run([
        "--data",
        data_arg.as_ref(),
        "workspace",
        "search",
        "arl",
        "--mode",
        "trigram",
        "--project",
        "alpha",
        "--date",
        &date,
    ])?;
    assert!(found.contains("starlight-lantern"));
    let absent = cli::run([
        "--data",
        data_arg.as_ref(),
        "workspace",
        "search",
        "arl",
        "--mode",
        "trigram",
        "--project",
        "beta",
    ])?;
    assert_eq!(absent, "no matches");

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let first = canonical_rows(&conn)?;
    drop(conn);
    cli::run(["--data", data_arg.as_ref(), "workspace", "--rebuild"])?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(first, canonical_rows(&conn)?);
    Ok(())
}

fn tag_project(data: &Path, id: &str, path: &str, project: &str) -> TestResult<String> {
    let full = data.join("workspace").join(path);
    let text = fs::read_to_string(&full)?;
    let text = text.replace("tags: []", &format!("tags: [project:{project}]"));
    let parsed = parse_record(&text)?;
    fs::write(full, &text)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let original = record(&conn, id)?.ok_or("record missing")?;
    upsert_record(
        &conn,
        &RecordRow {
            fingerprint: record_fingerprint(&text).map_err(|error| error.message)?,
            ..original
        },
    )?;
    Ok(parsed
        .created_at
        .split('T')
        .next()
        .unwrap_or_default()
        .to_string())
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
