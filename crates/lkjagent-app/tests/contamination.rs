use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, TaskSnapshot};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_catalog::shell_tool_view;
use lkjagent_store::decision_rows::insert_runtime_decision;
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

mod support;
use support::{action_chars, shell_action};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn shell_observation_is_external_raw_context() -> TestResult<()> {
    let data = fixture_root("shell-raw")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Survey the workspace and report.");
    snapshot.steps[0].kind = StepKind::Explore;
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "now")?;
    insert_runtime_decision(&conn, &shell_decision(&snapshot), "pending", "now")?;
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
    fs::create_dir_all(data.join("workspace"))?;
    fs::write(data.join("workspace/secret.txt"), "token=abc123")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![action_chars("fs.read", &[('p', "secret.txt")])],
        index: 0,
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (String, String) = conn.query_row(
        "SELECT contamination_class, body FROM context_items
         WHERE semantic_key = 'observation/fs.read'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(row.0, "SensitiveOwnerData");
    assert!(row.1.contains("[sensitive owner data redacted]"));
    assert!(!row.1.contains("abc123"));
    Ok(())
}

fn shell_decision(snapshot: &TaskSnapshot) -> RuntimeDecision {
    RuntimeDecision::new(
        "shell-decision",
        "1",
        OperationKey(format!("model.call/{}", snapshot.steps[0].id)),
        shell_tool_view(),
        OutputEnvelope::Action,
    )
}

fn persist(conn: &mut Connection, snapshot: &TaskSnapshot) -> TestResult<()> {
    insert_task(conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now")?;
    }
    tx.commit()?;
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
