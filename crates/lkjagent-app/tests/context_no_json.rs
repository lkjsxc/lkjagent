use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::ContextItem;
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn prompt_context_suppresses_json_like_context_bodies() -> TestResult<()> {
    let data = fixture_root("context-no-json")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What context is safe?");
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    insert_context_item(&conn, "1", &json_like_item())?;
    drop(conn);

    let mut endpoint = CapturingEndpoint {
        output: "<final><message>done</message></final>".to_string(),
        prompts: Vec::new(),
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    let prompt = endpoint
        .prompts
        .first()
        .map(|prompt| format!("{}\n{}", prompt.system, prompt.user))
        .unwrap_or_default();

    assert!(!prompt.contains("{\"tool\":\"shell.run\"}"));
    assert!(prompt.contains("json-like context suppressed"));
    assert!(prompt.contains("item=ctx-json"));
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

fn json_like_item() -> ContextItem {
    let mut item = ContextItem::clean_fact("ctx-json", "unsafe-json", "{\"tool\":\"shell.run\"}");
    item.source_type = "test".to_string();
    item.source_id = "ctx-json".to_string();
    item.source_fingerprint = "fp-json".to_string();
    item.created_at = "before".to_string();
    item
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
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
