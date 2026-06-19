pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    Store(String),
    Llm {
        message: String,
        retry_after_secs: Option<u64>,
    },
    CompletionOversize {
        preview: String,
    },
    Prompt(String),
}

impl RuntimeError {
    pub fn retry_after_secs(&self) -> Option<u64> {
        match self {
            RuntimeError::Llm {
                retry_after_secs, ..
            } => *retry_after_secs,
            _ => None,
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Store(message) => write!(formatter, "store error: {message}"),
            RuntimeError::Llm { message, .. } => write!(formatter, "llm error: {message}"),
            RuntimeError::CompletionOversize { preview } if preview.is_empty() => {
                formatter.write_str("llm error: endpoint completion hit max tokens")
            }
            RuntimeError::CompletionOversize { preview } => {
                write!(
                    formatter,
                    "llm error: endpoint completion hit max tokens; preview={preview}"
                )
            }
            RuntimeError::Prompt(message) => write!(formatter, "prompt error: {message}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<lkjagent_store::error::StoreError> for RuntimeError {
    fn from(error: lkjagent_store::error::StoreError) -> Self {
        Self::Store(error.to_string())
    }
}

impl From<lkjagent_llm::error::ClientError> for RuntimeError {
    fn from(error: lkjagent_llm::error::ClientError) -> Self {
        match error {
            lkjagent_llm::error::ClientError::Oversize { preview, .. } => {
                Self::CompletionOversize { preview }
            }
            lkjagent_llm::error::ClientError::Endpoint {
                failure,
                retry_after,
            } => Self::Llm {
                message: format!("endpoint failure: {failure}; retry after {retry_after:?}"),
                retry_after_secs: Some(retry_after.as_secs()),
            },
            other => Self::Llm {
                message: other.to_string(),
                retry_after_secs: None,
            },
        }
    }
}
