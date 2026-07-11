use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::workspace_rows::{
    prepare_or_load_operation, OperationDraft, OperationRevision,
};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn startup_skips_non_archive_prepared_operation() -> TestResult<()> {
    let data = fixture_root()?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let revisions: Vec<OperationRevision> = vec![];
    prepare_or_load_operation(
        &conn,
        &OperationDraft {
            id: "workspace-rebalance-test",
            key: "rebalance:test",
            kind: "rebalance",
            preimage: "{}",
            intended: "{}",
            revisions: &revisions,
            now: "now",
        },
    )?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec![],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let phase: String = conn.query_row(
        "SELECT phase FROM workspace_operations WHERE id = 'workspace-rebalance-test'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(phase, "prepared");
    Ok(())
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-prepared-operation-startup-{}",
        std::process::id(),
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
