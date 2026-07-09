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
    let budget = fs::read_to_string(data.join("workspace/indexes/budget-month.md"))?;
    assert!(budget.contains("Pay bill"));
    assert!(artifact_paths(&conn)?.contains(&"indexes/budget-month.md".to_string()));

    let calendar = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "calendar",
        "Team",
        "review",
    ])?;
    assert!(calendar.contains("path=records/life/calendar/"));
    assert!(state_labels(&conn)?.contains(&"calendar:due/".to_string()));

    let note = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "note",
        "Useful",
        "fact",
    ])?;
    assert!(note.contains("path=records/life/notes/"));

    let project = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "project",
        "lkjagent",
        "plan",
    ])?;
    assert!(project.contains("path=records/work/projects/"));
    assert!(state_labels(&conn)?.contains(&"project:active/".to_string()));
    Ok(())
}

#[test]
fn large_record_body_is_split_into_linked_parts() -> TestResult<()> {
    let data = fixture_root("record-large-body")?;
    let body = "alpha ".repeat(500);

    let added = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "record",
        "add",
        "note",
        "Large",
        "note",
        "--body",
        body.as_str(),
    ])?;
    let id = added.split_whitespace().nth(1).ok_or("missing id")?;
    let root = data.join("workspace/records/life/notes");
    let main = fs::read_to_string(root.join(format!("{id}.md")))?;
    let first = fs::read_to_string(root.join(format!("{id}.parts/part-001.md")))?;

    assert!(main.contains("Size justification"));
    assert!(main.contains(&format!("{id}.parts/part-001.md")));
    assert!(!main.contains(&body[..80]));
    assert!(first.contains("alpha alpha"));
    assert!(root.join(format!("{id}.parts/part-002.md")).exists());
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

fn artifact_paths(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = conn.prepare("SELECT path FROM artifacts ORDER BY path")?;
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
