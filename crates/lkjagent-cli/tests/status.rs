mod support;

use lkjagent_cli::run_cli;
use lkjagent_store::token_usage::{record, TokenUsageEvent};
use support::{open_store, temp_data, TestResult};

#[test]
fn status_prints_ranked_active_state_tracks() -> TestResult<()> {
    let data = temp_data("status-tracks")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured documentation for lkjagent.",
        "2026-06-20T00:00:00Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("active_states=1."));
    assert!(status.stdout.contains("document-structure"));
    assert!(status.stdout.contains("phase=planning"));
    assert!(status.stdout.contains("gpt_log="));
    Ok(())
}

#[test]
fn status_prints_compact_context_and_token_usage() -> TestResult<()> {
    let data = temp_data("status-accounting")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "context used tokens", "1234")?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: None,
            turn: 1,
            input_tokens: Some(8_120),
            output_tokens: Some(1_040),
            cached_input_tokens: Some(6_880),
            total_tokens: Some(9_160),
            context_window: Some(24_576),
            context_used_estimate: Some(1_234),
            source: "endpoint".to_string(),
        },
        "2026-06-20T00:00:00Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("ctx=1.23K/24.58K 5.02%"));
    assert!(status
        .stdout
        .contains("in=8.12K out=1.04K cache=6.88K total=9.16K"));
    Ok(())
}

#[test]
fn status_prints_unknown_token_usage_as_unknown() -> TestResult<()> {
    let data = temp_data("status-unknown-usage")?;
    open_store(&data)?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status
        .stdout
        .contains("in=unknown out=unknown cache=unknown total=unknown"));
    Ok(())
}
