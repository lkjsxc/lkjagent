use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
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

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn explore_registry_runs_bounded_workspace_tools() -> TestResult<()> {
    let data = fixture_root("registry")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            action("fs.write", &[('p', "probe.txt"), ('c', "hello aurora")]),
            action("fs.read", &[('p', "probe.txt")]),
            memory_save("probe", "hello aurora"),
            finish("read probe"),
            "<message>done</message>".to_string(),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 6)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(data.join("workspace/probe.txt").exists());
    assert!(snapshot.steps[0].inputs.contains("<observation>"));
    assert!(snapshot.steps[0].inputs.contains("saved topic=probe"));
    assert!(cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "memory",
        "aurora"
    ])?
    .contains("hello aurora"));
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
        output: action("fs.read", &[('p', "probe.txt")]),
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
    Ok(())
}

#[test]
fn memory_find_reads_durable_rows() -> TestResult<()> {
    let data = fixture_root("memory-find")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey memory and report.", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec![
            memory_save("probe", "hello aurora"),
            action("memory.find", &[('q', "aurora")]),
        ],
        index: 0,
    };
    let snapshot = run_until_idle(&data, &mut endpoint, 2)?;
    assert_eq!(snapshot.task.state, TaskState::Open);
    assert!(snapshot.steps[0].inputs.contains("memory 1 task=1 probe"));
    assert!(snapshot.steps[0].inputs.contains("hello aurora"));
    Ok(())
}

fn action(tool: &str, params: &[(char, &str)]) -> String {
    let mut body = format!("<tool>{tool}</tool>");
    for (kind, value) in params {
        let name = match *kind {
            'p' => "path",
            'q' => "query",
            _ => "content",
        };
        body.push_str(&format!("<{name}>{value}</{name}>"));
    }
    format!("<action>{body}</action>")
}

fn memory_save(topic: &str, content: &str) -> String {
    format!(
        "<action><tool>memory.save</tool><topic>{topic}</topic><content>{content}</content></action>"
    )
}

fn finish(summary: &str) -> String {
    format!("<action><tool>finish</tool><summary>{summary}</summary></action>")
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
