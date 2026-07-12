use super::{CacheMetric, CompletionUsage, ProviderAnomaly, ProviderAnomalyKind, WireError};
use serde::Deserialize;
use serde_json::{Map, Value};

pub(super) struct ResponseParts {
    pub content: String,
    pub finish_reason: super::FinishReason,
    pub usage: CompletionUsage,
    pub cache_metrics: Vec<CacheMetric>,
    pub anomaly: Option<ProviderAnomaly>,
}
#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<ResponseChoice>,
    usage: Option<ResponseUsage>,
    prompt_cache_hit_tokens: Option<u64>,
    timings: Option<Map<String, Value>>,
}

#[derive(Deserialize)]
struct ResponseChoice {
    message: Value,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ResponseUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    prompt_tokens_details: Option<PromptTokensDetails>,
    cache_read_input_tokens: Option<u64>,
}

#[derive(Deserialize)]
struct PromptTokensDetails {
    cached_tokens: Option<u64>,
}

pub(super) fn response_parts(value: Value) -> Result<ResponseParts, WireError> {
    let body: ResponseBody =
        serde_json::from_value(value).map_err(|_| WireError::Shape("response"))?;
    let cache_metrics = cache_metrics(&body);
    let usage = usage_from_response(body.usage, &cache_metrics);
    let choice = body
        .choices
        .into_iter()
        .next()
        .ok_or(WireError::Shape("choices[0]"))?;
    let finish_reason = super::finish_reason(choice.finish_reason);
    let (content, anomaly) = content_and_anomaly(&choice.message, &usage);
    Ok(ResponseParts {
        content,
        finish_reason,
        usage,
        cache_metrics,
        anomaly,
    })
}

fn content_and_anomaly(
    message: &Value,
    usage: &CompletionUsage,
) -> (String, Option<ProviderAnomaly>) {
    let Some(object) = message.as_object() else {
        return anomaly(
            ProviderAnomalyKind::MalformedProviderMessage,
            "choices[0].message is not an object",
        );
    };
    match object.get("content") {
        Some(Value::String(content)) => classify_content(content, object, usage),
        Some(Value::Null) | None => missing_content(object),
        Some(_) => anomaly(
            ProviderAnomalyKind::MalformedProviderMessage,
            "choices[0].message.content is not text",
        ),
    }
}

fn classify_content(
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
            "native tool-call-only response",
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

fn missing_content(object: &Map<String, Value>) -> (String, Option<ProviderAnomaly>) {
    if has_reasoning(object) {
        return anomaly(
            ProviderAnomalyKind::ReasoningOnlyResponse,
            "reasoning-only response",
        );
    }
    if has_tool_calls(object) {
        return anomaly(
            ProviderAnomalyKind::ToolCallOnlyResponse,
            "native tool-call-only response",
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
        .any(|key| object.get(*key).is_some_and(non_empty))
}

fn has_tool_calls(object: &Map<String, Value>) -> bool {
    object.get("tool_calls").is_some_and(non_empty)
        || object.get("function_call").is_some_and(non_empty)
}

fn non_empty(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(value) => !value.trim().is_empty(),
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => !value.is_empty(),
        Value::Bool(_) | Value::Number(_) => true,
    }
}

fn cache_metrics(body: &ResponseBody) -> Vec<CacheMetric> {
    let mut metrics = Vec::new();
    if let Some(value) = body.prompt_cache_hit_tokens {
        metrics.push(CacheMetric {
            name: "prompt_cache_hit_tokens".into(),
            value: value.to_string(),
        });
    }
    for (name, value) in body.timings.iter().flatten() {
        if let Some(value) = value.as_f64() {
            metrics.push(CacheMetric {
                name: format!("timings.{name}"),
                value: value.to_string(),
            });
        }
    }
    metrics
}

fn usage_from_response(usage: Option<ResponseUsage>, metrics: &[CacheMetric]) -> CompletionUsage {
    let cached = usage
        .as_ref()
        .and_then(|value| value.prompt_tokens_details.as_ref())
        .and_then(|value| value.cached_tokens)
        .or_else(|| {
            usage
                .as_ref()
                .and_then(|value| value.cache_read_input_tokens)
        })
        .or_else(|| metric_u64(metrics, "prompt_cache_hit_tokens"));
    CompletionUsage {
        prompt_tokens: usage.as_ref().and_then(|value| value.prompt_tokens),
        completion_tokens: usage.as_ref().and_then(|value| value.completion_tokens),
        cached_prompt_tokens: cached,
        total_tokens: usage.and_then(|value| value.total_tokens),
    }
}

fn metric_u64(metrics: &[CacheMetric], name: &str) -> Option<u64> {
    metrics
        .iter()
        .find(|metric| metric.name == name)
        .and_then(|metric| metric.value.parse().ok())
}
