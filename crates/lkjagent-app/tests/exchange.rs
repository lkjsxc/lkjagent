use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use lkjagent_app::daemon::{run_until_idle, CompletionRecord, Endpoint, ScriptedEndpoint};
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn endpoint_completion_writes_exchange_and_usage_rows() -> TestResult<()> {
    let data = fixture_root("exchange")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);

    let mut endpoint = UsageEndpoint;
    run_until_idle(&data, &mut endpoint, 3)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let exchange_ref: String = conn.query_row(
        "SELECT exchange_ref FROM attempts ORDER BY id LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(exchange_ref, "logs/task-1/step-1/attempt-1");
    let request = fs::read_to_string(data.join(&exchange_ref).join("request.json"))?;
    assert!(request.contains("decision_id"));
    assert!(request.contains("timeout_seconds"));
    assert!(
        fs::read_to_string(data.join(&exchange_ref).join("response.json"))?
            .contains("StopSequenceClosed")
    );
    let provider: (String, String, i64) = conn.query_row(
        "SELECT decision_id, exchange_ref, timeout_seconds FROM provider_exchanges LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    assert!(provider.0.starts_with("case-1-decision-"));
    assert_eq!(provider.1, exchange_ref);
    assert_eq!(provider.2, 900);
    let frames: i64 = conn.query_row(
        "SELECT COUNT(*) FROM prompt_frames WHERE decision_id = ?1",
        [provider.0],
        |row| row.get(0),
    )?;
    assert_eq!(frames, 1);
    let usage: (i64, i64) = conn.query_row(
        "SELECT prompt_tokens, completion_tokens FROM token_usage LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(usage, (13, 7));
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("task in=13 out=7 cached=3"));
    Ok(())
}

#[test]
fn missing_endpoint_usage_gets_unknown_token_row() -> TestResult<()> {
    let data = fixture_root("unknown-usage")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>done</message>".to_string()],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 3)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let unknown: i64 = conn.query_row(
        "SELECT COUNT(*) FROM token_usage WHERE prompt_tokens IS NULL",
        [],
        |row| row.get(0),
    )?;
    assert!(unknown > 0);
    Ok(())
}

struct UsageEndpoint;

impl Endpoint for UsageEndpoint {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        Ok(CompletionRecord {
            content: "<message>done</message>".to_string(),
            prompt_tokens: Some(13),
            completion_tokens: Some(7),
            cached_tokens: Some(3),
            finish_reason: "Stop".to_string(),
            closure_mode: "StopSequenceClosed".to_string(),
            cache_metrics: vec![("cache".to_string(), "hit".to_string())],
            anomaly: None,
        })
    }

    fn timeout_seconds(&self) -> Option<u64> {
        Some(900)
    }
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-exchange-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
