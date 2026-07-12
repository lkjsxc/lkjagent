use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint};
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn default_explore_prompt_hides_shell_and_finish_tools() -> TestResult<()> {
    let data = fixture_root("default-explore-view")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Survey the workspace and report.", "now")?;
    drop(conn);

    let mut endpoint = CapturingEndpoint {
        output: "<final><message>done</message></final>".to_string(),
        prompts: Vec::new(),
    };
    let _snapshot = run_until_idle(&data, &mut endpoint, 1)?;
    let prompt = endpoint
        .prompts
        .first()
        .map(|prompt| prompt.user.as_str())
        .unwrap_or("");

    assert!(prompt.contains("fs.read"));
    assert!(!prompt.contains("finish"));
    assert!(!prompt.contains("shell.run"));
    assert!(!prompt.contains("run a bounded shell command"));
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

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-tools-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
