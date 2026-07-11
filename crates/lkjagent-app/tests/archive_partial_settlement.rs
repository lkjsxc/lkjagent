use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_record::archive_path;
use lkjagent_store::record_rows::record;
use lkjagent_store::workspace_rows::{
    prepare_or_load_operation, OperationDraft, OperationRevision,
};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn archive_recovery_restores_preimage_alias_and_cells() -> TestResult<()> {
    let data = fixture_root()?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "todo",
        "Partial",
        "Settlement",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let old_path = field(&added, "path=")?;
    let workspace = data.join("workspace");
    let old = workspace.join(&old_path);
    let destination = archive_path("todo", &id)?;
    let new = workspace.join(&destination);
    let bytes = fs::read(&old)?;
    fs::create_dir_all(
        new.parent()
            .ok_or_else(|| std::io::Error::other("missing parent"))?,
    )?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row = record(&conn, &id)?.ok_or("record missing")?;
    let fingerprint = row.fingerprint.clone();
    let preimage = serde_json::to_string(&row)?;
    let operation_id = format!("workspace-archive-{id}");
    let revision_fingerprint =
        stable_fingerprint(&bytes).map_err(|error| std::io::Error::other(error.message))?;
    let intended =
        serde_json::json!({"id": id, "path": destination, "state": "archived"}).to_string();
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
            preimage: &preimage,
            intended: &intended,
            revisions: &revisions,
            now: "now",
        },
    )?;
    fs::rename(&old, &new)?;
    conn.execute(
        "UPDATE workspace_records SET path = ?1, state = 'archived', archived = 1 WHERE id = ?2",
        [&destination, &id],
    )?;
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
    let alias: String = conn.query_row(
        "SELECT new_path FROM workspace_path_aliases WHERE old_path = ?1",
        [&old_path],
        |row| row.get(0),
    )?;
    let status: String = conn.query_row(
        "SELECT status FROM state_cells WHERE key_label = ?1",
        [format!("todo:open/{id}")],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "settled");
    assert_eq!(alias, destination);
    assert_eq!(status, "Suppressed");
    assert!(new.exists());
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
    let path =
        std::env::temp_dir().join(format!("lkjagent-archive-partial-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
