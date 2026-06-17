pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    InvalidParam(String),
    Io(String),
    Store(String),
    Skill(String),
    MissingStore,
}

impl ToolError {
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::InvalidParam(message.into())
    }
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::InvalidParam(message) => write!(formatter, "invalid parameter: {message}"),
            ToolError::Io(message) => write!(formatter, "io error: {message}"),
            ToolError::Store(message) => write!(formatter, "store error: {message}"),
            ToolError::Skill(message) => write!(formatter, "skill error: {message}"),
            ToolError::MissingStore => write!(formatter, "store connection required"),
        }
    }
}

impl std::error::Error for ToolError {}

impl From<std::io::Error> for ToolError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl From<lkjagent_store::error::StoreError> for ToolError {
    fn from(error: lkjagent_store::error::StoreError) -> Self {
        Self::Store(error.to_string())
    }
}
