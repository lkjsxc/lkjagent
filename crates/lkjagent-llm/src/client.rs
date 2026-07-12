use std::io::Read;
use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;

use crate::error::{ClientError, ClientResult, EndpointFailure};
use crate::message::Message;
use crate::wire::{
    build_request, decode_completion, CallSpec, Completion, TransportOutcome, WireError, MAX_TOKENS,
};

pub const DEFAULT_TIMEOUT_SECONDS: u64 = 900;
pub const MAX_RESPONSE_BYTES: usize = 1_048_576;
pub const BACKOFF_CAP: Duration = Duration::from_secs(900);

pub fn delay_for_attempt(attempt: u32) -> Duration {
    let seconds = 1_u64.checked_shl(attempt).unwrap_or(BACKOFF_CAP.as_secs());
    Duration::from_secs(seconds.min(BACKOFF_CAP.as_secs()))
}

pub fn delays(count: usize) -> Vec<Duration> {
    (0..count)
        .map(|attempt| delay_for_attempt(attempt as u32))
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
pub struct ClientConfig {
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub max_tokens: u16,
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("timeout", &self.timeout)
            .field("max_tokens", &self.max_tokens)
            .finish()
    }
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            api_key: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECONDS),
            max_tokens: MAX_TOKENS,
        }
    }
}

pub fn request_json(
    config: &ClientConfig,
    messages: &[Message],
    spec: &CallSpec,
) -> ClientResult<String> {
    request_body(&config.model, messages, Duration::ZERO, spec)
}

pub fn complete(
    config: &ClientConfig,
    messages: &[Message],
    spec: &CallSpec,
    attempt: u32,
) -> ClientResult<Completion> {
    let started = Instant::now();
    let retry_after = delay_for_attempt(attempt);
    let client = Client::builder()
        .timeout(config.timeout)
        .build()
        .map_err(|_| endpoint_error(EndpointFailure::Transport, retry_after))?;
    let body = request_body(&config.model, messages, retry_after, spec)?;
    let response = send_request(&client, config, body, retry_after)?;
    let status = response.status();
    if status.is_client_error() {
        return Err(ClientError::EndpointOverflow {
            status: status.as_u16(),
        });
    }
    if !status.is_success() {
        return Err(endpoint_error(
            EndpointFailure::Status {
                status: status.as_u16(),
            },
            retry_after,
        ));
    }
    let text = read_bounded(response, retry_after)?;
    let response_bytes = u32::try_from(text.len()).unwrap_or(u32::MAX);
    let mut completion = decode_completion(&text, spec).map_err(|error| {
        let failure = match error {
            WireError::Json => EndpointFailure::MalformedJson,
            WireError::Shape(_) => EndpointFailure::MalformedShape,
        };
        endpoint_error(failure, retry_after)
    })?;
    completion.transport = Some(TransportOutcome {
        elapsed: started.elapsed(),
        response_bytes,
    });
    Ok(completion)
}

fn request_body(
    model: &str,
    messages: &[Message],
    retry_after: Duration,
    spec: &CallSpec,
) -> ClientResult<String> {
    serde_json::to_string(&build_request(model, messages, spec))
        .map_err(|_| endpoint_error(EndpointFailure::Transport, retry_after))
}

fn send_request(
    client: &Client,
    config: &ClientConfig,
    body: String,
    retry_after: Duration,
) -> ClientResult<Response> {
    let mut request = client
        .post(chat_url(&config.base_url))
        .header(CONTENT_TYPE, "application/json")
        .body(body);
    if let Some(api_key) = &config.api_key {
        request = request.bearer_auth(api_key);
    }
    request
        .send()
        .map_err(|error| endpoint_error(classify_transport(&error), retry_after))
}

fn read_bounded(response: Response, retry_after: Duration) -> ClientResult<String> {
    let mut bytes = Vec::new();
    response
        .take((MAX_RESPONSE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            let failure = if error.kind() == std::io::ErrorKind::TimedOut {
                EndpointFailure::Timeout
            } else {
                EndpointFailure::Transport
            };
            endpoint_error(failure, retry_after)
        })?;
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(endpoint_error(
            EndpointFailure::ResponseTooLarge {
                limit: MAX_RESPONSE_BYTES,
            },
            retry_after,
        ));
    }
    String::from_utf8(bytes)
        .map_err(|_| endpoint_error(EndpointFailure::MalformedJson, retry_after))
}

fn classify_transport(error: &reqwest::Error) -> EndpointFailure {
    if error.is_timeout() {
        return EndpointFailure::Timeout;
    }
    if error.is_connect() {
        let sources = format!("{error:?}").to_ascii_lowercase();
        if ["dns", "resolve", "lookup", "name or service"]
            .iter()
            .any(|term| sources.contains(term))
        {
            EndpointFailure::Dns
        } else {
            EndpointFailure::Connect
        }
    } else {
        EndpointFailure::Transport
    }
}

fn chat_url(base_url: &str) -> String {
    format!("{}/v1/chat/completions", base_url.trim_end_matches('/'))
}

fn endpoint_error(failure: EndpointFailure, retry_after: Duration) -> ClientError {
    ClientError::Endpoint {
        failure,
        retry_after,
    }
}
