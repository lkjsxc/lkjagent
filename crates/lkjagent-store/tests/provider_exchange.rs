mod support;

use lkjagent_store::provider_exchange::{
    complete_exchange, fail_exchange, latest_for_case_turn, record_request,
    ProviderExchangeCompletion, ProviderExchangeFailure, ProviderExchangeRequest,
};
use support::{memory_store, TestResult};

#[test]
fn provider_exchange_records_successful_model_turn() -> TestResult<()> {
    let conn = memory_store()?;
    record_request(
        &conn,
        &ProviderExchangeRequest {
            id: "exchange-1",
            case_id: "7",
            turn_id: 3,
            prompt_frame_id: Some("prompt-3"),
            authority_decision_id: Some("42"),
            admission_decision_id: None,
            provider: "openai-compatible",
            model: "local-model",
            created_at: "2026-01-01T00:00:00Z",
            request_json: "{\"messages\":[]}",
            request_hash: "hash-request",
            redaction_schema_version: 1,
        },
    )?;
    complete_exchange(
        &conn,
        &ProviderExchangeCompletion {
            id: "exchange-1",
            response_json: "{\"content\":\"ok\"}",
            response_hash: "hash-response",
            finish_reason: "stop",
            usage_json: Some("{\"total_tokens\":4}"),
            stats_json: None,
            latency_ms: 17,
            status: "succeeded",
        },
    )?;

    let row = latest_for_case_turn(&conn, "7", 3)?.ok_or("missing exchange row")?;
    assert_eq!(row.id, "exchange-1");
    assert_eq!(row.status, "succeeded");
    assert_eq!(row.finish_reason.as_deref(), Some("stop"));
    assert_eq!(row.request_hash, "hash-request");
    assert_eq!(row.response_hash.as_deref(), Some("hash-response"));
    Ok(())
}

#[test]
fn provider_exchange_records_failed_model_turn() -> TestResult<()> {
    let conn = memory_store()?;
    record_request(
        &conn,
        &ProviderExchangeRequest {
            id: "exchange-2",
            case_id: "none",
            turn_id: 4,
            prompt_frame_id: None,
            authority_decision_id: None,
            admission_decision_id: None,
            provider: "openai-compatible",
            model: "local-model",
            created_at: "2026-01-01T00:00:01Z",
            request_json: "{\"messages\":[]}",
            request_hash: "hash-request-2",
            redaction_schema_version: 1,
        },
    )?;
    fail_exchange(
        &conn,
        &ProviderExchangeFailure {
            id: "exchange-2",
            error_class: "EndpointError",
            response_json: Some("{\"error\":\"offline\"}"),
            response_hash: Some("hash-error"),
            latency_ms: 8,
        },
    )?;

    let row = latest_for_case_turn(&conn, "none", 4)?.ok_or("missing exchange row")?;
    assert_eq!(row.status, "failed");
    assert_eq!(row.error_class.as_deref(), Some("EndpointError"));
    assert_eq!(row.finish_reason, None);
    Ok(())
}
