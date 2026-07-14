mod support;
use std::time::Duration;

use lkjagent_llm::client::{complete, ClientConfig, MAX_RESPONSE_BYTES};
use lkjagent_llm::error::{ClientError, EndpointFailure, FaultClass};
use lkjagent_llm::message::Message;
use lkjagent_llm::wire::{
    build_request, decode_completion, CallSpec, FinishReason, ProviderAnomalyKind, WireError,
};
use support::{serve_once, serve_owned, TestResult};
const OK: &str = r#"{"choices":[{"message":{"content":"done"},"finish_reason":"stop"}]}"#;

#[test]
fn nondefault_fields_reasoning_and_auth_are_confined() -> TestResult<()> {
    let server = serve_once(200, OK)?;
    let mut config = ClientConfig::new(&server.base_url, "chosen-model");
    let marker = "fixture-credential";
    config.api_key = Some(marker.into());
    config.timeout = Duration::from_secs(2);
    let spec = CallSpec::with_stop(777, "END")
        .with_sampling(0.17, 0.61)
        .with_reasoning_effort("high");
    complete(&config, &[Message::user("hello")], &spec, 0)?;
    let request = server.recorded()?;
    let value: serde_json::Value = serde_json::from_str(&request.body)?;
    assert_eq!(value["model"], "chosen-model");
    assert_eq!(value["max_tokens"], 777);
    assert_eq!(value["temperature"], 0.17);
    assert_eq!(value["top_p"], 0.61);
    assert_eq!(value["reasoning_effort"], "high");
    assert_eq!(
        request.authorization.as_deref(),
        Some("Bearer fixture-credential")
    );
    assert!(!request.body.contains(marker));
    assert!(!format!("{config:?}").contains(marker));
    Ok(())
}

#[test]
fn absent_reasoning_is_omitted_and_explicit_none_is_retained() -> TestResult<()> {
    let absent = build_request("m", &[Message::user("x")], &CallSpec::action(33));
    let absent = serde_json::to_value(absent)?;
    assert!(absent.get("reasoning_effort").is_none());
    let none = build_request(
        "m",
        &[Message::user("x")],
        &CallSpec::action(33).with_reasoning_effort("none"),
    );
    assert_eq!(serde_json::to_value(none)?["reasoning_effort"], "none");
    Ok(())
}

#[test]
fn usage_cache_and_finish_variants_are_lossless() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"x"},"finish_reason":"content_filter"}],"usage":{"prompt_tokens":8,"completion_tokens":3,"total_tokens":11,"cache_read_input_tokens":4},"prompt_cache_hit_tokens":2,"timings":{"prompt_ms":1.5}}"#;
    let result = decode_completion(body, &CallSpec::action(8))?;
    assert_eq!(
        result.finish_reason,
        FinishReason::Other("content_filter".into())
    );
    assert_eq!(result.usage.prompt_tokens, Some(8));
    assert_eq!(result.usage.completion_tokens, Some(3));
    assert_eq!(result.usage.total_tokens, Some(11));
    assert_eq!(result.usage.cached_prompt_tokens, Some(4));
    assert_eq!(result.cache_metrics.len(), 2);
    let missing = decode_completion(
        r#"{"choices":[{"message":{"content":"x"}}]}"#,
        &CallSpec::action(8),
    )?;
    assert_eq!(missing.finish_reason, FinishReason::Missing);
    Ok(())
}

