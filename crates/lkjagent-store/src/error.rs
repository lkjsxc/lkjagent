pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Sql(String),
    NotFound(String),
    InvalidState(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Sql(message) => write!(formatter, "sqlite error: {message}"),
            StoreError::NotFound(message) => write!(formatter, "not found: {message}"),
            StoreError::InvalidState(message) => write!(formatter, "invalid state: {message}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<rusqlite::Error> for StoreError {
    fn from(error: rusqlite::Error) -> Self {
        StoreError::Sql(error.to_string())
    }
}
