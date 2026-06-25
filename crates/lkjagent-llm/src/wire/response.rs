use serde::Deserialize;
use serde_json::{Map, Value};

use super::{CacheMetric, CompletionUsage, ProviderAnomaly, ProviderAnomalyKind, WireError};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ResponseParts {
    pub content: String,
    pub finish_reason: super::FinishReason,
    pub usage: CompletionUsage,
    pub anomaly: Option<ProviderAnomaly>,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    choices: Vec<ResponseChoice>,
    usage: Option<ResponseUsage>,
}

#[derive(Debug, Deserialize)]
struct ResponseChoice {
    message: Value,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ResponseUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct PromptTokensDetails {
    cached_tokens: Option<u64>,
}

pub(super) fn response_parts(
    value: Value,
    cache_metrics: &[CacheMetric],
) -> Result<ResponseParts, WireError> {
    let body: ResponseBody =
        serde_json::from_value(value).map_err(|error| WireError::Json(error.to_string()))?;
    let usage = usage_from_response(body.usage, cache_metrics);
    let choice = body
        .choices
        .into_iter()
        .next()
        .ok_or(WireError::Missing("choices[0]"))?;
    let finish_reason = super::finish_reason(choice.finish_reason);
    let (content, anomaly) = content_and_anomaly(&choice.message, &usage);
    Ok(ResponseParts {
        content,
        finish_reason,
        usage,
        anomaly,
    })
}

fn content_and_anomaly(
    message: &Value,
    usage: &CompletionUsage,
) -> (String, Option<ProviderAnomaly>) {
    let Some(object) = message.as_object() else {
        return (
            String::new(),
            Some(ProviderAnomaly::new(
                ProviderAnomalyKind::MalformedProviderMessage,
                "choices[0].message is not an object",
            )),
        );
    };
    match object.get("content") {
        Some(Value::String(content)) => classify_string_content(content, object, usage),
        Some(Value::Null) | None => missing_content(object, usage),
        Some(value) => (
            String::new(),
            Some(ProviderAnomaly::new(
                ProviderAnomalyKind::MalformedProviderMessage,
                format!("choices[0].message.content has type {}", value_type(value)),
            )),
        ),
    }
}

fn classify_string_content(
    content: &str,
    object: &Map<String, Value>,
    usage: &CompletionUsage,
) -> (String, Option<ProviderAnomaly>) {
    if !content.trim().is_empty() {
        return (content.to_string(), None);
    }
    if has_reasoning(object) {
        return anomaly(
            ProviderAnomalyKind::ReasoningOnlyResponse,
            "reasoning-only response",
        );
    }
    if has_tool_calls(object) {
        return anomaly(
            ProviderAnomalyKind::ToolCallOnlyResponse,
            "tool-call-only response",
        );
    }
    if usage.completion_tokens.unwrap_or(0) > 0 {
        return anomaly(
            ProviderAnomalyKind::EmptyContentWithUsage,
            "empty content with nonzero completion tokens",
        );
    }
    anomaly(
        ProviderAnomalyKind::EmptyContentNoUsage,
        "empty content without output token evidence",
    )
}

fn missing_content(
    object: &Map<String, Value>,
    _usage: &CompletionUsage,
) -> (String, Option<ProviderAnomaly>) {
    if has_reasoning(object) {
        return anomaly(
            ProviderAnomalyKind::ReasoningOnlyResponse,
            "reasoning-only response",
        );
    }
    if has_tool_calls(object) {
        return anomaly(
            ProviderAnomalyKind::ToolCallOnlyResponse,
            "tool-call-only response",
        );
    }
    anomaly(
        ProviderAnomalyKind::MissingContentField,
        "choices[0].message.content is missing",
    )
}

fn anomaly(kind: ProviderAnomalyKind, detail: &str) -> (String, Option<ProviderAnomaly>) {
    (String::new(), Some(ProviderAnomaly::new(kind, detail)))
}

fn has_reasoning(object: &Map<String, Value>) -> bool {
    ["reasoning", "reasoning_content", "thoughts", "thinking"]
        .iter()
        .any(|key| object.get(*key).is_some_and(non_empty_value))
}

fn has_tool_calls(object: &Map<String, Value>) -> bool {
    object.get("tool_calls").is_some_and(non_empty_value)
        || object.get("function_call").is_some_and(non_empty_value)
}

fn non_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(value) => !value.trim().is_empty(),
        Value::Array(values) => !values.is_empty(),
        Value::Object(values) => !values.is_empty(),
        Value::Bool(_) | Value::Number(_) => true,
    }
}

fn usage_from_response(
    usage: Option<ResponseUsage>,
    cache_metrics: &[CacheMetric],
) -> CompletionUsage {
    let cached = usage
        .as_ref()
        .and_then(|usage| usage.prompt_tokens_details.as_ref())
        .and_then(|details| details.cached_tokens)
        .or_else(|| cache_metric_u64(cache_metrics, "prompt_cache_hit_tokens"));
    CompletionUsage {
        prompt_tokens: usage.as_ref().and_then(|usage| usage.prompt_tokens),
        completion_tokens: usage.as_ref().and_then(|usage| usage.completion_tokens),
        cached_prompt_tokens: cached,
        total_tokens: usage.and_then(|usage| usage.total_tokens),
    }
}

fn cache_metric_u64(metrics: &[CacheMetric], name: &str) -> Option<u64> {
    metrics
        .iter()
        .find(|metric| metric.name == name)
        .and_then(|metric| metric.value.parse::<u64>().ok())
}

fn value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
