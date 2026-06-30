mod support;

use lkjagent_store::token_usage::{
    aggregate_all, aggregate_latest, aggregate_session, aggregate_task, latest, record,
    TokenUsageEvent,
};
use support::{memory_store, TestResult};

#[test]
fn token_usage_ledger_preserves_known_and_unknown_values() -> TestResult<()> {
    let conn = memory_store()?;
    record_full(&conn, Some(42), 7, "2026-06-20T00:00:00Z")?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: None,
            turn: 8,
            input_tokens: None,
            output_tokens: Some(3),
            cached_input_tokens: None,
            total_tokens: None,
            context_window: None,
            context_used_estimate: None,
            source: "endpoint".to_string(),
        },
        "2026-06-20T00:00:01Z",
    )?;

    let row = latest(&conn)?.ok_or("latest token usage row")?;

    assert_eq!(row.turn, 8);
    assert_eq!(row.input_tokens, None);
    assert_eq!(row.output_tokens, Some(3));
    assert_eq!(row.total_tokens, None);
    Ok(())
}

#[test]
fn token_usage_aggregates_scopes_and_unknown_fields() -> TestResult<()> {
    let conn = memory_store()?;
    record_full(&conn, Some(42), 1, "2026-06-20T00:00:00Z")?;
    record_partial(&conn, Some(42), 2, "2026-06-20T00:00:01Z")?;
    record_full(&conn, Some(7), 3, "2026-06-20T00:00:02Z")?;

    let task = aggregate_task(&conn, 42)?;
    let all = aggregate_all(&conn)?;
    let latest = aggregate_latest(&conn)?;

    assert_eq!(task.rows, 2);
    assert_eq!(task.rows_with_unknown, 1);
    assert_eq!(task.input_tokens.sum, 8_120);
    assert_eq!(task.input_tokens.unknown, 1);
    assert_eq!(task.output_tokens.sum, 1_043);
    assert_eq!(task.output_tokens.unknown, 0);
    assert_eq!(task.cached_input_tokens.sum, 6_880);
    assert_eq!(task.cached_input_tokens.unknown, 1);
    assert_eq!(all.rows, 3);
    assert_eq!(latest.rows, 1);
    assert_eq!(latest.total_tokens.sum, 9_160);
    Ok(())
}

#[test]
fn token_usage_session_uses_daemon_lock_start() -> TestResult<()> {
    let conn = memory_store()?;
    record_full(&conn, Some(1), 1, "2026-06-20T00:00:00Z")?;
    lkjagent_store::state::set(
        &conn,
        "daemon lock",
        "pid1|2026-06-20T00:00:01Z|2026-06-20T00:00:03Z",
    )?;
    record_partial(&conn, Some(1), 2, "2026-06-20T00:00:01Z")?;
    record_full(&conn, Some(1), 3, "2026-06-20T00:00:02Z")?;

    let session = aggregate_session(&conn)?;

    assert_eq!(session.rows, 2);
    assert_eq!(session.rows_with_unknown, 1);
    assert_eq!(session.output_tokens.sum, 1_043);
    Ok(())
}

fn record_full(
    conn: &rusqlite::Connection,
    task_id: Option<i64>,
    turn: i64,
    now: &str,
) -> TestResult<()> {
    record(
        conn,
        &TokenUsageEvent {
            task_id,
            turn,
            input_tokens: Some(8_120),
            output_tokens: Some(1_040),
            cached_input_tokens: Some(6_880),
            total_tokens: Some(9_160),
            context_window: Some(24_576),
            context_used_estimate: Some(12_432),
            source: "endpoint".to_string(),
        },
        now,
    )?;
    Ok(())
}

fn record_partial(
    conn: &rusqlite::Connection,
    task_id: Option<i64>,
    turn: i64,
    now: &str,
) -> TestResult<()> {
    record(
        conn,
        &TokenUsageEvent {
            task_id,
            turn,
            input_tokens: None,
            output_tokens: Some(3),
            cached_input_tokens: None,
            total_tokens: None,
            context_window: None,
            context_used_estimate: None,
            source: "endpoint".to_string(),
        },
        now,
    )?;
    Ok(())
}
