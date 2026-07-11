use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepState, TaskSnapshot, TaskState};
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

mod support;

use support::action_pairs;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn recovery_ladder_records_failure_cells() -> TestResult<()> {
    assert_failure_cell("parse", run_parse_failure)?;
    assert_failure_cell("endpoint", run_endpoint_failure)?;
    assert_failure_cell("admission", run_admission_failure)?;
    assert_failure_cell("effect", run_effect_failure)?;
    assert_failure_cell("check", run_check_failure)?;
    Ok(())
}

#[test]
fn repeated_parse_failure_advances_without_premature_block() -> TestResult<()> {
    let data = fixture_root("parse-repeat")?;
    enqueue_case(&data, "Investigate workspace files")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            "<message>not an action</message>".to_string(),
            "<message>not an action</message>".to_string(),
        ],
        index: 0,
    };

    let snapshot = run_until_idle(&data, &mut endpoint, 4)?;

    assert_eq!(snapshot.task.state, TaskState::Open);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let (repair, example, blocked): (i64, i64, i64) = conn.query_row(
        "SELECT COUNT(*) FILTER (WHERE json_extract(payload_json, '$.next_strategy') = 'grammar-repair'),
         COUNT(*) FILTER (WHERE json_extract(payload_json, '$.next_strategy') = 'concrete-example'),
         COUNT(*) FILTER (WHERE payload_schema = 'completion.blocked') FROM state_cells",
        [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    assert_eq!((repair, example, blocked), (1, 1, 0));
    Ok(())
}

#[test]
fn recovery_failure_selects_a_changed_strategy_before_more_model_work() -> TestResult<()> {
    let data = fixture_root("parse-selected")?;
    enqueue_case(&data, "Investigate workspace files")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>not an action</message>".to_string()],
        index: 0,
    };

    run_until_idle(&data, &mut endpoint, 2)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let (operation, policy, json): (String, String, String) = conn.query_row(
        "SELECT operation_key, recovery_policy, decision_json FROM runtime_decisions
         WHERE recovery_policy = 'grammar-repair' LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert!(operation.starts_with("model.call/"));
    assert_eq!(policy, "grammar-repair");
    assert!(json.contains("\"harness_state\":\"recover\""));
    Ok(())
}

fn run_parse_failure(data: &Path) -> TestResult<()> {
    enqueue_case(data, "Investigate workspace files")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>not an action</message>".to_string()],
        index: 0,
    };
    run_until_idle(data, &mut endpoint, 1)?;
    Ok(())
}

fn run_endpoint_failure(data: &Path) -> TestResult<()> {
    enqueue_case(data, "Investigate workspace files")?;
    let mut endpoint = FailingEndpoint;
    run_until_idle(data, &mut endpoint, 1)?;
    Ok(())
}

fn run_admission_failure(data: &Path) -> TestResult<()> {
    enqueue_case(data, "Investigate workspace files")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![action_pairs("fs.read", &[("path", "../secret")])],
        index: 0,
    };
    run_until_idle(data, &mut endpoint, 1)?;
    Ok(())
}

fn run_effect_failure(data: &Path) -> TestResult<()> {
    enqueue_case(data, "Investigate workspace files")?;
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![action_pairs("fs.read", &[("path", "missing.md")])],
        index: 0,
    };
    run_until_idle(data, &mut endpoint, 1)?;
    Ok(())
}

fn run_check_failure(data: &Path) -> TestResult<()> {
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Write notes/out.md with setup notes.");
    snapshot.steps[0].state = StepState::Done;
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: Vec::new(),
        index: 0,
    };
    run_until_idle(data, &mut endpoint, 1)?;
    Ok(())
}

struct FailingEndpoint;

impl Endpoint for FailingEndpoint {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        Err("endpoint down".to_string())
    }
}

#[rustfmt::skip]
fn assert_failure_cell(kind: &str, runner: fn(&Path) -> TestResult<()>) -> TestResult<()> {
    let data = fixture_root(kind)?; runner(&data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let pattern = format!("recovery:{kind}/%");
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM state_cells
        WHERE key_label LIKE ?1 AND payload_schema = 'recovery.failure' AND status = 'Active'",
        [pattern], |row| row.get(0))?;
    assert_eq!(count, 1, "kind={kind}");
    let payload: String = conn.query_row("SELECT payload_json FROM state_cells
        WHERE payload_schema = 'recovery.failure' LIMIT 1", [], |row| row.get(0))?;
    let value: serde_json::Value = serde_json::from_str(&payload)?;
    for field in ["normalized_signature", "operation", "prompt_fingerprint", "state_vector_fingerprint",
        "context_fingerprint", "tool_view_fingerprint", "budget_fingerprint", "attempted_strategy",
        "changed_condition", "diagnostic", "retry_count", "next_strategy", "remaining_budget", "tuple_fingerprint"] {
        assert!(value.get(field).is_some(), "kind={kind} field={field}");
    }
    Ok(())
}

fn enqueue_case(data: &Path, objective: &str) -> TestResult<()> {
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    Ok(())
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
        "lkjagent-recovery-ladder-{name}-{}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
