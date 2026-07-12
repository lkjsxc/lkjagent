use std::{fs, path::PathBuf};

use lkjagent_app::turn_effects::{gather_checks, settle_check_effects};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::CheckSpec;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn file_preflight_failure_prevents_earlier_shell_execution() -> TestResult<()> {
    let workspace = fixture_root("files")?;
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let mut snapshot = instantiate(7, "preflight before command");
    let step = snapshot.steps.first_mut().ok_or("missing step")?;
    step.checks = vec![
        CheckSpec::Command {
            cmd: "printf ran > marker.txt".to_string(),
        },
        CheckSpec::FileExists {
            path: "../outside.txt".to_string(),
        },
    ];
    let step_id = step.id;
    let decision = RuntimeDecision::new(
        "preflight-decision",
        "case-7",
        OperationKey(format!("check.run/{step_id}")),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::Message,
    );
    let result = gather_checks(&mut conn, &workspace, &snapshot, step_id, &decision, "now");
    assert!(result.is_err());
    assert!(!workspace.join("marker.txt").exists());
    let journals: i64 =
        conn.query_row("SELECT COUNT(*) FROM effect_journal", [], |row| row.get(0))?;
    let observations: i64 =
        conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    assert_eq!((journals, observations), (0, 0));
    Ok(())
}

#[test]
fn direct_multi_check_settlement_rolls_back_earlier_rows() -> TestResult<()> {
    let workspace = fixture_root("settlement")?;
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let mut snapshot = instantiate(8, "settle two command checks");
    let step = snapshot.steps.first_mut().ok_or("missing step")?;
    step.checks = vec![
        CheckSpec::Command {
            cmd: "printf one".to_string(),
        },
        CheckSpec::Command {
            cmd: "printf two".to_string(),
        },
    ];
    let step_id = step.id;
    let decision = RuntimeDecision::new(
        "settlement-decision",
        "case-8",
        OperationKey(format!("check.run/{step_id}")),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::Message,
    );
    let gathered = gather_checks(&mut conn, &workspace, &snapshot, step_id, &decision, "now")?;
    conn.execute(
        "UPDATE effect_journal SET state = 'failed' WHERE command_ordinal = 2",
        [],
    )?;
    assert!(settle_check_effects(&conn, &gathered.effects).is_err());
    let observations: i64 =
        conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    let first: String = conn.query_row(
        "SELECT state FROM effect_journal WHERE command_ordinal = 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!((observations, first.as_str()), (0, "applying"));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-shell-preflight-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
