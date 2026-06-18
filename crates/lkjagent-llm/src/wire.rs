mod metrics;

use lkjagent_context::model::{Message, Role};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use metrics::collect_cache_metrics;

pub const MAX_TOKENS: u16 = 1024;
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
    pub usage: CompletionUsage,
    pub cache_metrics: Vec<CacheMetric>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
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
    usage: ResponseUsage,
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
    prompt_tokens: u64,
    completion_tokens: u64,
}

pub fn build_request(model: &str, messages: &[Message]) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: messages.iter().map(ChatMessage::from_context).collect(),
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
        top_p: TOP_P,
        stop: Vec::new(),
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
    Ok(Completion {
        content: choice.message.content,
        finish_reason: finish_reason(choice.finish_reason),
        usage: CompletionUsage {
            prompt_tokens: body.usage.prompt_tokens,
            completion_tokens: body.usage.completion_tokens,
        },
        cache_metrics,
    })
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
