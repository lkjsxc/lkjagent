use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{upsert_record, RecordRow};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn rebalance_retries_group_projection_when_index_rebuild_fails() -> TestResult<()> {
    let data = fixture_root("rebalance-index-fail")?;
    let workspace = data.join("workspace");
    let old = "records/knowledge/notes/old.md";
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    let body = record_body();
    fs::write(workspace.join(old), &body)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_1".to_string(),
            kind: "todo".to_string(),
            title: "Move me".to_string(),
            state: "open".to_string(),
            path: old.to_string(),
            fingerprint: record_fingerprint(&body).map_err(|error| error.message)?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    drop(conn);
    cli::run(["--data", data_str(&data), "workspace", "--rebuild"])?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_rebalance_index
         BEFORE INSERT ON artifacts
         WHEN NEW.id LIKE 'index-%'
         BEGIN SELECT RAISE(FAIL, 'index blocked'); END;",
    )?;
    drop(conn);

    let result = cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"]);

    assert!(result.is_err());
    assert!(!workspace.join(old).exists());
    assert!(workspace.join("records/life/todo/open/rec_1.md").exists());
    let index = fs::read_to_string(workspace.join("indexes/open-todos.md"))?;
    assert!(index.contains("rec_1 [open] Move me (records/knowledge/notes/old.md)"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let path: String = conn.query_row(
        "SELECT path FROM workspace_records WHERE id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(path, "records/life/todo/open/rec_1.md");
    let chunks: i64 =
        conn.query_row("SELECT COUNT(*) FROM workspace_search_chunks", [], |row| {
            row.get(0)
        })?;
    assert!(chunks > 0);
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "projecting");
    let history: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_record_history WHERE record_id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    conn.execute("DROP TRIGGER fail_rebalance_index", [])?;
    drop(conn);
    cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"])?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    let retried_history: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_record_history WHERE record_id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(retried_history, history);
    let index = fs::read_to_string(workspace.join("indexes/open-todos.md"))?;
    assert!(index.contains("records/life/todo/open/rec_1.md"));
    Ok(())
}

#[test]
fn rebalance_preserves_owner_readme_and_keeps_group_projecting() -> TestResult<()> {
    let data = fixture_root("rebalance-owner-readme")?;
    let workspace = data.join("workspace");
    let old = "records/knowledge/notes/old.md";
    let owner = workspace.join("records/life/todo/README.md");
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    fs::create_dir_all(owner.parent().ok_or("owner parent missing")?)?;
    fs::write(&owner, "# Owner\n\nDo not replace.\n")?;
    let body = record_body();
    fs::write(workspace.join(old), &body)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_1".to_string(),
            kind: "todo".to_string(),
            title: "Move me".to_string(),
            state: "open".to_string(),
            path: old.to_string(),
            fingerprint: record_fingerprint(&body).map_err(|error| error.message)?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    drop(conn);
    assert!(cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"]).is_err());
    assert_eq!(fs::read_to_string(&owner)?, "# Owner\n\nDo not replace.\n");
    assert!(workspace.join("records/life/todo/open/rec_1.md").exists());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "projecting");
    Ok(())
}

fn record_body() -> String {
    "---\nid: rec_1\nkind: todo\ntitle: Move me\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# Move me\n".to_string()
}

fn data_str(path: &std::path::Path) -> &str {
    path.to_str().unwrap_or("")
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
