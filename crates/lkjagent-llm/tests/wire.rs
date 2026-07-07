use lkjagent_llm::closure::ClosureMode;
use lkjagent_llm::message::{Message, Role};
use lkjagent_llm::wire::{
    build_request, decode_completion, CallSpec, FinishReason, ProviderAnomalyKind, MAX_TOKENS,
};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn request_serializes_exact_documented_fields() -> TestResult<()> {
    let messages = vec![
        Message::new(Role::System, "system prefix"),
        Message::new(Role::User, "<owner>hello</owner>"),
    ];
    let spec = CallSpec::with_stop(1_400, "</content>");
    let request = build_request("local-model", &messages, &spec);
    let body = serde_json::to_string(&request)?;
    assert_eq!(
        body,
        "{\"model\":\"local-model\",\"messages\":[{\"role\":\"system\",\"content\":\"system prefix\"},{\"role\":\"user\",\"content\":\"<owner>hello</owner>\"}],\"max_tokens\":1400,\"temperature\":0.3,\"top_p\":0.9,\"reasoning_effort\":\"none\",\"stop\":[\"</content>\"],\"stream\":false}"
    );
    Ok(())
}

#[test]
fn compact_default_max_tokens_is_512() -> TestResult<()> {
    let messages = vec![Message::new(Role::System, "system prefix")];
    let spec = CallSpec::action(MAX_TOKENS);
    let request = build_request("local-model", &messages, &spec);
    let body = serde_json::to_string(&request)?;

    assert!(body.contains("\"max_tokens\":512"));
    Ok(())
}

#[test]
fn response_reads_usage_finish_reason_and_cache_metrics() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<lkjagent_action></lkjagent_action>"},"finish_reason":"stop"}],
        "usage":{
          "prompt_tokens":11,
          "completion_tokens":7,
          "total_tokens":18,
          "prompt_tokens_details":{"cached_tokens":5}
        },
        "prompt_cache_hit_tokens":9,
        "timings":{"prompt_ms":4.5}
    }"#;
    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;
    assert_eq!(completion.content, "<lkjagent_action></lkjagent_action>");
    assert_eq!(completion.finish_reason, FinishReason::Stop);
    assert_eq!(completion.closure_mode, ClosureMode::Natural);
    assert_eq!(completion.usage.prompt_tokens, Some(11));
    assert_eq!(completion.usage.completion_tokens, Some(7));
    assert_eq!(completion.usage.cached_prompt_tokens, Some(5));
    assert_eq!(completion.usage.total_tokens, Some(18));
    assert!(completion
        .cache_metrics
        .iter()
        .any(|metric| metric.name == "prompt_cache_hit_tokens" && metric.value == "9"));
    assert!(completion
        .cache_metrics
        .iter()
        .any(|metric| metric.name == "timings.prompt_ms" && metric.value == "4.5"));
    assert_eq!(completion.provider_anomaly, None);
    Ok(())
}

#[test]
fn response_preserves_missing_usage_as_unknown() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<lkjagent_action></lkjagent_action>"},"finish_reason":"stop"}]
    }"#;

    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;

    assert_eq!(completion.usage.prompt_tokens, None);
    assert_eq!(completion.usage.completion_tokens, None);
    assert_eq!(completion.usage.cached_prompt_tokens, None);
    assert_eq!(completion.usage.total_tokens, None);
    Ok(())
}

#[test]
fn empty_content_with_completion_tokens_is_provider_anomaly() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":""},"finish_reason":"stop"}],
        "usage":{"prompt_tokens":10512,"completion_tokens":485,"total_tokens":10997}
    }"#;

    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;

    let anomaly = completion.provider_anomaly.ok_or("provider anomaly")?;
    assert_eq!(completion.content, "");
    assert_eq!(anomaly.kind, ProviderAnomalyKind::EmptyContentWithUsage);
    assert_eq!(completion.usage.completion_tokens, Some(485));
    Ok(())
}

#[test]
fn reasoning_only_response_is_not_tool_call_text() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"","reasoning":"hidden chain"},"finish_reason":"stop"}],
        "usage":{"completion_tokens":12}
    }"#;

    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;

    let anomaly = completion.provider_anomaly.ok_or("provider anomaly")?;
    assert_eq!(completion.content, "");
    assert_eq!(anomaly.kind, ProviderAnomalyKind::ReasoningOnlyResponse);
    Ok(())
}

#[test]
fn missing_content_field_is_provider_anomaly() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"role":"assistant"},"finish_reason":"stop"}],
        "usage":{"completion_tokens":0}
    }"#;

    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;

    let anomaly = completion.provider_anomaly.ok_or("provider anomaly")?;
    assert_eq!(completion.content, "");
    assert_eq!(anomaly.kind, ProviderAnomalyKind::MissingContentField);
    Ok(())
}

#[test]
fn stop_stripped_tool_call_close_is_restored() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<lkjagent_action>\n{}\n"},"finish_reason":"stop"}],
        "usage":{"prompt_tokens":11,"completion_tokens":7}
    }"#;

    let completion = decode_completion(response, &CallSpec::action(MAX_TOKENS))?;

    assert!(completion.content.ends_with("</lkjagent_action>"));
    assert_eq!(completion.finish_reason, FinishReason::Stop);
    assert_eq!(completion.closure_mode, ClosureMode::StopSequenceClosed);
    Ok(())
}

#[test]
fn stop_stripped_content_plan_and_message_closures_are_restored() -> TestResult<()> {
    for (open, close) in [
        ("<content>body", "</content>"),
        ("<plan>respond | done", "</plan>"),
        ("<message>done", "</message>"),
    ] {
        let response = format!(
            "{{\"choices\":[{{\"message\":{{\"content\":{open:?}}},\"finish_reason\":\"stop\"}}]}}"
        );
        let spec = CallSpec::with_stop(MAX_TOKENS, close);
        let completion = decode_completion(&response, &spec)?;
        assert!(completion.content.ends_with(close));
        assert_eq!(completion.closure_mode, ClosureMode::StopSequenceClosed);
    }
    Ok(())
}
