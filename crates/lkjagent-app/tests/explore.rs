use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint, ScriptedEndpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{StepKind, TaskSnapshot, TaskState};
use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_store::decision_rows::insert_runtime_decision;
use lkjagent_store::plan_access::{enqueue, insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

mod support;
use support::{action_chars, action_for, action_pairs};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn explore_registry_runs_small_workspace_view() -> TestResult<()> {
    let data = fixture_root("registry")?;
    fs::create_dir_all(data.join("workspace"))?;
    fs::write(data.join("workspace/probe.txt"), "hello release")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            action_chars("fs.read", &[('p', "probe.txt")]),
            action_chars("fs.search", &[('q', "release")]),
            action_pairs("plan.note", &[("note", "probe reviewed")]),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 3)?;
    assert_eq!(snapshot.task.state, TaskState::Open);
    assert!(snapshot.steps[0].inputs.contains("<observation>"));
    assert!(snapshot.steps[0].inputs.contains("noted: probe reviewed"));
    Ok(())
}

#[test]
fn prompt_view_and_admission_share_persisted_tool_fingerprint() -> TestResult<()> {
    let data = fixture_root("view-admission")?;
    fs::create_dir_all(data.join("workspace"))?;
    fs::write(data.join("workspace/probe.txt"), "hello")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let mut snapshot = instantiate(1, "Survey workspace and report.");
    snapshot.steps[0].kind = StepKind::Explore;
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    let decision = restricted_read_decision(&snapshot);
    let expected_fp = decision.tool_view_fingerprint().unwrap_or_default();
    insert_runtime_decision(&conn, &decision, "pending", "before")?;
    drop(conn);

    let mut endpoint = CapturingEndpoint {
        output: action_for("decision-view", "", "fs.read", &[("path", "probe.txt")]),
        prompts: Vec::new(),
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    let prompt = endpoint
        .prompts
        .first()
        .map(|prompt| prompt.user.as_str())
        .unwrap_or("");
    assert!(prompt.contains("fs.read"));
    assert!(!prompt.contains("shell.run"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let row: (i64, String, String, String) = conn.query_row(
        "SELECT COUNT(*), action_tool, status, tool_view_fingerprint FROM tool_admissions",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    assert_eq!(row.0, 1);
    assert_eq!(row.1, "fs.read");
    assert_eq!(row.2, "Admitted");
    assert_eq!(row.3, expected_fp);
    let observation: (i64, String, String) = conn.query_row(
        "SELECT COUNT(*), effect_name, status FROM observations",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert_eq!(observation.0, 1);
    assert_eq!(observation.1, "fs.read");
    assert_eq!(observation.2, "ok");
    let context: (String, String) = conn.query_row(
        "SELECT semantic_key, contamination_class FROM context_items
         WHERE semantic_key = 'observation/fs.read'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(context.0, "observation/fs.read");
    assert_eq!(context.1, "Clean");
    Ok(())
}

#[test]
fn memory_find_reads_durable_rows() -> TestResult<()> {
    let data = fixture_root("memory-find")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    conn.execute(
        "INSERT INTO memory (topic, content, created_at) VALUES ('probe', 'hello release', 'now')",
        [],
    )?;
    enqueue(&conn, "Survey memory and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![action_chars("memory.find", &[('q', "release")])],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    assert_eq!(snapshot.task.state, TaskState::Open);
    assert!(snapshot.steps[0]
        .inputs
        .contains("memory 1 matter=none probe"));
    assert!(snapshot.steps[0].inputs.contains("hello release"));
    Ok(())
}

struct CapturingEndpoint {
    output: String,
    prompts: Vec<Prompt>,
}

impl Endpoint for CapturingEndpoint {
    fn complete(&mut self, prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        self.prompts.push(prompt.clone());
        Ok(CompletionRecord::scripted(self.output.clone()))
    }
}

fn restricted_read_decision(snapshot: &TaskSnapshot) -> RuntimeDecision {
    let step_id = snapshot.steps.first().map_or(0, |step| step.id);
    RuntimeDecision::new(
        "decision-view",
        "1",
        OperationKey(format!("model.call/{step_id}")),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
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
    let path = std::env::temp_dir().join(format!("lkjagent-explore-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
