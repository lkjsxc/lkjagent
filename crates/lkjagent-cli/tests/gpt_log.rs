mod support;

use std::fs;

use lkjagent_cli::run_cli;
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::token_usage::{record, TokenUsageEvent};
use support::{open_store, temp_data, TestResult};

#[test]
fn gpt_log_command_writes_and_prints_single_current_markdown_file() -> TestResult<()> {
    let data = temp_data("gpt-log")?;
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

    let path_output = run_cli(["--data", data.to_string_lossy().as_ref(), "gpt-log"]);
    let path = data.join("logs/current-gpt-5.5-pro.md");
    let printed = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "gpt-log",
        "--print",
    ]);

    assert_eq!(path_output.code, 0);
    assert!(path_output.stdout.contains("current-gpt-5.5-pro.md"));
    assert!(path.exists());
    assert!(printed.stdout.contains("# lkjagent GPT-5.5-Pro Run Log"));
    assert!(printed.stdout.contains("## Active State Tracks"));
    assert!(printed.stdout.contains("token_usage: in=8.12K"));
    assert!(printed.stdout.contains("Create structured documentation"));
    assert_eq!(fs::read_to_string(path)?, printed.stdout);
    Ok(())
}

#[test]
fn gpt_log_uses_large_manual_handoff_budget() -> TestResult<()> {
    let data = temp_data("gpt-log-budget")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Prepare a manual ChatGPT handoff.",
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
        "gpt-log",
        "--print",
    ]);

    assert_eq!(printed.code, 0);
    assert!(printed.stdout.chars().count() <= 100_000);
    assert!(printed.stdout.contains("event-078"));
    assert!(printed.stdout.contains("event-089"));
    assert!(!printed.stdout.contains("event-000"));
    Ok(())
}
