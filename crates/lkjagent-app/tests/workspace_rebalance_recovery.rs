use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::record_fingerprint;
use lkjagent_store::plan_schema::setup;
use lkjagent_store::record_rows::{upsert_record, RecordRow};
use lkjagent_store::workspace_rows::{
    prepare_or_load_operation, OperationDraft, OperationRevision,
};
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);
const OLD: &str = "records/knowledge/notes/old.md";
const NEW: &str = "records/life/todo/open/rec_1.md";
const OPERATION: &str = "workspace-rebalance-recovery";

#[test]
fn rebalance_startup_settles_moved_exact_revisions() -> TestResult<()> {
    let setup = prepared_move()?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&setup.data, &mut endpoint, 1)?;

    let conn = Connection::open(setup.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [OPERATION],
        |row| row.get(0),
    )?;
    let path: String = conn.query_row(
        "SELECT path FROM workspace_records WHERE id = 'rec_1'",
        [],
        |row| row.get(0),
    )?;
    let alias: String = conn.query_row(
        "SELECT new_path FROM workspace_path_aliases WHERE old_path = ?1",
        [OLD],
        |row| row.get(0),
    )?;
    let revisions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM workspace_operation_revisions WHERE operation_id = ?1",
        [OPERATION],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    assert_eq!(path, NEW);
    assert_eq!(alias, NEW);
    assert_eq!(revisions, 2);
    assert!(setup.workspace.join(NEW).exists());
    assert!(!setup.workspace.join(OLD).exists());
    Ok(())
}

#[test]
#[rustfmt::skip]
fn rebalance_startup_preserves_conflicting_target() -> TestResult<()> {
    let setup = prepared_move()?;
    let owner = setup.data.join("owner.md"); fs::write(&owner, "owner bytes")?;
    fs::remove_file(setup.workspace.join(NEW))?;
    std::os::unix::fs::symlink(&owner, setup.workspace.join(NEW))?;
    let mut endpoint = ScriptedEndpoint { outputs: vec![], index: 0 };
    let error = run_until_idle(&setup.data, &mut endpoint, 1).err().ok_or_else(|| std::io::Error::other("rebalance startup unexpectedly succeeded"))?;
    assert!(error.contains("target conflicts"));
    assert!(fs::symlink_metadata(setup.workspace.join(NEW))?.file_type().is_symlink());
    assert_eq!(fs::read(setup.workspace.join(NEW))?, b"owner bytes");
    let conn = Connection::open(setup.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row("SELECT phase FROM workspace_operations WHERE id = ?1", [OPERATION], |row| row.get(0))?;
    assert_eq!(phase, "prepared");
    Ok(())
}

#[test]
fn rebalance_startup_preserves_moved_bytes_when_settlement_projection_fails() -> TestResult<()> {
    let setup = prepared_move()?;
    let conn = Connection::open(setup.data.join("lkjagent.sqlite3"))?;
    conn.execute_batch(
        "CREATE TRIGGER fail_recovery_projection BEFORE INSERT ON workspace_rebalance_audit
         BEGIN SELECT RAISE(FAIL, 'audit blocked'); END;",
    )?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    assert!(run_until_idle(&setup.data, &mut endpoint, 1).is_err());
    let conn = Connection::open(setup.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [OPERATION],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "prepared");
    assert!(!setup.workspace.join(OLD).exists());
    assert!(setup.workspace.join(NEW).exists());
    Ok(())
}

fn prepared_move() -> TestResult<Prepared> {
    let data = fixture_root()?;
    let workspace = data.join("workspace");
    let body = record_body();
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
    fs::create_dir_all(workspace.join("records/life/todo/open"))?;
    fs::write(workspace.join(OLD), &body)?;
    let fingerprint = record_fingerprint(&body).map_err(|error| error.message)?;
    let original = RecordRow {
        id: "rec_1".to_string(),
        kind: "todo".to_string(),
        title: "Move me".to_string(),
        state: "open".to_string(),
        path: OLD.to_string(),
        fingerprint: fingerprint.clone(),
        archived: false,
        updated_at: "old".to_string(),
    };
    let item = RebalanceMove {
        entity_id: original.id.clone(),
        entity_kind: "record".to_string(),
        old_path: OLD.to_string(),
        new_path: NEW.to_string(),
        decision_id: "workspace.rebalance".to_string(),
        reason: "canonical record path".to_string(),
        validation: vec!["ok".to_string()],
    };
    let bytes = body.into_bytes();
    let stable = stable_fingerprint(&bytes).map_err(|error| error.message)?;
    let revisions = vec![
        revision("prior", OLD, &bytes, &stable),
        revision("intended", NEW, &bytes, &stable),
    ];
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(&conn, &original)?;
    prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: OPERATION,
            key: &format!("rebalance:rec_1:{fingerprint}"),
            kind: "rebalance",
            preimage: &preimage(&original),
            intended: &serde_json::json!({"id": "rec_1", "path": NEW, "move": item}).to_string(),
            revisions: &revisions,
            now: "now",
        },
    )?;
    fs::rename(workspace.join(OLD), workspace.join(NEW))?;
    Ok(Prepared { data, workspace })
}

fn revision(role: &str, path: &str, bytes: &[u8], fingerprint: &str) -> OperationRevision {
    OperationRevision {
        role: role.to_string(),
        path: path.to_string(),
        bytes: bytes.to_vec(),
        fingerprint: fingerprint.to_string(),
    }
}

fn preimage(row: &RecordRow) -> String {
    serde_json::json!({"id": row.id, "kind": row.kind, "title": row.title, "state": row.state, "path": row.path, "fingerprint": row.fingerprint, "archived": row.archived, "updated_at": row.updated_at}).to_string()
}

fn record_body() -> String {
    "---\nid: rec_1\nkind: todo\ntitle: Move me\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# Move me\n".to_string()
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-rebalance-recovery-{}-{}",
        std::process::id(),
        NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}

struct Prepared {
    data: PathBuf,
    workspace: PathBuf,
}
