pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Sql(String),
    Busy(String),
    IncompatibleSchema { version: i64, objects: Vec<String> },
    NotFound(String),
    InvalidState(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Sql(message) => write!(formatter, "sqlite error: {message}"),
            StoreError::Busy(message) => write!(formatter, "sqlite busy: {message}"),
            StoreError::IncompatibleSchema { version, objects } => {
                write!(
                    formatter,
                    "incompatible schema version {version}: {}",
                    objects.join(", ")
                )
            }
            StoreError::NotFound(message) => write!(formatter, "not found: {message}"),
            StoreError::InvalidState(message) => write!(formatter, "invalid state: {message}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<rusqlite::Error> for StoreError {
    fn from(error: rusqlite::Error) -> Self {
        match &error {
            rusqlite::Error::SqliteFailure(code, _)
                if matches!(
                    code.code,
                    rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
                ) =>
            {
                StoreError::Busy(error.to_string())
            }
            _ => StoreError::Sql(error.to_string()),
        }
    }
}
