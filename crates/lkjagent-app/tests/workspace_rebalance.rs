use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{upsert_record, RecordRow};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn workspace_rebalance_plans_applies_audits_and_resolves_alias() -> TestResult<()> {
    let data = fixture_root("rebalance")?;
    let workspace = data.join("workspace");
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    fs::create_dir_all(workspace.join("records/life/notes"))?;
    let body = record_body();
    let linked = linked_record_body("records/knowledge/notes/old.md");
    fs::write(workspace.join("records/knowledge/notes/old.md"), &body)?;
    fs::write(workspace.join("records/life/notes/rec_2.md"), &linked)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_1".to_string(),
            kind: "todo".to_string(),
            title: "Move me".to_string(),
            state: "open".to_string(),
            path: "records/knowledge/notes/old.md".to_string(),
            fingerprint: record_fingerprint(&body)
                .map_err(|error| std::io::Error::other(error.message))?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_2".to_string(),
            kind: "note".to_string(),
            title: "Linker".to_string(),
            state: "open".to_string(),
            path: "records/life/notes/rec_2.md".to_string(),
            fingerprint: record_fingerprint(&linked)
                .map_err(|error| std::io::Error::other(error.message))?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    drop(conn);

    let plan = cli::run(["--data", data_str(&data), "workspace", "plan-rebalance"])?;
    assert!(plan.contains("records/knowledge/notes/old.md -> records/life/todo/open/rec_1.md"));
    assert!(workspace
        .join("system/manifests/workspace-manifest.json")
        .exists());
    let applied = cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"])?;
    assert!(applied.contains("move rec_1"));
    assert!(workspace.join("records/life/todo/open/rec_1.md").exists());
    assert!(!workspace.join("records/knowledge/notes/old.md").exists());
    let todo_readme = fs::read_to_string(workspace.join("records/life/todo/README.md"))?;
    assert!(todo_readme.contains("[open](open/)"));
    let life_readme = fs::read_to_string(workspace.join("records/life/README.md"))?;
    assert!(life_readme.contains("[todo](todo/)"));
    let index = fs::read_to_string(workspace.join("indexes/open-todos.md"))?;
    assert!(index.contains("rec_1 [open] Move me (records/life/todo/open/rec_1.md)"));
    let repaired_link = fs::read_to_string(workspace.join("records/life/notes/rec_2.md"))?;
    let expected_link = linked.replacen("links: [records/knowledge/notes/old.md]", "links: [records/life/todo/open/rec_1.md]", 1);
    assert_eq!(repaired_link, expected_link);

    let shown = cli::run([
        "--data",
        data_str(&data),
        "record",
        "show",
        "records/knowledge/notes/old.md",
    ])?;
    assert!(shown.contains("record rec_1"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let audits: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_rebalance_audit",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(audits, 1);
    let validation: String = conn.query_row(
        "SELECT validation_json FROM workspace_rebalance_audit",
        [],
        |row| row.get(0),
    )?;
    assert!(validation.contains("fingerprint-before:"));
    assert!(validation.contains("links-repaired:1"));
    let link_fingerprint: String = conn.query_row(
        "SELECT fingerprint FROM workspace_records WHERE id = 'rec_2'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(
        link_fingerprint,
        record_fingerprint(&repaired_link).map_err(|error| std::io::Error::other(error.message))?
    );
    let alias: String = conn.query_row(
        "SELECT new_path FROM workspace_path_aliases WHERE old_path = ?1",
        ["records/knowledge/notes/old.md"],
        |row| row.get(0),
    )?;
    assert_eq!(alias, "records/life/todo/open/rec_1.md");
    let valid = cli::run(["--data", data_str(&data), "workspace", "validate"])?;
    assert_eq!(valid, "workspace validate: ok");
    let external_bytes = format!("{body}\nchanged");
    let record_path = workspace.join("records/life/todo/open/rec_1.md");
    fs::write(&record_path, &external_bytes)?;
    let stale = cli::run(["--data", data_str(&data), "workspace", "validate"])?;
    assert_eq!(stale, "workspace validate: stale rec_1");
    assert_eq!(fs::read_to_string(record_path)?, external_bytes);
    Ok(())
}

#[test]
fn workspace_rebalance_leaves_projection_retryable_when_audit_write_fails() -> TestResult<()> {
    let data = fixture_root("rebalance-audit-fail")?;
    let workspace = data.join("workspace");
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    let body = record_body();
    fs::write(workspace.join("records/knowledge/notes/old.md"), &body)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(
        &conn,
        &RecordRow {
            id: "rec_1".to_string(),
            kind: "todo".to_string(),
            title: "Move me".to_string(),
            state: "open".to_string(),
            path: "records/knowledge/notes/old.md".to_string(),
            fingerprint: record_fingerprint(&body)
                .map_err(|error| std::io::Error::other(error.message))?,
            archived: false,
            updated_at: "old".to_string(),
        },
    )?;
    conn.execute(
        "CREATE TRIGGER fail_rebalance_audit
         BEFORE INSERT ON workspace_rebalance_audit
         BEGIN SELECT RAISE(FAIL, 'audit blocked'); END",
        [],
    )?;
    drop(conn);

    let result = cli::run(["--data", data_str(&data), "workspace", "apply-rebalance"]);

    assert!(result.is_err());
    assert!(!workspace.join("records/knowledge/notes/old.md").exists());
    assert!(workspace.join("records/life/todo/open/rec_1.md").exists());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let path: String = conn.query_row(
        "SELECT path FROM workspace_records WHERE id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(path, "records/life/todo/open/rec_1.md");
    let aliases: i64 =
        conn.query_row("SELECT COUNT(*) FROM workspace_path_aliases", [], |row| {
            row.get(0)
        })?;
    assert_eq!(aliases, 0);
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

fn linked_record_body(target: &str) -> String {
    format!(
        "---\nid: rec_2\nkind: note\ntitle: Linker\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: [{target}]\nstate_keys: []\n---\n\n# Linker\n\nSee {target}\n"
    )
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
