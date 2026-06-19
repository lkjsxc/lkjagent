pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    Store(String),
    Llm(String),
    CompletionOversize,
    Prompt(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Store(message) => write!(formatter, "store error: {message}"),
            RuntimeError::Llm(message) => write!(formatter, "llm error: {message}"),
            RuntimeError::CompletionOversize => {
                formatter.write_str("llm error: endpoint completion hit max tokens")
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
            lkjagent_llm::error::ClientError::Oversize { .. } => Self::CompletionOversize,
            other => Self::Llm(other.to_string()),
        }
    }
}
