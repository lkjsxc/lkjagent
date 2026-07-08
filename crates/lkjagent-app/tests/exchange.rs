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
    assert_eq!(exchange_ref, "logs/matter-1/operation-1/attempt-1");
    let request = fs::read_to_string(data.join(&exchange_ref).join("request.json"))?;
    assert!(request.contains("decision_id"));
    assert!(request.contains("context_frame_fingerprint"));
    assert!(request.contains("timeout_seconds"));
    assert!(
        fs::read_to_string(data.join(&exchange_ref).join("response.json"))?
            .contains("StopSequenceClosed")
    );
    let provider: (String, String, String, i64) = conn.query_row(
        "SELECT decision_id, exchange_ref, context_frame_fingerprint, timeout_seconds
         FROM provider_exchanges LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    assert!(provider.0.starts_with("case-1-decision-"));
    assert_eq!(provider.1, exchange_ref);
    assert_eq!(provider.3, 900);
    let decision_fp: String = conn.query_row(
        "SELECT context_frame_fingerprint FROM runtime_decisions WHERE id = ?1",
        [&provider.0],
        |row| row.get(0),
    )?;
    assert_eq!(provider.2, decision_fp);
    let body_ref: String = conn.query_row(
        "SELECT body_ref FROM prompt_frames WHERE decision_id = ?1",
        [provider.0],
        |row| row.get(0),
    )?;
    assert!(body_ref.starts_with("logs/case-1/decision-"));
    let prompt_body = fs::read_to_string(data.join(&body_ref))?;
    assert!(prompt_body.contains(&provider.2));
    let usage: (i64, i64, i64, i64, String, String) = conn.query_row(
        "SELECT input_total_tokens, input_cached_tokens, input_uncached_tokens,
         output_tokens, cache_status, raw_usage_json FROM token_usage LIMIT 1",
        [],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        },
    )?;
    assert_eq!(
        (usage.0, usage.1, usage.2, usage.3, usage.4.as_str()),
        (13, 3, 10, 7, "known")
    );
    assert!(usage.5.contains("cache_status"));
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("input_uncached=10 input_cached=3 input_total=13 output=7 cache=known"));
    Ok(())
}

#[test]
fn parse_fault_exchange_creates_contaminated_context_item() -> TestResult<()> {
    let data = fixture_root("parse-fault-context")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<content>wrong</content>".to_string()],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 1)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let contamination: String = conn.query_row(
        "SELECT contamination_class FROM context_items
         WHERE source_type = 'provider_exchange' LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(contamination, "FailedModelOutput");
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
        "SELECT COUNT(*) FROM token_usage WHERE input_total_tokens IS NULL
         AND cache_status = 'unknown'",
        [],
        |row| row.get(0),
    )?;
    assert!(unknown > 0);
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "status"])?;
    assert!(status.contains("input_cached=unknown"));
    assert!(status.contains("cache=unknown"));
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
