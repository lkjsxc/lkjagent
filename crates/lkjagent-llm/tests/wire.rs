use lkjagent_context::model::{Message, Role};
use lkjagent_llm::wire::{build_request, decode_completion, FinishReason};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn request_serializes_exact_documented_fields() -> TestResult<()> {
    let messages = vec![
        Message::new(Role::System, "system prefix"),
        Message::new(Role::User, "<owner>hello</owner>"),
    ];
    let request = build_request("local-model", &messages, 2_048);
    let body = serde_json::to_string(&request)?;
    assert_eq!(
        body,
        "{\"model\":\"local-model\",\"messages\":[{\"role\":\"system\",\"content\":\"system prefix\"},{\"role\":\"user\",\"content\":\"<owner>hello</owner>\"}],\"max_tokens\":2048,\"temperature\":0.3,\"top_p\":0.9,\"stop\":[\"</act>\"],\"stream\":false}"
    );
    Ok(())
}

#[test]
fn response_reads_usage_finish_reason_and_cache_metrics() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<act></act>"},"finish_reason":"stop"}],
        "usage":{
          "prompt_tokens":11,
          "completion_tokens":7,
          "total_tokens":18,
          "prompt_tokens_details":{"cached_tokens":5}
        },
        "prompt_cache_hit_tokens":9,
        "timings":{"prompt_ms":4.5}
    }"#;
    let completion = decode_completion(response)?;
    assert_eq!(completion.content, "<act></act>");
    assert_eq!(completion.finish_reason, FinishReason::Stop);
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
    Ok(())
}

#[test]
fn response_preserves_missing_usage_as_unknown() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<act></act>"},"finish_reason":"stop"}]
    }"#;

    let completion = decode_completion(response)?;

    assert_eq!(completion.usage.prompt_tokens, None);
    assert_eq!(completion.usage.completion_tokens, None);
    assert_eq!(completion.usage.cached_prompt_tokens, None);
    assert_eq!(completion.usage.total_tokens, None);
    Ok(())
}

#[test]
fn stop_stripped_act_close_is_restored() -> TestResult<()> {
    let response = r#"{
        "choices":[{"message":{"content":"<act>\n<tool>agent.done</tool>\n<summary>x</summary>\n"},"finish_reason":"stop"}],
        "usage":{"prompt_tokens":11,"completion_tokens":7}
    }"#;

    let completion = decode_completion(response)?;

    assert!(completion.content.ends_with("</act>"));
    assert_eq!(completion.finish_reason, FinishReason::Stop);
    Ok(())
}
