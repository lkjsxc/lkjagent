use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;
use support::{action_chars, shell_action};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn shell_observation_is_external_raw_context() -> TestResult<()> {
    let data = fixture_root("shell-raw")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![shell_action("printf external_raw")],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let class: String = conn.query_row(
        "SELECT contamination_class FROM context_items
         WHERE semantic_key = 'observation/shell.run'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(class, "ExternalRaw");
    Ok(())
}

#[test]
fn sensitive_observation_body_is_redacted() -> TestResult<()> {
    let data = fixture_root("sensitive")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            action_chars("fs.write", &[('p', "secret.txt"), ('c', "token=abc123")]),
            action_chars("fs.read", &[('p', "secret.txt")]),
        ],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 2)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (String, String) = conn.query_row(
        "SELECT contamination_class, body FROM context_items
         WHERE semantic_key = 'observation/fs.read'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row.0, "SensitiveOwnerData");
    assert!(row.1.contains("token=[redacted]"));
    assert!(!row.1.contains("abc123"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-contamination-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
