mod support;

use std::fs;

use lkjagent_cli::run_cli;
use lkjagent_store::provider_exchange::{
    complete_exchange, record_request, ProviderExchangeCompletion, ProviderExchangeRequest,
};
use support::{open_store, temp_data, TestResult};

#[test]
fn model_log_export_writes_replay_json() -> TestResult<()> {
    let data = temp_data("model-log-export")?;
    seed_exchange(&data, "7", 3, "exchange-export")?;
    let turn_dir = data.join("logs/model/epoch-2026/case-7/turn-000003");
    fs::create_dir_all(&turn_dir)?;
    fs::write(turn_dir.join("request.json"), "{\"messages\":[]}")?;
    fs::write(turn_dir.join("parsed-action.json"), "{\"status\":\"ok\"}")?;

    let output = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "export",
        "--case",
        "7",
        "--turn",
        "3",
    ]);

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("provider_exchange_export="));
    let path = data.join("logs/model/archive/case-7/turn-000003.json");
    let replay = fs::read_to_string(path)?;
    assert!(replay.contains("exchange-export"));
    assert!(replay.contains("request_json"));
    assert!(replay.contains("response_json"));
    assert!(data
        .join("logs/model/archive/case-7/files/request.json")
        .exists());
    assert!(data
        .join("logs/model/archive/case-7/files/parsed-action.json")
        .exists());
    Ok(())
}

#[test]
fn model_log_raw_case_lists_case_turns() -> TestResult<()> {
    let data = temp_data("model-log-raw-case")?;
    seed_exchange(&data, "9", 4, "exchange-raw")?;

    let output = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "model-log",
        "raw-case",
        "--case",
        "9",
    ]);

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("case=9 turn=4"));
    assert!(output.stdout.contains("status=succeeded"));
    Ok(())
}

fn seed_exchange(data: &std::path::Path, case_id: &str, turn_id: i64, id: &str) -> TestResult<()> {
    let conn = open_store(data)?;
    record_request(
        &conn,
        &ProviderExchangeRequest {
            id,
            case_id,
            turn_id,
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
            id,
            response_json: "{\"content\":\"ok\"}",
            response_hash: "response-hash",
            finish_reason: "stop",
            usage_json: Some("{\"total_tokens\":4}"),
            stats_json: None,
            latency_ms: 5,
            status: "succeeded",
        },
    )?;
    Ok(())
}
