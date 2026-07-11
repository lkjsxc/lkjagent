use std::{fs, path::PathBuf};

use lkjagent_app::cli;
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

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
const OLD: &str = "records/knowledge/notes/old.md";
const NEW: &str = "records/life/todo/open/rec_1.md";

#[test]
#[rustfmt::skip]
fn explicit_apply_resumes_exact_unstarted_operation() -> TestResult<()> {
    let prepared = prepare(true, "exact")?;
    let mut endpoint = ScriptedEndpoint { outputs: vec![], index: 0 };
    let error = run_until_idle(&prepared.data, &mut endpoint, 1).err().ok_or("unstarted operation replayed at startup")?;
    assert!(error.contains("requires explicit apply"));
    assert!(prepared.workspace.join(OLD).exists()); assert!(!prepared.workspace.join(NEW).exists());
    cli::run(["--data", prepared.data.to_string_lossy().as_ref(), "workspace", "apply-rebalance"])?;
    let conn = Connection::open(prepared.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row("SELECT phase FROM workspace_operations WHERE id = ?1", [&prepared.operation_id], |row| row.get(0))?;
    assert_eq!(phase, "settled");
    assert!(prepared.workspace.join(NEW).exists());
    assert!(!prepared.workspace.join(OLD).exists());
    Ok(())
}

#[test]
#[rustfmt::skip]
fn explicit_apply_preserves_dangling_target_conflict() -> TestResult<()> {
    let prepared = prepare(true, "dangling-target")?;
    fs::create_dir_all(prepared.workspace.join(NEW).parent().ok_or("missing parent")?)?;
    std::os::unix::fs::symlink(prepared.data.join("missing-owner"), prepared.workspace.join(NEW))?;
    let error = cli::run(["--data", prepared.data.to_string_lossy().as_ref(), "workspace", "apply-rebalance"]).err().ok_or("dangling target accepted")?;
    assert!(error.to_string().contains("prior path remains occupied"), "{error}"); assert!(prepared.workspace.join(OLD).exists());
    assert!(fs::symlink_metadata(prepared.workspace.join(NEW))?.file_type().is_symlink());
    let conn = Connection::open(prepared.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row("SELECT phase FROM workspace_operations WHERE id = ?1", [&prepared.operation_id], |row| row.get(0))?;
    assert_eq!(phase, "prepared"); Ok(())
}

#[test]
#[rustfmt::skip]
fn missing_intended_revision_blocks_without_moving_source() -> TestResult<()> {
    let prepared = prepare(false, "missing")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    let error = run_until_idle(&prepared.data, &mut endpoint, 1)
        .err()
        .ok_or("revisionless operation unexpectedly recovered")?;
    assert!(error.contains("intended revision missing"));
    assert!(prepared.workspace.join(OLD).exists());
    assert!(!prepared.workspace.join(NEW).exists());
    let conn = Connection::open(prepared.data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [&prepared.operation_id],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "prepared");
    let mismatch = prepare(true, "different-revisions")?;
    let conn = Connection::open(mismatch.data.join("lkjagent.sqlite3"))?;
    let fingerprint = lkjagent_core::runtime_fingerprint::stable_fingerprint(b"xx").map_err(|error| error.message)?;
    conn.execute("UPDATE workspace_operation_revisions SET bytes = X'7878', fingerprint = ?1", [fingerprint])?; drop(conn);
    let error = run_until_idle(&mismatch.data, &mut endpoint, 1).err().ok_or("changed revisions accepted")?;
    assert!(error.contains("content conflicts")); assert!(mismatch.workspace.join(OLD).exists());
    Ok(())
}

#[test]
#[rustfmt::skip]
fn owner_row_change_blocks_unstarted_resume() -> TestResult<()> {
    let prepared = prepare(true, "row-conflict")?;
    let conn = Connection::open(prepared.data.join("lkjagent.sqlite3"))?;
    conn.execute("UPDATE workspace_records SET title = 'owner title' WHERE id = 'rec_1'", [])?; drop(conn);
    let mut endpoint = ScriptedEndpoint { outputs: vec![], index: 0 };
    let error = run_until_idle(&prepared.data, &mut endpoint, 1).err().ok_or("row conflict ignored")?;
    assert!(error.contains("record preimage changed")); assert!(prepared.workspace.join(OLD).exists());
    Ok(())
}

#[test]
#[rustfmt::skip]
fn invalid_persisted_move_blocks_before_filesystem_change() -> TestResult<()> {
    let prepared = prepare(true, "invalid-intent")?;
    let conn = Connection::open(prepared.data.join("lkjagent.sqlite3"))?;
    conn.execute("UPDATE workspace_operations SET intended_json = replace(intended_json, ?1, '../escape.md')", [OLD])?; drop(conn);
    let mut endpoint = ScriptedEndpoint { outputs: vec![], index: 0 };
    let error = run_until_idle(&prepared.data, &mut endpoint, 1).err().ok_or("invalid intent accepted")?;
    assert!(error.contains("persisted rebalance move is invalid")); assert!(prepared.workspace.join(OLD).exists());
    Ok(())
}

fn prepare(with_intended: bool, suffix: &str) -> TestResult<Prepared> {
    let data = fixture_root(suffix)?;
    let workspace = data.join("workspace");
    let body = record_body();
    fs::create_dir_all(workspace.join("records/knowledge/notes"))?;
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
    let mut revisions = vec![revision("prior", OLD, &bytes, &stable)];
    if with_intended {
        revisions.push(revision("intended", NEW, &bytes, &stable));
    }
    let identity = format!("rec_1\0{OLD}\0{NEW}");
    let audit = stable_fingerprint(&identity).map_err(|error| error.message)?;
    let operation_id = format!("workspace-rebalance-{audit}-{fingerprint}");
    let key = format!("rebalance:rec_1:{OLD}:{NEW}:{fingerprint}");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    upsert_record(&conn, &original)?;
    prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: &operation_id,
            key: &key,
            kind: "rebalance",
            preimage: &preimage(&original),
            intended: &serde_json::json!({"id": "rec_1", "path": NEW, "move": item}).to_string(),
            revisions: &revisions,
            now: "now",
        },
    )?;
    Ok(Prepared {
        data,
        workspace,
        operation_id,
    })
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
    serde_json::json!({"id": row.id, "kind": row.kind, "title": row.title, "state": row.state,
        "path": row.path, "fingerprint": row.fingerprint, "archived": row.archived,
        "updated_at": row.updated_at})
    .to_string()
}

fn record_body() -> String {
    "---\nid: rec_1\nkind: todo\ntitle: Move me\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# Move me\n".to_string()
}

fn fixture_root(suffix: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-rebalance-retry-{suffix}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

struct Prepared {
    data: PathBuf,
    workspace: PathBuf,
    operation_id: String,
}
