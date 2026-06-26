use std::time::Instant;

use lkjagent_llm::error::ClientError;
use lkjagent_llm::wire::{Completion, CompletionUsage, FinishReason};

use crate::model_log::json_escape;

pub(super) fn completion_response_json(completion: &Completion) -> String {
    format!(
        "{{\"content\":\"{}\",\"provider_anomaly\":{},\"finish_reason\":\"{}\",\"closure_mode\":\"{}\",\"usage\":{}}}\n",
        json_escape(&completion.content),
        provider_anomaly_json(completion),
        finish_reason_name(&completion.finish_reason),
        completion.closure_mode.as_str(),
        usage_json(&completion.usage)
    )
}

pub(super) fn usage_json(usage: &CompletionUsage) -> String {
    format!(
        "{{\"prompt_tokens\":{},\"completion_tokens\":{},\"cached_prompt_tokens\":{},\"total_tokens\":{}}}",
        opt_u64(usage.prompt_tokens),
        opt_u64(usage.completion_tokens),
        opt_u64(usage.cached_prompt_tokens),
        opt_u64(usage.total_tokens)
    )
}

pub(super) fn finish_reason_name(reason: &FinishReason) -> &str {
    match reason {
        FinishReason::Stop => "stop",
        FinishReason::Length => "length",
        FinishReason::Other(_) => "other",
        FinishReason::Missing => "missing",
    }
}

pub(super) fn error_class(error: &ClientError) -> &str {
    match error {
        ClientError::Endpoint { .. } => "EndpointError",
        ClientError::EndpointOverflow { .. } => "EndpointOverflow",
        ClientError::Oversize { .. } => "CompletionOversize",
    }
}

pub(super) fn latency_ms(started: Instant) -> i64 {
    i64::try_from(started.elapsed().as_millis()).unwrap_or(i64::MAX)
}

fn provider_anomaly_json(completion: &Completion) -> String {
    completion.provider_anomaly.as_ref().map_or_else(
        || "null".to_string(),
        |anomaly| {
            format!(
                "{{\"kind\":\"{}\",\"detail\":\"{}\"}}",
                anomaly.kind.as_str(),
                json_escape(&anomaly.detail)
            )
        },
    )
}

fn opt_u64(value: Option<u64>) -> String {
    value.map_or_else(|| "null".to_string(), |value| value.to_string())
}