#[test]
fn response_anomalies_remain_distinct() -> TestResult<()> {
    let cases = [
        (
            r#"{"content":"","reasoning":"x"}"#,
            ProviderAnomalyKind::ReasoningOnlyResponse,
        ),
        (
            r#"{"content":"","tool_calls":[{}]}"#,
            ProviderAnomalyKind::ToolCallOnlyResponse,
        ),
        (
            r#"{"content":""}"#,
            ProviderAnomalyKind::EmptyContentWithUsage,
        ),
        (
            r#"{"role":"assistant"}"#,
            ProviderAnomalyKind::MissingContentField,
        ),
        (
            r#"{"content":7}"#,
            ProviderAnomalyKind::MalformedProviderMessage,
        ),
    ];
    for (message, expected) in cases {
        let body = format!(
            r#"{{"choices":[{{"message":{message},"finish_reason":"stop"}}],"usage":{{"completion_tokens":2}}}}"#
        );
        let result = decode_completion(&body, &CallSpec::action(8))?;
        assert_eq!(
            result.provider_anomaly.map(|value| value.kind),
            Some(expected)
        );
    }
    Ok(())
}

#[test]
fn malformed_json_and_shape_are_distinct() {
    assert_eq!(
        decode_completion("{", &CallSpec::action(8)),
        Err(WireError::Json)
    );
    assert_eq!(
        decode_completion(r#"{"choices":[]}"#, &CallSpec::action(8)),
        Err(WireError::Shape("choices[0]"))
    );
}

#[test]
fn response_and_status_errors_are_bounded() -> TestResult<()> {
    let server = serve_owned(200, "x".repeat(MAX_RESPONSE_BYTES + 1), Duration::ZERO)?;
    let config = ClientConfig::new(&server.base_url, "m");
    let Err(error) = complete(&config, &[Message::user("x")], &CallSpec::action(8), 0) else {
        return Err("oversized body was accepted".into());
    };
    assert!(matches!(
        error,
        ClientError::Endpoint {
            failure: EndpointFailure::ResponseTooLarge {
                limit: MAX_RESPONSE_BYTES
            },
            ..
        }
    ));
    let raw = "private provider body";
    let server = serve_once(503, raw)?;
    let config = ClientConfig::new(&server.base_url, "m");
    let Err(error) = complete(&config, &[Message::user("x")], &CallSpec::action(8), 0) else {
        return Err("status was accepted".into());
    };
    let _ = server.recorded()?;
    assert_eq!(error.fault_class(), FaultClass::HttpStatus);
    assert!(!format!("{error:?} {error}").contains(raw));
    Ok(())
}
#[test]
fn timeout_connect_and_status_are_distinct() -> TestResult<()> {
    let server = serve_owned(200, OK.into(), Duration::from_millis(150))?;
    let mut config = ClientConfig::new(&server.base_url, "m");
    config.timeout = Duration::from_millis(20);
    let timeout = complete(&config, &[Message::user("x")], &CallSpec::action(8), 0);
    assert!(matches!(
        timeout,
        Err(ClientError::Endpoint {
            failure: EndpointFailure::Timeout,
            ..
        })
    ));

    let config = ClientConfig::new("http://127.0.0.1:1", "m");
    let connect = complete(&config, &[Message::user("x")], &CallSpec::action(8), 0);
    assert!(matches!(
        connect,
        Err(ClientError::Endpoint {
            failure: EndpointFailure::Connect,
            ..
        })
    ));

    let server = serve_once(502, "ignored")?;
    let config = ClientConfig::new(&server.base_url, "m");
    let status = complete(&config, &[Message::user("x")], &CallSpec::action(8), 0);
    let _ = server.recorded()?;
    assert!(matches!(
        status,
        Err(ClientError::Endpoint {
            failure: EndpointFailure::Status { status: 502 },
            ..
        })
    ));
    Ok(())
}

#[test]
fn length_is_not_repaired_and_ambiguous_send_is_not_retried() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"<tool_call>{}"},"finish_reason":"length"}]}"#;
    let server = serve_once(200, body)?;
    let config = ClientConfig::new(&server.base_url, "m");
    let result = complete(&config, &[Message::user("x")], &CallSpec::action(8), 9)?;
    let _single_request = server.recorded()?;
    assert_eq!(result.content, "<tool_call>{}");
    assert_eq!(result.finish_reason, FinishReason::Length);
    assert!(result.transport.is_some());
    Ok(())
}
