use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_record::archive_path;
use lkjagent_store::workspace_rows::{
    prepare_or_load_operation, OperationDraft, OperationRevision,
};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

#[test]
fn archive_resumes_prepared_operation_after_file_move() -> TestResult<()> {
    let data = fixture_root()?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Resume",
        "Archive",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let workspace = data.join("workspace");
    let old = workspace.join(&old_path);
    let destination = archive_path("custom", &id)?;
    let new = workspace.join(&destination);
    let bytes = fs::read(&old)?;
    fs::create_dir_all(
        new.parent()
            .ok_or_else(|| std::io::Error::other("missing parent"))?,
    )?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let fingerprint: String = conn.query_row(
        "SELECT fingerprint FROM workspace_records WHERE id = ?1",
        [&id],
        |row| row.get(0),
    )?;
    let operation_id = format!("workspace-archive-{id}");
    let key = format!("archive:{id}:{fingerprint}");
    let revision_fingerprint =
        stable_fingerprint(&bytes).map_err(|error| std::io::Error::other(error.message))?;
    let revisions = vec![
        revision("prior", &old_path, &bytes, &revision_fingerprint),
        revision("intended", &destination, &bytes, &revision_fingerprint),
    ];
    prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: &operation_id,
            key: &key,
            kind: "archive",
            preimage: &serde_json::json!({"id": id, "path": old_path, "fingerprint": fingerprint, "state": "open", "archived": false}).to_string(),
            intended: "{}",
            revisions: &revisions,
            now: "now",
        },
    )?;
    fs::rename(&old, &new)?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [&operation_id],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    assert!(new.exists());
    assert!(!old.exists());
    Ok(())
}

#[test]
fn archive_startup_preserves_conflicting_target() -> TestResult<()> {
    let data = fixture_root()?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Conflict",
        "Archive",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let workspace = data.join("workspace");
    let old = workspace.join(&old_path);
    let destination = archive_path("custom", &id)?;
    let new = workspace.join(&destination);
    let bytes = fs::read(&old)?;
    fs::create_dir_all(
        new.parent()
            .ok_or_else(|| std::io::Error::other("missing parent"))?,
    )?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let fingerprint: String = conn.query_row(
        "SELECT fingerprint FROM workspace_records WHERE id = ?1",
        [&id],
        |row| row.get(0),
    )?;
    let operation_id = format!("workspace-archive-{id}");
    let revision_fingerprint =
        stable_fingerprint(&bytes).map_err(|error| std::io::Error::other(error.message))?;
    let revisions = vec![
        revision("prior", &old_path, &bytes, &revision_fingerprint),
        revision("intended", &destination, &bytes, &revision_fingerprint),
    ];
    prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: &operation_id,
            key: &format!("archive:{id}:{fingerprint}"),
            kind: "archive",
            preimage: &serde_json::json!({"id": id, "path": old_path, "fingerprint": fingerprint, "state": "open", "archived": false}).to_string(),
            intended: "{}",
            revisions: &revisions,
            now: "now",
        },
    )?;
    fs::rename(&old, &new)?;
    fs::write(&new, "owner bytes")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    let error = run_until_idle(&data, &mut endpoint, 1)
        .err()
        .ok_or_else(|| std::io::Error::other("startup unexpectedly succeeded"))?;
    assert!(error.contains("conflicts"));
    assert_eq!(fs::read(&new)?, b"owner bytes");
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = ?1",
        [&operation_id],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "prepared");
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

fn field(output: &str, marker: &str) -> Result<String, String> {
    output
        .split(marker)
        .nth(1)
        .and_then(|value| value.split_whitespace().next())
        .map(str::to_string)
        .ok_or_else(|| format!("missing {marker} in {output}"))
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-archive-recovery-{}-{}",
        std::process::id(),
        NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed),
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
