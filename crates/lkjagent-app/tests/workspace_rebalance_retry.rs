use std::fs;
use std::path::PathBuf;

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
fn apply_rebalance_resumes_moved_prepared_operation() -> TestResult<()> {
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
    fs::rename(workspace.join(OLD), workspace.join(NEW))?;
    drop(conn);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute_batch("CREATE TRIGGER fail_recovery_index BEFORE INSERT ON artifacts WHEN NEW.id LIKE 'index-%' BEGIN SELECT RAISE(FAIL, 'index blocked'); END;")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    assert!(run_until_idle(&data, &mut endpoint, 1).is_err());
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    conn.execute_batch("DROP TRIGGER fail_recovery_index")?;
    drop(conn);

    cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "workspace",
        "apply-rebalance",
    ])?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [&operation_id],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    assert!(workspace.join(NEW).exists());
    assert!(!workspace.join(OLD).exists());
    Ok(())
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
    serde_json::json!({"id": row.id, "kind": row.kind, "title": row.title, "state": row.state, "path": row.path, "fingerprint": row.fingerprint, "archived": row.archived}).to_string()
}

fn record_body() -> String {
    "---\nid: rec_1\nkind: todo\ntitle: Move me\nstate: open\ncreated_at: old\nupdated_at: old\ntags: []\nlinks: []\nstate_keys: []\n---\n\n# Move me\n".to_string()
}

fn fixture_root() -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-rebalance-retry-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
