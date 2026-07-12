use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_app::turn_effects::{gather_checks, settle_check_effects};
use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::TurnOutcome;
use lkjagent_core::model::{CheckSpec, StepKind};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::{insert_case, upsert_state_cell};
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
type JournalRow = (String, String, String, String, String);

#[test]
#[rustfmt::skip]
fn late_turn_failure_rolls_back_shell_observation_and_check() -> TestResult<()> {
    let data = fixture_root("atomic-shell")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?; setup(&conn)?;
    let mut snapshot = instantiate(3, "run one atomic command check");
    let step = snapshot.steps.first_mut().ok_or("missing step")?;
    step.kind = StepKind::Verify;
    step.checks = vec![CheckSpec::Command { cmd: "printf atomic-shell".to_string() }];
    insert_task(&conn, &snapshot.task, None, "now")?;
    let tx = conn.transaction()?;
    for step in &snapshot.steps { insert_step_tx(&tx, step, "now")?; }
    tx.commit()?; insert_case(&conn, "3", &snapshot.task.objective, "now")?;
    let key = lkjagent_core::runtime_state::StateKey::new("matter", "snapshot/3")
        .map_err(|error| std::io::Error::other(error.message))?;
    let mut cell = lkjagent_core::runtime_state::StateCell::active(key, "snapshot-event");
    cell.payload_schema = "matter-snapshot".to_string(); cell.payload_json = serde_json::to_string(&snapshot)?;
    cell.created_at = "now".to_string(); cell.updated_at = "now".to_string();
    upsert_state_cell(&conn, "3", &cell)?;
    conn.execute_batch("CREATE TRIGGER fail_shell_turn BEFORE UPDATE OF status ON runtime_decisions
        WHEN NEW.status <> OLD.status BEGIN SELECT RAISE(FAIL, 'forced shell settlement failure'); END;")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint { outputs: Vec::new(), index: 0 };
    let error = run_until_idle(&data, &mut endpoint, 1)
        .err().ok_or("shell turn unexpectedly settled")?;
    assert!(error.contains("forced shell settlement failure"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let state: String = conn.query_row("SELECT state FROM effect_journal", [], |row| row.get(0))?;
    let decision: String = conn.query_row("SELECT status FROM runtime_decisions", [], |row| row.get(0))?;
    let observations: i64 = conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    let checks: i64 = conn.query_row("SELECT COUNT(*) FROM check_results", [], |row| row.get(0))?;
    assert_eq!((state.as_str(), decision.as_str(), observations, checks), ("applying", "pending", 0, 0));
    Ok(())
}

#[test]
fn shell_checks_have_prepared_journals_and_bounded_observations() -> TestResult<()> {
    let workspace = fixture_root("settled")?;
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "observe command checks");
    let step = snapshot
        .steps
        .first_mut()
        .ok_or_else(|| std::io::Error::other("missing step"))?;
    step.checks = vec![
        CheckSpec::Command {
            cmd: "printf check-ok".to_string(),
        },
        CheckSpec::Command {
            cmd: "printf check-fail; exit 7".to_string(),
        },
        CheckSpec::Command {
            cmd: "printf '{\"token\":\"secret\"}'".to_string(),
        },
        CheckSpec::Command {
            cmd: "printf 'Authorization: Bearer secret'".to_string(),
        },
    ];
    let step_id = step.id;

    let gathered = gather_checks(
        &mut conn,
        &workspace,
        &snapshot,
        step_id,
        &decision("settled"),
        "now",
    )?;
    let applying: i64 = conn.query_row(
        "SELECT COUNT(*) FROM effect_journal WHERE state = 'applying'",
        [],
        |row| row.get(0),
    )?;
    let observations: i64 =
        conn.query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))?;
    assert_eq!((applying, observations), (4, 0));
    settle_check_effects(&conn, &gathered.effects)?;
    let TurnOutcome::Checks(_, facts) = gathered.outcome else {
        return Err("checks did not produce facts".into());
    };
    assert_eq!(
        facts.iter().map(|fact| fact.success).collect::<Vec<_>>(),
        [true, false, true, true]
    );
    let rows = rows(&conn)?;
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|row| row.0 == "shell.run"));
    assert!(rows.iter().all(|row| row.1 == "committed"));
    assert!(rows.iter().all(|row| row.2 == "ok"));
    assert!(rows[0].3.contains("exit_code=Some(0)"));
    assert!(rows[1].3.contains("exit_code=Some(7)"));
    assert_eq!(rows[0].4, "ExternalRaw");
    assert_eq!(rows[2].4, "SensitiveOwnerData");
    assert_eq!(rows[3].4, "SensitiveOwnerData");
    assert!(rows[2].3.contains("[sensitive owner data redacted]"));
    assert!(rows[3].3.contains("[sensitive owner data redacted]"));
    assert!(rows.iter().all(|row| row.3.len() <= 4_000));
    Ok(())
}

#[test]
fn invalid_shell_check_records_failed_fact_and_journal() -> TestResult<()> {
    let workspace = fixture_root("invalid")?;
    let mut conn = Connection::open_in_memory()?;
    setup(&conn)?;
    let mut snapshot = instantiate(2, "observe invalid command check");
    let step = snapshot
        .steps
        .first_mut()
        .ok_or_else(|| std::io::Error::other("missing step"))?;
    step.checks = vec![CheckSpec::Command {
        cmd: "   ".to_string(),
    }];
    let step_id = step.id;

    let gathered = gather_checks(
        &mut conn,
        &workspace,
        &snapshot,
        step_id,
        &decision("invalid"),
        "now",
    )?;
    settle_check_effects(&conn, &gathered.effects)?;
    let TurnOutcome::Checks(_, facts) = gathered.outcome else {
        return Err("invalid check did not produce a fact".into());
    };
    assert_eq!(facts.len(), 1);
    assert!(!facts[0].success);
    let rows = rows(&conn)?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].1, "failed");
    assert_eq!(rows[0].2, "error");
    assert_eq!(rows[0].4, "RecoveryOnly");
    Ok(())
}

fn rows(conn: &Connection) -> TestResult<Vec<JournalRow>> {
    let mut statement = conn.prepare(
        "SELECT admissions.action_tool, journal.state, observations.status,
                observations.content, observations.contamination_class
         FROM effect_journal AS journal JOIN tool_admissions AS admissions
         ON admissions.id = journal.admission_id JOIN observations
         ON observations.id = journal.observation_id ORDER BY journal.command_ordinal",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn decision(suffix: &str) -> RuntimeDecision {
    RuntimeDecision::new(
        format!("shell-check-{suffix}"),
        "case-1",
        OperationKey(format!("check/{suffix}")),
        ToolSetView::new(Vec::new()),
        OutputEnvelope::None,
    )
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-shell-check-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
