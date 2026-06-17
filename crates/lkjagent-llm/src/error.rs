use std::time::Duration;

use crate::wire::{CacheMetric, CompletionUsage};

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointFailure {
    Connection(String),
    Malformed(String),
    Status { status: u16, body: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    Endpoint {
        failure: EndpointFailure,
        retry_after: Duration,
    },
    EndpointOverflow {
        status: u16,
        body: String,
    },
    Oversize {
        usage: CompletionUsage,
        cache_metrics: Vec<CacheMetric>,
    },
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Endpoint {
                failure,
                retry_after,
            } => {
                write!(
                    formatter,
                    "endpoint failure: {failure}; retry after {retry_after:?}"
                )
            }
            ClientError::EndpointOverflow { status, .. } => {
                write!(formatter, "endpoint overflow: HTTP {status}")
            }
            ClientError::Oversize { .. } => {
                formatter.write_str("endpoint completion hit max tokens")
            }
        }
    }
}

impl std::fmt::Display for EndpointFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointFailure::Connection(message) => write!(formatter, "connection: {message}"),
            EndpointFailure::Malformed(message) => {
                write!(formatter, "malformed response: {message}")
            }
            EndpointFailure::Status { status, .. } => write!(formatter, "HTTP {status}"),
        }
    }
}

impl std::error::Error for ClientError {}
