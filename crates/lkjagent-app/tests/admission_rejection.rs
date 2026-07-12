use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, TaskSnapshot, TaskState};
use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_catalog::tool_view_for_names;
use lkjagent_store::decision_rows::{insert_runtime_decision, unfinished_decisions};
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

mod support;
use support::action_for;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn rejected_protocol_call_persists_recovery_without_admission() -> TestResult<()> {
    let data = fixture_root("admission-rejection")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Survey workspace and report.");
    snapshot.steps[0].kind = StepKind::Explore;
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let decision = restricted_read_decision(&snapshot);
    assert!(decision.tool_view.has_current_constraints());
    insert_runtime_decision(&conn, &decision, "pending", "before")?;
    assert_eq!(unfinished_decisions(&conn, "1")?.len(), 1);
    drop(conn);

    let mut endpoint = OneShotEndpoint {
        output: action_for("decision-view", "", "fs.read", &[("path", "PATH")]),
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)
        .map_err(|error| format!("run until idle: {error}"))?;
    assert_eq!(snapshot.task.state, TaskState::Open);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    assert_eq!(count(&conn, "tool_admissions")?, 0);
    assert_eq!(count(&conn, "observations")?, 0);
    let status: String = conn
        .query_row(
            "SELECT status FROM runtime_decisions WHERE id = 'decision-view'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| format!("decision row: {error}"))?;
    assert_eq!(status, "settled");
    assert_eq!(recovery_count(&conn)?, 1);
    assert_eq!(count(&conn, "attempts")?, 1);
    assert_eq!(count(&conn, "provider_exchanges")?, 1);
    Ok(())
}

fn recovery_count(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM state_cells
         WHERE key_label LIKE 'recovery:parse/%' AND payload_schema = 'recovery.failure'",
        [],
        |row| row.get(0),
    )
}

struct OneShotEndpoint {
    output: String,
}

impl Endpoint for OneShotEndpoint {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        Ok(CompletionRecord::scripted(self.output.clone()))
    }
}

fn restricted_read_decision(snapshot: &TaskSnapshot) -> RuntimeDecision {
    let step_id = snapshot.steps.first().map_or(0, |step| step.id);
    RuntimeDecision::new(
        "decision-view",
        "1",
        OperationKey(format!("model.call/{step_id}")),
        tool_view_for_names(&["fs.read"]),
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

fn count(conn: &Connection, table: &str) -> rusqlite::Result<i64> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
        row.get(0)
    })
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
