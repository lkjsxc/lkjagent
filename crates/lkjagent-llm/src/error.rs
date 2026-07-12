use std::time::Duration;

use crate::wire::{CacheMetric, CompletionUsage};

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultClass {
    Timeout,
    Dns,
    Connect,
    Transport,
    HttpStatus,
    ResponseTooLarge,
    MalformedJson,
    MalformedShape,
    Length,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointFailure {
    Timeout,
    Dns,
    Connect,
    Transport,
    MalformedJson,
    MalformedShape,
    Status { status: u16 },
    ResponseTooLarge { limit: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    Endpoint {
        failure: EndpointFailure,
        retry_after: Duration,
    },
    EndpointOverflow {
        status: u16,
    },
    Oversize {
        usage: CompletionUsage,
        cache_metrics: Vec<CacheMetric>,
        preview: String,
    },
}

impl ClientError {
    pub fn fault_class(&self) -> FaultClass {
        match self {
            Self::Endpoint { failure, .. } => failure.fault_class(),
            Self::EndpointOverflow { .. } => FaultClass::HttpStatus,
            Self::Oversize { .. } => FaultClass::Length,
        }
    }
}

impl EndpointFailure {
    pub fn fault_class(&self) -> FaultClass {
        match self {
            Self::Timeout => FaultClass::Timeout,
            Self::Dns => FaultClass::Dns,
            Self::Connect => FaultClass::Connect,
            Self::Transport => FaultClass::Transport,
            Self::MalformedJson => FaultClass::MalformedJson,
            Self::MalformedShape => FaultClass::MalformedShape,
            Self::Status { .. } => FaultClass::HttpStatus,
            Self::ResponseTooLarge { .. } => FaultClass::ResponseTooLarge,
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Endpoint {
                failure,
                retry_after,
            } => {
                write!(
                    formatter,
                    "endpoint failure: {failure}; retry after {retry_after:?}"
                )
            }
            Self::EndpointOverflow { status } => write!(formatter, "endpoint HTTP {status}"),
            Self::Oversize { .. } => formatter.write_str("endpoint completion hit max tokens"),
        }
    }
}

impl std::fmt::Display for EndpointFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => formatter.write_str("timeout"),
            Self::Dns => formatter.write_str("DNS resolution"),
            Self::Connect => formatter.write_str("connect"),
            Self::Transport => formatter.write_str("transport"),
            Self::MalformedJson => formatter.write_str("malformed JSON"),
            Self::MalformedShape => formatter.write_str("malformed response shape"),
            Self::Status { status } => write!(formatter, "HTTP {status}"),
            Self::ResponseTooLarge { limit } => {
                write!(formatter, "response exceeds {limit} byte limit")
            }
        }
    }
}

impl std::error::Error for ClientError {}
