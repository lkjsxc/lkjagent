use std::time::Duration;

use lkjagent_context::model::Message;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

use crate::backoff::delay_for_attempt;
use crate::error::{ClientError, ClientResult, EndpointFailure};
use crate::wire::{build_request, decode_completion, Completion, FinishReason, MAX_TOKENS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub max_tokens: u16,
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            api_key: None,
            timeout: Duration::from_secs(60),
            max_tokens: MAX_TOKENS,
        }
    }
}

pub fn request_json(config: &ClientConfig, messages: &[Message]) -> ClientResult<String> {
    request_body(
        &config.model,
        messages,
        Duration::from_secs(0),
        config.max_tokens,
    )
}

pub fn complete(
    config: &ClientConfig,
    messages: &[Message],
    attempt: u32,
) -> ClientResult<Completion> {
    let retry_after = delay_for_attempt(attempt);
    let client = Client::builder()
        .timeout(config.timeout)
        .build()
        .map_err(|error| {
            endpoint_error(EndpointFailure::Connection(error.to_string()), retry_after)
        })?;
    let body = request_body(&config.model, messages, retry_after, config.max_tokens)?;
    let response = send_request(&client, config, body, retry_after)?;
    let status = response.status();
    let text = response.text().map_err(|error| {
        endpoint_error(EndpointFailure::Connection(error.to_string()), retry_after)
    })?;
    if status.is_client_error() {
        return Err(ClientError::EndpointOverflow {
            status: status.as_u16(),
            body: text,
        });
    }
    if !status.is_success() {
        let failure = EndpointFailure::Status {
            status: status.as_u16(),
            body: text,
        };
        return Err(endpoint_error(failure, retry_after));
    }
    let completion = decode_completion(&text).map_err(|error| {
        endpoint_error(EndpointFailure::Malformed(error.to_string()), retry_after)
    })?;
    if completion.finish_reason == FinishReason::Length && !has_closed_act(&completion.content) {
        let preview = preview(&completion.content);
        return Err(ClientError::Oversize {
            usage: completion.usage,
            cache_metrics: completion.cache_metrics,
            preview,
        });
    }
    Ok(completion)
}

fn request_body(
    model: &str,
    messages: &[Message],
    retry_after: Duration,
    max_tokens: u16,
) -> ClientResult<String> {
    let request = build_request(model, messages, max_tokens);
    serde_json::to_string(&request)
        .map_err(|error| endpoint_error(EndpointFailure::Malformed(error.to_string()), retry_after))
}

fn send_request(
    client: &Client,
    config: &ClientConfig,
    body: String,
    retry_after: Duration,
) -> ClientResult<reqwest::blocking::Response> {
    let mut request = client
        .post(chat_url(&config.base_url))
        .header(CONTENT_TYPE, "application/json")
        .body(body);
    if let Some(api_key) = &config.api_key {
        request = request.bearer_auth(api_key);
    }
    request.send().map_err(|error| {
        endpoint_error(EndpointFailure::Connection(error.to_string()), retry_after)
    })
}

fn chat_url(base_url: &str) -> String {
    format!("{}/v1/chat/completions", base_url.trim_end_matches('/'))
}

fn has_closed_act(content: &str) -> bool {
    content
        .find("<act>")
        .is_some_and(|start| content[start..].contains("</act>"))
}

fn preview(content: &str) -> String {
    content
        .chars()
        .take(240)
        .collect::<String>()
        .replace('\n', "\\n")
}

fn endpoint_error(failure: EndpointFailure, retry_after: Duration) -> ClientError {
    ClientError::Endpoint {
        failure,
        retry_after,
    }
}
