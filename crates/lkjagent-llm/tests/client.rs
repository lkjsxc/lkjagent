mod support;

use std::net::TcpListener;
use std::time::Duration;

use lkjagent_context::model::{Message, Role};
use lkjagent_llm::client::{complete, ClientConfig};
use lkjagent_llm::error::{ClientError, EndpointFailure};
use support::{serve_once, TestResult};

#[test]
fn local_stub_server_receives_request_and_returns_completion() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"<act></act>"},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":3},"prompt_cache_hit_tokens":4}"#;
    let server = serve_once(200, body)?;
    let mut config = ClientConfig::new(server.base_url.clone(), "local-model");
    config.api_key = Some("secret-token".to_string());
    let messages = vec![Message::new(Role::System, "system")];

    let completion = complete(&config, &messages, 0)?;
    let request = server.recorded()?;

    assert_eq!(completion.content, "<act></act>");
    assert_eq!(completion.usage.prompt_tokens, 5);
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/v1/chat/completions");
    assert_eq!(
        request.authorization.as_deref(),
        Some("Bearer secret-token")
    );
    assert_eq!(
        request.body,
        "{\"model\":\"local-model\",\"messages\":[{\"role\":\"system\",\"content\":\"system\"}],\"max_tokens\":1024,\"temperature\":0.3,\"top_p\":0.9,\"stop\":[\"</act>\"],\"stream\":false}"
    );
    Ok(())
}

#[test]
fn length_finish_reason_maps_to_oversize() -> TestResult<()> {
    let body = r#"{"choices":[{"message":{"content":"partial"},"finish_reason":"length"}],"usage":{"prompt_tokens":5,"completion_tokens":1024},"prompt_cache_hit_tokens":4}"#;
    let server = serve_once(200, body)?;
    let config = ClientConfig::new(server.base_url.clone(), "local-model");
    let result = complete(&config, &[Message::new(Role::System, "system")], 1);
    let _request = server.recorded()?;

    assert!(matches!(
        result,
        Err(ClientError::Oversize {
            usage,
            cache_metrics
        }) if usage.completion_tokens == 1024 && cache_metrics.len() == 1
    ));
    Ok(())
}

#[test]
fn connection_failure_maps_to_attempt_backoff() -> TestResult<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    drop(listener);
    let config = ClientConfig::new(format!("http://{address}"), "local-model");
    let result = complete(&config, &[Message::new(Role::System, "system")], 3);

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
    let result = complete(&config, &[Message::new(Role::System, "system")], 0);
    let _request = server.recorded()?;

    assert!(matches!(
        result,
        Err(ClientError::EndpointOverflow { status: 400, .. })
    ));
    Ok(())
}
