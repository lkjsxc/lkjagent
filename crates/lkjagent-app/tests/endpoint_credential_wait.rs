use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lkjagent_app::clock::FixedClock;
use lkjagent_app::daemon::{run_until_idle_with_clock, CompletionRecord, Endpoint};
use lkjagent_core::render::Prompt;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
const CHILD_DATA: &str = "LKJAGENT_CREDENTIAL_CHILD_DATA";

#[test]
fn credential_rotation_wakes_once_without_persisting_the_secret() -> TestResult<()> {
    let data = fixture_root()?;
    fs::write(
        data.join("lkjagent.json"),
        r#"{
        "endpoint_api_key_env":"LKJAGENT_API_KEY",
        "endpoint_retry_limit":0,
        "endpoint_backoff_milliseconds":50,
        "queue_wake_milliseconds":50
    }"#,
    )?;
    support::retain_workspace_config(&data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "Recover after credential rotation", "before")?;
    drop(conn);

    child(&data, "credential-one", "2026-07-11T12:00:00Z")?;
    assert_eq!(exchange_count(&data)?, 1);
    child(&data, "credential-one", "2026-07-11T12:00:01Z")?;
    assert_eq!(exchange_count(&data)?, 1);
    child(&data, "credential-two", "2026-07-11T12:00:02Z")?;
    assert_eq!(exchange_count(&data)?, 2);

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let (released, active, token): (i64, i64, String) = conn.query_row("SELECT
        (SELECT COUNT(*) FROM state_cells WHERE status = 'Suppressed' AND source_event_id IN
            (SELECT id FROM runtime_events WHERE source = 'endpoint-recovery')),
        (SELECT COUNT(*) FROM state_cells WHERE status = 'Active' AND payload_schema = 'recovery.failure'
            AND json_extract(payload_json, '$.next_strategy') = 'wait-external'),
        (SELECT value FROM config WHERE key = 'runtime.endpoint_condition_fingerprint')",
        [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    assert_eq!((released, active), (1, 1));
    assert!(!token.contains("credential-one") && !token.contains("credential-two"));
    Ok(())
}

#[test]
fn credential_endpoint_child() -> TestResult<()> {
    let Ok(data) = std::env::var(CHILD_DATA) else {
        return Ok(());
    };
    let now = std::env::var("LKJAGENT_CREDENTIAL_CHILD_NOW")?;
    let mut endpoint = ChildFailure;
    let mut clock = FixedClock::new(&now);
    run_until_idle_with_clock(Path::new(&data), &mut endpoint, 4, &mut clock)?;
    Ok(())
}

fn child(data: &Path, credential: &str, now: &str) -> TestResult<()> {
    let status = Command::new(std::env::current_exe()?)
        .args(["--exact", "credential_endpoint_child", "--nocapture"])
        .env(CHILD_DATA, data)
        .env("LKJAGENT_CREDENTIAL_CHILD_NOW", now)
        .env("LKJAGENT_API_KEY", credential)
        .status()?;
    if !status.success() {
        return Err("credential child failed".into());
    }
    Ok(())
}

fn exchange_count(data: &Path) -> TestResult<i64> {
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    Ok(
        conn.query_row("SELECT COUNT(*) FROM provider_exchanges", [], |row| {
            row.get(0)
        })?,
    )
}

struct ChildFailure;
impl Endpoint for ChildFailure {
    fn complete(&mut self, _prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        Err("endpoint authentication failed request-id: credential-test".to_string())
    }
}

fn fixture_root() -> TestResult<PathBuf> {
    let path =
        std::env::temp_dir().join(format!("lkjagent-credential-wait-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}
