mod response;
mod spec;

use std::time::Duration;

use serde::Serialize;
use serde_json::Value;

use crate::closure::{classify_closure, ClosureMode};
use crate::message::{Message, Role};
pub use spec::CallSpec;

pub const MAX_TOKENS: u16 = 512;
pub const TEMPERATURE: f32 = 0.3;
pub const TOP_P: f32 = 0.9;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u16,
    pub temperature: f32,
    pub top_p: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
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
    pub provider_anomaly: Option<ProviderAnomaly>,
    pub transport: Option<TransportOutcome>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportOutcome {
    pub elapsed: Duration,
    pub response_bytes: u32,
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
pub struct ProviderAnomaly {
    pub kind: ProviderAnomalyKind,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderAnomalyKind {
    EmptyContentWithUsage,
    EmptyContentNoUsage,
    MissingContentField,
    ReasoningOnlyResponse,
    MalformedProviderMessage,
    ToolCallOnlyResponse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireError {
    Json,
    Shape(&'static str),
}

pub fn build_request(model: &str, messages: &[Message], spec: &CallSpec) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: messages.iter().map(ChatMessage::from_context).collect(),
        max_tokens: spec.max_tokens,
        temperature: spec.temperature,
        top_p: spec.top_p,
        reasoning_effort: spec.reasoning_effort.clone(),
        stop: spec.stop.clone(),
        stream: false,
    }
}

pub fn decode_completion(text: &str, _spec: &CallSpec) -> Result<Completion, WireError> {
    let value: Value = serde_json::from_str(text).map_err(|_| WireError::Json)?;
    let parts = response::response_parts(value)?;
    let closure_mode = classify_closure(&parts.content);
    Ok(Completion {
        content: parts.content,
        finish_reason: parts.finish_reason,
        closure_mode,
        usage: parts.usage,
        cache_metrics: parts.cache_metrics,
        provider_anomaly: parts.anomaly,
        transport: None,
    })
}

impl ProviderAnomaly {
    pub fn new(kind: ProviderAnomalyKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

impl ProviderAnomalyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyContentWithUsage => "empty_content_with_usage",
            Self::EmptyContentNoUsage => "empty_content_no_usage",
            Self::MissingContentField => "missing_content_field",
            Self::ReasoningOnlyResponse => "reasoning_only_response",
            Self::MalformedProviderMessage => "malformed_provider_message",
            Self::ToolCallOnlyResponse => "tool_call_only_response",
        }
    }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => f.write_str("malformed JSON"),
            Self::Shape(field) => write!(f, "malformed response shape at {field}"),
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
