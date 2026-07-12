use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_app::clock::FixedClock;
use lkjagent_app::daemon::{run_until_idle_with_clock, CompletionRecord, Endpoint};
use lkjagent_core::model::TaskState;
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

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
    support::retain_workspace_config(&data)?;
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
    assert!(payload.contains("endpoint configuration fingerprint change"));
    assert!(payload.contains("\"operation_key\":\"runtime.wait\""));
    assert_eq!(decisions, 3);
    let blocked: i64 = conn.query_row(
        "SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(blocked, 0);
    drop(conn);

    fs::write(
        data.join("lkjagent.json"),
        r#"{
        "endpoint_timeout_seconds":301,
        "endpoint_retry_limit":2,
        "endpoint_backoff_milliseconds":50,
        "queue_wake_milliseconds":50
    }"#,
    )?;
    support::retain_workspace_config(&data)?;
    let mut clock = FixedClock::new("2026-07-11T10:00:01Z");
    run_until_idle_with_clock(&data, &mut endpoint, 4, &mut clock)?;
    assert_eq!(endpoint.calls, 4);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let (released, active): (i64, i64) = conn.query_row("SELECT
        (SELECT COUNT(*) FROM state_cells WHERE status = 'Suppressed' AND source_event_id IN
            (SELECT id FROM runtime_events WHERE source = 'endpoint-recovery')),
        (SELECT COUNT(*) FROM state_cells WHERE status = 'Active' AND payload_schema = 'recovery.failure'
            AND json_extract(payload_json, '$.next_strategy') = 'wait-external')",
        [], |row| Ok((row.get(0)?, row.get(1)?)))?;
    assert_eq!((released, active), (1, 1));
    Ok(())
}

#[test]
fn interrupted_sent_request_blocks_without_a_second_provider_call() -> TestResult<()> {
    let data = fixture_root()?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Do not repeat an ambiguous call", "before")?;
    drop(conn);
    let crash = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut endpoint = PanicEndpoint;
        let mut clock = FixedClock::new("2026-07-11T11:00:00Z");
        let _ = run_until_idle_with_clock(&data, &mut endpoint, 1, &mut clock);
    }));
    assert!(crash.is_err());
    let mut endpoint = CountingSuccess { calls: 0 };
    let mut clock = FixedClock::new("2026-07-11T11:00:01Z");
    let snapshot = run_until_idle_with_clock(&data, &mut endpoint, 2, &mut clock)?;
    assert_eq!(endpoint.calls, 0);
    assert_eq!(snapshot.task.state, TaskState::Blocked);
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let evidence: (i64, i64, i64, i64, i64) = conn.query_row(
        "SELECT
        (SELECT COUNT(*) FROM runtime_decisions WHERE status = 'interrupted'),
        (SELECT COUNT(*) FROM provider_exchanges WHERE finished_at IS NULL
            AND json_extract(outcome_json, '$.state') = 'dispatching'),
        (SELECT COUNT(*) FROM state_cells WHERE payload_schema = 'completion.blocked'),
        (SELECT COUNT(*) FROM state_cells WHERE status = 'Suppressed' AND source_event_id IN
            (SELECT id FROM runtime_events WHERE source = 'provider-recovery'
                AND kind = 'state.cell.suppress')),
        (SELECT COUNT(*) FROM provider_exchanges AS exchange
            JOIN runtime_decisions AS decision ON decision.id = exchange.decision_id
            JOIN prompt_frames AS prompt ON prompt.decision_id = decision.id
            WHERE decision.status = 'interrupted' AND prompt.prompt_fingerprint =
                json_extract(exchange.outcome_json, '$.prompt_fingerprint'))",
        [],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        },
    )?;
    assert_eq!(evidence, (1, 1, 1, 1, 1));
    Ok(())
}

struct PanicEndpoint;
impl Endpoint for PanicEndpoint {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        std::panic::resume_unwind(Box::new(
            "simulated process loss after durable request intent",
        ))
    }
}

struct CountingSuccess {
    calls: usize,
}
impl Endpoint for CountingSuccess {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        self.calls += 1;
        Ok(CompletionRecord::scripted(
            "<final><message>unexpected</message></final>".to_string(),
        ))
    }
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
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let path = std::env::temp_dir().join(format!(
        "lkjagent-endpoint-wait-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
