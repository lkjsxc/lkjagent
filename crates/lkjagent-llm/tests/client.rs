mod support;

use std::time::Duration;

use lkjagent_llm::client::{complete, ClientConfig, DEFAULT_TIMEOUT_SECONDS};
use lkjagent_llm::error::{ClientError, EndpointFailure};
use lkjagent_llm::message::{Message, Role};
use lkjagent_llm::wire::CallSpec;
use support::{serve_once, TestResult};

#[test]
fn action_call_spec_uses_compact_output_budget() {
    let spec = CallSpec::action(512);

    assert_eq!(spec.max_tokens, 512);
    assert_eq!(spec.stop, vec!["</action>".to_string()]);
}

#[test]
fn client_config_defaults_to_loose_finite_timeout() {
    let config = ClientConfig::new("http://localhost:1234", "local-model");

    assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));
    assert_eq!(DEFAULT_TIMEOUT_SECONDS, 900);
}

#[test]
fn local_stub_server_receives_request_and_returns_completion() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"<action></action>"},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":3},"prompt_cache_hit_tokens":4}"#;
    let server = serve_once(200, body)?;
    let mut config = ClientConfig::new(server.base_url.clone(), "local-model");
    config.api_key = Some("secret-token".to_string());
    config.max_tokens = 1_024;
    let messages = vec![Message::new(Role::System, "system")];

    let spec = CallSpec::action(config.max_tokens);
    let completion = complete(&config, &messages, &spec, 0)?;
    let request = server.recorded()?;

    assert_eq!(completion.content, "<action></action>");
    assert_eq!(completion.usage.prompt_tokens, Some(5));
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/v1/chat/completions");
    assert_eq!(
        request.authorization.as_deref(),
        Some("Bearer secret-token")
    );
    assert_eq!(
        request.body,
        "{\"model\":\"local-model\",\"messages\":[{\"role\":\"system\",\"content\":\"system\"}],\"max_tokens\":1024,\"temperature\":0.3,\"top_p\":0.9,\"reasoning_effort\":\"none\",\"stop\":[\"</action>\"],\"stream\":false}"
    );
    Ok(())
}

#[test]
fn length_finish_reason_maps_to_oversize() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"partial"},"finish_reason":"length"}],"usage":{"prompt_tokens":5,"completion_tokens":2048},"prompt_cache_hit_tokens":4}"#;
    let server = serve_once(200, body)?;
    let config = ClientConfig::new(server.base_url.clone(), "local-model");
    let spec = CallSpec::action(config.max_tokens);
    let result = complete(&config, &[Message::new(Role::System, "system")], &spec, 1);
    let _request = server.recorded()?;

    assert!(matches!(
        result,
        Err(ClientError::Oversize {
            usage,
            cache_metrics,
            preview
        }) if usage.completion_tokens == Some(2048) && cache_metrics.len() == 1
            && preview == "partial"
    ));
    Ok(())
}

#[test]
fn length_with_closed_action_is_accepted() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"<action>\n<tool>agent.done</tool>\n<summary>x</summary>\n</action>\nextra"},"finish_reason":"length"}],"usage":{"prompt_tokens":5,"completion_tokens":2048}}"#;
    let server = serve_once(200, body)?;
    let config = ClientConfig::new(server.base_url.clone(), "local-model");

    let spec = CallSpec::action(config.max_tokens);
    let completion = complete(&config, &[Message::new(Role::System, "system")], &spec, 1)?;
    let _request = server.recorded()?;

    assert!(completion.content.contains("</action>"));
    assert_eq!(
        completion.finish_reason,
        lkjagent_llm::wire::FinishReason::Length
    );
    Ok(())
}

#[test]
fn connection_failure_maps_to_attempt_backoff() -> TestResult<()> {
    let config = ClientConfig::new("http://127.0.0.1:1", "local-model");
    let spec = CallSpec::action(config.max_tokens);
    let result = complete(&config, &[Message::new(Role::System, "system")], &spec, 3);

    assert!(matches!(
        result,
        Err(ClientError::Endpoint {
            failure: EndpointFailure::Connection(_),
            retry_after
        }) if retry_after == Duration::from_secs(8)
    ));
    Ok(())
}

#[test]
fn four_hundred_status_maps_to_endpoint_overflow() -> TestResult<()> {
    let server = serve_once(400, "{\"error\":\"context overflow\"}")?;
    let config = ClientConfig::new(server.base_url.clone(), "local-model");
    let spec = CallSpec::action(config.max_tokens);
    let result = complete(&config, &[Message::new(Role::System, "system")], &spec, 0);
    let _request = server.recorded()?;

    assert!(matches!(
        result,
        Err(ClientError::EndpointOverflow { status: 400, .. })
    ));
    Ok(())
}
