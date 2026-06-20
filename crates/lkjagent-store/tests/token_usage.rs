mod support;

use lkjagent_store::token_usage::{latest, record, TokenUsageEvent};
use support::{memory_store, TestResult};

#[test]
fn token_usage_ledger_preserves_known_and_unknown_values() -> TestResult<()> {
    let conn = memory_store()?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: Some(42),
            turn: 7,
            input_tokens: Some(8_120),
            output_tokens: Some(1_040),
            cached_input_tokens: Some(6_880),
            total_tokens: Some(9_160),
            context_window: Some(24_576),
            context_used_estimate: Some(12_432),
            source: "endpoint".to_string(),
        },
        "2026-06-20T00:00:00Z",
    )?;
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

    let row = latest(&conn)?.expect("latest token usage row");

    assert_eq!(row.turn, 8);
    assert_eq!(row.input_tokens, None);
    assert_eq!(row.output_tokens, Some(3));
    assert_eq!(row.total_tokens, None);
    Ok(())
}
