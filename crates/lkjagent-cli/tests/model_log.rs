mod support;

use std::fs;

use lkjagent_cli::run_cli;
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::provider_exchange::{
    complete_exchange, record_request, ProviderExchangeCompletion, ProviderExchangeRequest,
};
use lkjagent_store::token_usage::{record, TokenUsageEvent};
use support::{open_store, temp_data, TestResult};

#[test]
fn model_log_command_writes_and_prints_single_current_markdown_file() -> TestResult<()> {
    let data = temp_data("model-log")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured documentation for lkjagent.",
        "2026-06-20T00:00:00Z",
    )?;
    append_event(
        &conn,
        Some(1),
        EventKind::Owner,
        "Create structured documentation for lkjagent.",
        6,
        "2026-06-20T00:00:00Z",
    )?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: Some(1),
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

    let path_output = run_cli(["--data", data.to_string_lossy().as_ref(), "model-log"]);
    let path = data.join("logs/current-model-run.md");
    let printed = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "--print",
    ]);

    assert_eq!(path_output.code, 0);
    assert!(path_output.stdout.contains("current-model-run.md"));
    assert!(path.exists());
    assert!(printed.stdout.contains("# lkjagent Model Run Log"));
    assert!(printed.stdout.contains("## Active State Tracks"));
    assert!(printed.stdout.contains("token_usage: in=8.12K"));
    assert!(printed.stdout.contains("Create structured documentation"));
    assert_eq!(fs::read_to_string(path)?, printed.stdout);
    Ok(())
}

#[test]
fn model_log_uses_large_manual_handoff_budget() -> TestResult<()> {
    let data = temp_data("model-log-budget")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Prepare a manual model handoff.",
        "2026-06-20T00:00:00Z",
    )?;
    for index in 0..30 {
        append_event(
            &conn,
            Some(1),
            EventKind::Observation,
            &format!("event-{index:03} {}", "context ".repeat(40)),
            index + 1,
            "2026-06-20T00:00:00Z",
        )?;
    }
    for index in 30..90 {
        append_event(
            &conn,
            Some(1),
            EventKind::Observation,
            &format!("event-{index:03} {}", "large-context ".repeat(200)),
            index + 1,
            "2026-06-20T00:00:00Z",
        )?;
    }

    let printed = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "--print",
    ]);

    assert_eq!(printed.code, 0);
    assert!(printed.stdout.chars().count() <= 1_000_000);
    assert!(printed.stdout.contains("event-000"));
    assert!(printed.stdout.contains("event-089"));
    Ok(())
}

#[test]
fn model_log_lists_and_shows_provider_exchanges() -> TestResult<()> {
    let data = temp_data("model-log-exchange")?;
    let conn = open_store(&data)?;
    record_request(
        &conn,
        &ProviderExchangeRequest {
            id: "exchange-cli",
            case_id: "9",
            turn_id: 4,
            prompt_frame_id: Some("prompt"),
            authority_decision_id: Some("12"),
            admission_decision_id: None,
            provider: "openai-compatible",
            model: "local-model",
            created_at: "2026-01-01T00:00:00Z",
            request_json: "{\"messages\":[]}",
            request_hash: "request-hash",
            redaction_schema_version: 1,
        },
    )?;
    complete_exchange(
        &conn,
        &ProviderExchangeCompletion {
            id: "exchange-cli",
            response_json: "{\"content\":\"ok\"}",
            response_hash: "response-hash",
            finish_reason: "stop",
            usage_json: Some("{\"total_tokens\":4}"),
            stats_json: None,
            latency_ms: 5,
        },
    )?;

    let list = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "list",
    ]);
    let show = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "show",
        "--case",
        "9",
        "--turn",
        "4",
    ]);

    assert_eq!(list.code, 0);
    assert!(list.stdout.contains("exchange-cli"));
    assert!(list.stdout.contains("status=succeeded"));
    assert_eq!(show.code, 0);
    assert!(show.stdout.contains("request_json:"));
    assert!(show.stdout.contains("response_hash=response-hash"));
    assert!(show.stdout.contains("{\"content\":\"ok\"}"));
    Ok(())
}
