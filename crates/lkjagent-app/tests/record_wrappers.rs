use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn friendly_wrappers_write_generic_records() -> TestResult<()> {
    let data = fixture_root("record-wrappers")?;

    let added = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "todo",
        "Buy",
        "milk",
    ])?;
    assert!(added.contains("path=records/life/todo/open/"));
    let id = added.split_whitespace().nth(1).ok_or("missing id")?;

    let listed = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "record",
        "list",
        "todo",
    ])?;
    assert!(listed.contains("kind=todo"));
    assert!(listed.contains("title=Buy milk"));

    let shown = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "record",
        "show",
        id,
    ])?;
    assert!(shown.contains("kind: todo"));
    assert!(shown.contains("## Body\n\nBuy milk"));

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let labels = state_labels(&conn)?;
    assert!(labels.contains(&"todo:open/".to_string()));
    assert!(labels.contains(&"index:stale/records".to_string()));
    assert!(edge_relations(&conn)?.contains(&"owns".to_string()));
    assert!(edge_relations(&conn)?.contains(&"stale-input".to_string()));

    let dev = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "dev",
        "Fix",
        "parser",
    ])?;
    assert!(dev.contains("path=records/work/development/"));
    assert!(state_labels(&conn)?.contains(&"dev:repo-work/".to_string()));

    let finance = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "finance",
        "Pay",
        "bill",
    ])?;
    assert!(finance.contains("path=records/life/finance/"));
    assert!(!finance.contains("unix:"));
    assert!(state_labels(&conn)?.contains(&"finance:review/".to_string()));
    Ok(())
}

fn state_labels(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = conn.prepare("SELECT key_label FROM state_cells ORDER BY key_label")?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    rows.map(|row| row.map(|label| label_prefix(&label)))
        .collect()
}

fn edge_relations(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = conn.prepare("SELECT relation FROM state_edges ORDER BY relation")?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    rows.collect()
}

fn label_prefix(label: &str) -> String {
    if let Some((prefix, _)) = label.rsplit_once("rec_") {
        prefix.to_string()
    } else {
        label.to_string()
    }
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
