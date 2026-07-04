use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint};
use lkjagent_core::classify::instantiate;
use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::{ContaminationClass, ContextItem};
use lkjagent_store::context_rows::insert_context_item;
use lkjagent_store::plan_access::{insert_step_tx, insert_task};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::state_rows::insert_case;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn context_prompt_excludes_contaminated_and_creates_conflict_cell() -> TestResult<()> {
    let data = fixture_root("hygiene")?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    let snapshot = instantiate(1, "What is the target root?");
    persist(&mut conn, &snapshot)?;
    insert_case(&conn, "1", &snapshot.task.objective, "before")?;
    insert_context_item(&conn, "1", &clean("ctx-a", "target-root", "root-a"))?;
    insert_context_item(&conn, "1", &clean("ctx-b", "target-root", "root-b"))?;
    insert_context_item(&conn, "1", &contaminated("ctx-bad", "bad-pattern"))?;
    drop(conn);

    let mut endpoint = CapturingEndpoint {
        output: "<message>done</message>".to_string(),
        prompts: Vec::new(),
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;

    let prompt = endpoint
        .prompts
        .first()
        .map(|prompt| format!("{}\n{}", prompt.system, prompt.user))
        .unwrap_or_default();
    assert!(prompt.contains("Unresolved conflict target-root"));
    assert!(!prompt.contains("bad-pattern"));
    assert!(!prompt.contains("root-a"));
    assert!(!prompt.contains("root-b"));
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE key_label = 'context:conflict/target-root'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 1);
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

fn clean(id: &str, key: &str, body: &str) -> ContextItem {
    let mut item = ContextItem::clean_fact(id, key, body);
    item.source_type = "test".to_string();
    item.source_id = id.to_string();
    item.created_at = "before".to_string();
    item
}

fn contaminated(id: &str, body: &str) -> ContextItem {
    let mut item = clean(id, "model-output", body);
    item.contamination = ContaminationClass::FailedModelOutput;
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
    let path = std::env::temp_dir().join(format!("lkjagent-context-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
