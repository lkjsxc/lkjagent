pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    Store(String),
    Llm(String),
    Prompt(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Store(message) => write!(formatter, "store error: {message}"),
            RuntimeError::Llm(message) => write!(formatter, "llm error: {message}"),
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
        Self::Llm(error.to_string())
    }
}
