use std::fs;
use std::path::PathBuf;

use lkjagent_app::clock::FixedClock;
use lkjagent_app::daemon::{run_until_idle_with_clock, CompletionRecord, Endpoint};
use lkjagent_core::model::TaskState;
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn endpoint_retries_wait_until_due_then_stop_at_configured_limit() -> TestResult<()> {
    let data = fixture_root()?;
    fs::write(
        data.join("lkjagent.json"),
        r#"{
        "endpoint_retry_limit":2,
        "endpoint_backoff_milliseconds":50,
        "queue_wake_milliseconds":50
    }"#,
    )?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "before")?;
    drop(conn);
    let mut endpoint = CountingFailure { calls: 0 };

    for (now, calls) in [
        ("2026-07-11T10:00:00Z", 1),
        ("2026-07-11T10:00:00.049Z", 1),
        ("2026-07-11T10:00:00.050Z", 2),
        ("2026-07-11T10:00:00.149Z", 2),
        ("2026-07-11T10:00:00.150Z", 3),
        ("2026-07-11T10:00:00.999Z", 3),
    ] {
        let mut clock = FixedClock::new(now);
        let snapshot = run_until_idle_with_clock(&data, &mut endpoint, 4, &mut clock)?;
        assert_eq!(snapshot.task.state, TaskState::Open);
        assert_eq!(endpoint.calls, calls, "now={now}");
    }
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let (payload, decisions): (String, i64) = conn.query_row(
        "SELECT payload_json, (SELECT COUNT(*) FROM runtime_decisions)
         FROM state_cells WHERE payload_schema = 'recovery.failure'
         AND json_extract(payload_json, '$.next_strategy') = 'wait-external'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert!(payload.contains("endpoint configuration or owner retry"));
    assert!(payload.contains("\"operation_key\":\"runtime.wait\""));
    assert_eq!(decisions, 3);
    let blocked: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(blocked, 0);
    Ok(())
}

struct CountingFailure {
    calls: usize,
}

impl Endpoint for CountingFailure {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        self.calls += 1;
        Err(format!(
            "endpoint unavailable request-id: opaque{}",
            self.calls
        ))
    }
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-endpoint-wait-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
