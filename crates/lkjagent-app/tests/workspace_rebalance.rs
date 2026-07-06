use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{upsert_record, RecordRow};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn workspace_rebalance_plans_applies_audits_and_resolves_alias() -> TestResult<()> {
    let data = fixture_root("rebalance")?;
    let workspace = data.join("workspace");
    fs::create_dir_all(workspace.join("records/note"))?;
    let body = record_body();
    fs::write(workspace.join("records/note/old.md"), &body)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_1".to_string(),
            kind: "todo".to_string(),
            title: "Move me".to_string(),
            state: "open".to_string(),
            path: "records/note/old.md".to_string(),
            fingerprint: record_fingerprint(&body)
                .map_err(|error| std::io::Error::other(error.message))?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    drop(conn);

    let plan = cli::run(["--data", data_str(&data), "workspace", "plan-rebalance"])?;
    assert!(plan.contains("records/note/old.md -> records/todo/rec_1.md"));
    let applied = cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"])?;
    assert!(applied.contains("move rec_1"));
    assert!(workspace.join("records/todo/rec_1.md").exists());
    assert!(!workspace.join("records/note/old.md").exists());

    let shown = cli::run([
        "--data",
        data_str(&data),
        "record",
        "show",
        "records/note/old.md",
    ])?;
    assert!(shown.contains("record rec_1"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let audits: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_rebalance_audit",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(audits, 1);
    let valid = cli::run(["--data", data_str(&data), "workspace", "validate"])?;
    assert_eq!(valid, "workspace validate: ok");
    Ok(())
}

fn record_body() -> String {
    "---\nid: rec_1\nkind: todo\ntitle: Move me\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# Move me\n".to_string()
}

fn data_str(path: &std::path::Path) -> &str {
    path.to_str().unwrap_or("")
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
