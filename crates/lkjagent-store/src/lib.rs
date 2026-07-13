pub mod direct_transactions;

pub mod error {
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
                Self::Sql(message) => write!(formatter, "sqlite error: {message}"),
                Self::Busy(message) => write!(formatter, "sqlite busy: {message}"),
                Self::IncompatibleSchema { version, objects } => write!(
                    formatter,
                    "incompatible schema version {version}: {}",
                    objects.join(", ")
                ),
                Self::NotFound(message) => write!(formatter, "not found: {message}"),
                Self::InvalidState(message) => write!(formatter, "invalid state: {message}"),
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
                    Self::Busy(error.to_string())
                }
                _ => Self::Sql(error.to_string()),
            }
        }
    }
}

pub(crate) mod journal_obligations;
pub(crate) mod managed_record_obligations;
mod matter_control;
pub mod native_schema;
pub mod transactions;
pub mod tui_snapshot;
