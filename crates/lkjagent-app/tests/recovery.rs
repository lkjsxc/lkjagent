use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};
use lkjagent_store::decision_rows::{insert_runtime_decision, unfinished_decisions};
use lkjagent_store::exchange_rows::{insert_provider_exchange, ProviderExchangeRow};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn unfinished_decision_with_exchange_blocks_without_provider_replay() -> TestResult<()> {
    let data = fixture_root("exchange-recovery")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What is an agent?");
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let empty = pending_decision(&snapshot, "decision-empty");
    insert_runtime_decision(&conn, &empty, "pending", "before-1")?;
    let old = pending_decision(&snapshot, "decision-old");
    insert_runtime_decision(&conn, &old, "pending", "before-2")?;
    insert_provider_exchange(&conn, &exchange_row(&old))?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<final><message>done</message></final>".to_string()],
        index: 0,
    };
    let blocked = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(blocked.task.state, lkjagent_core::model::TaskState::Blocked);
    assert_eq!(endpoint.index, 0);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let old_status: String = conn.query_row(
        "SELECT status FROM runtime_decisions WHERE id = 'decision-old'",
        [],
        |row| row.get(0),
    )?;
    let pending: i64 = conn.query_row(
        "SELECT COUNT(*) FROM runtime_decisions WHERE status = 'pending'",
        [],
        |row| row.get(0),
    )?;
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM runtime_decisions", [], |row| {
        row.get(0)
    })?;
    assert_eq!(old_status, "interrupted");
    assert_eq!(pending, 0);
    assert_eq!(total, 3);
    Ok(())
}

#[test]
fn pending_decision_with_stale_field_bounds_fails_closed() -> TestResult<()> {
    let data = fixture_root("stale-field-bounds")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What is an agent?");
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let mut decision = pending_decision(&snapshot, "decision-stale");
    decision.tool_view = lkjagent_core::runtime_tool_catalog::tool_view_for_names(&["fs.read"]);
    decision.tool_view.entries[0]
        .field_specs
        .iter_mut()
        .find(|spec| spec.name == "count")
        .ok_or("count field missing")?
        .maximum = None;
    insert_runtime_decision(&conn, &decision, "pending", "before")?;
    let error = match unfinished_decisions(&conn, "1") {
        Ok(_) => return Err("stale bounds accepted".into()),
        Err(error) => error,
    };
    assert!(error.to_string().contains("tool constraints are stale"));
    Ok(())
}

fn pending_decision(snapshot: &TaskSnapshot, id: &str) -> RuntimeDecision {
    let step_id = snapshot.steps.first().map_or(0, |step| step.id);
    RuntimeDecision::new(
        id,
        "1",
        OperationKey(format!("model.call/{step_id}")),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    )
}

fn exchange_row(decision: &RuntimeDecision) -> ProviderExchangeRow {
    ProviderExchangeRow {
        id: "exchange-old".to_string(),
        case_id: decision.case_id.clone(),
        decision_id: decision.id.clone(),
        exchange_ref: "logs/matter-1/operation-1/attempt-1".to_string(),
        outcome_json: "{\"outcome\":\"parsed\"}".to_string(),
        context_frame_fingerprint: decision.context_frame_fingerprint.clone(),
        timeout_seconds: Some(900),
        started_at: "before".to_string(),
        finished_at: Some("before".to_string()),
    }
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
    let path =
        std::env::temp_dir().join(format!("lkjagent-recovery-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
