use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{upsert_record, RecordRow};
use rusqlite::Connection;

const OLD_A: &str = "records/knowledge/notes/a.md";
const OLD_B: &str = "records/knowledge/notes/b.md";
const NEW_A: &str = "records/life/todo/open/rec_a.md";
const NEW_B: &str = "records/life/todo/open/rec_b.md";
static NEXT: AtomicU64 = AtomicU64::new(1);
mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn multi_move_group_settles_with_exact_membership() -> TestResult<()> {
    let fixture = fixture("settle")?;
    cli::run([
        "--data",
        data(&fixture.data),
        "workspace",
        "apply-rebalance",
    ])?;
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    let row: (String, i64) = conn.query_row(
        "SELECT phase, (SELECT COUNT(*) FROM workspace_operation_revisions
         WHERE operation_id = workspace_operations.id)
         FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row, ("settled".to_string(), 4));
    let aliases: i64 =
        conn.query_row("SELECT COUNT(*) FROM workspace_path_aliases", [], |row| {
            row.get(0)
        })?;
    assert_eq!(aliases, 2);
    assert_moved(&fixture.workspace);
    Ok(())
}

#[test]
fn group_preparation_is_atomic_before_first_move() -> TestResult<()> {
    let fixture = fixture("prepare-rollback")?;
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_group_revision BEFORE INSERT ON workspace_operation_revisions
         WHEN NEW.role = 'prior:0001' BEGIN SELECT RAISE(FAIL, 'revision blocked'); END;",
    )?;
    drop(conn);
    assert!(cli::run([
        "--data",
        data(&fixture.data),
        "workspace",
        "apply-rebalance"
    ])
    .is_err());
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    let operations: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| row.get(0),
    )?;
    let revisions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_operation_revisions",
        [],
        |row| row.get(0),
    )?;
    assert_eq!((operations, revisions), (0, 0));
    assert_sources(&fixture.workspace);
    Ok(())
}

#[test]
fn started_group_resumes_partial_moves_but_unstarted_group_blocks() -> TestResult<()> {
    let fixture = fixture("resume")?;
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_group_start BEFORE UPDATE OF phase ON workspace_operations
         WHEN OLD.kind = 'rebalance-group' AND NEW.phase = 'moving'
         BEGIN SELECT RAISE(FAIL, 'start blocked'); END;",
    )?;
    drop(conn);
    assert!(cli::run([
        "--data",
        data(&fixture.data),
        "workspace",
        "apply-rebalance"
    ])
    .is_err());
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    let error = run_until_idle(&fixture.data, &mut endpoint, 1)
        .err()
        .ok_or("unstarted group ran")?;
    assert!(error.contains("requires explicit apply"));
    assert_sources(&fixture.workspace);
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    conn.execute("DROP TRIGGER fail_group_start", [])?;
    conn.execute(
        "UPDATE workspace_operations SET phase = 'moving' WHERE kind = 'rebalance-group'",
        [],
    )?;
    drop(conn);
    fs::create_dir_all(
        fixture
            .workspace
            .join(NEW_A)
            .parent()
            .ok_or("target parent missing")?,
    )?;
    fs::rename(fixture.workspace.join(OLD_A), fixture.workspace.join(NEW_A))?;
    run_until_idle(&fixture.data, &mut endpoint, 1)?;
    let conn = Connection::open(fixture.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE kind = 'rebalance-group'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    assert_moved(&fixture.workspace);
    Ok(())
}

fn fixture(name: &str) -> TestResult<Fixture> {
    let data = std::env::temp_dir().join(format!(
        "lkjagent-group-{name}-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    if data.exists() {
        fs::remove_dir_all(&data)?;
    }
    let workspace = support::isolate_workspace(&data)?;
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    for (id, title, path) in [("rec_a", "Alpha", OLD_A), ("rec_b", "Beta", OLD_B)] {
        let body = body(id, title);
        fs::write(workspace.join(path), &body)?;
        upsert_record(
            &conn,
            &RecordRow {
                id: id.to_string(),
                kind: "todo".to_string(),
                title: title.to_string(),
                state: "open".to_string(),
                path: path.to_string(),
                fingerprint: record_fingerprint(&body).map_err(|error| error.message)?,
                archived: false,
                updated_at: "old".to_string(),
            },
        )?;
    }
    Ok(Fixture { data, workspace })
}

fn body(id: &str, title: &str) -> String {
    format!("---\nid: {id}\nkind: todo\ntitle: {title}\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# {title}\n")
}
fn assert_sources(workspace: &Path) {
    assert!(workspace.join(OLD_A).exists());
    assert!(workspace.join(OLD_B).exists());
    assert!(!workspace.join(NEW_A).exists());
    assert!(!workspace.join(NEW_B).exists());
}
fn assert_moved(workspace: &Path) {
    assert!(!workspace.join(OLD_A).exists());
    assert!(!workspace.join(OLD_B).exists());
    assert!(workspace.join(NEW_A).exists());
    assert!(workspace.join(NEW_B).exists());
}
fn data(path: &Path) -> &str {
    path.to_str().unwrap_or("")
}
struct Fixture {
    data: PathBuf,
    workspace: PathBuf,
}
