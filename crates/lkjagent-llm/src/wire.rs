mod metrics;

use lkjagent_context::model::{Message, Role};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use lkjagent_protocol::ACTION_CLOSE;

use crate::closure::{restore_stop_suffix, ClosureMode};
use metrics::collect_cache_metrics;

pub const MAX_TOKENS: u16 = 2048;
pub const TEMPERATURE: f32 = 0.3;
pub const TOP_P: f32 = 0.9;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u16,
    pub temperature: f32,
    pub top_p: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    pub stream: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChatMessage {
    pub role: &'static str,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub content: String,
    pub finish_reason: FinishReason,
    pub closure_mode: ClosureMode,
    pub usage: CompletionUsage,
    pub cache_metrics: Vec<CacheMetric>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionUsage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub cached_prompt_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheMetric {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishReason {
    Stop,
    Length,
    Other(String),
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireError {
    Json(String),
    Missing(&'static str),
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    choices: Vec<ResponseChoice>,
    usage: Option<ResponseUsage>,
}

#[derive(Debug, Deserialize)]
struct ResponseChoice {
    message: ResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ResponseUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct PromptTokensDetails {
    cached_tokens: Option<u64>,
}

pub fn build_request(model: &str, messages: &[Message], max_tokens: u16) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: messages.iter().map(ChatMessage::from_context).collect(),
        max_tokens,
        temperature: TEMPERATURE,
        top_p: TOP_P,
        stop: vec![ACTION_CLOSE.to_string()],
        stream: false,
    }
}

pub fn decode_completion(text: &str) -> Result<Completion, WireError> {
    let value: Value =
        serde_json::from_str(text).map_err(|error| WireError::Json(error.to_string()))?;
    let cache_metrics = collect_cache_metrics(&value);
    let body: ResponseBody =
        serde_json::from_value(value).map_err(|error| WireError::Json(error.to_string()))?;
    let choice = body
        .choices
        .into_iter()
        .next()
        .ok_or(WireError::Missing("choices[0]"))?;
    let finish_reason = finish_reason(choice.finish_reason);
    let (content, closure_mode) = restore_stop_suffix(choice.message.content, &finish_reason);
    Ok(Completion {
        content,
        finish_reason,
        closure_mode,
        usage: usage_from_response(body.usage, &cache_metrics),
        cache_metrics,
    })
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

impl ChatMessage {
    fn from_context(message: &Message) -> Self {
        Self {
            role: role_name(message.role),
            content: message.content.clone(),
        }
    }
}

impl std::fmt::Display for WireError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WireError::Json(message) => write!(formatter, "json: {message}"),
            WireError::Missing(field) => write!(formatter, "missing {field}"),
        }
    }
}

impl std::error::Error for WireError {}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::Assistant => "assistant",
        Role::User => "user",
    }
}

fn finish_reason(reason: Option<String>) -> FinishReason {
    match reason.as_deref() {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some(value) => FinishReason::Other(value.to_string()),
        None => FinishReason::Missing,
    }
}
