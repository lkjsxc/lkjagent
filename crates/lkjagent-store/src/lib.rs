pub mod admission_rows;
pub mod artifact_rows;
pub mod context_rows;
pub mod decision_rows;
pub mod direct_transactions;
pub mod effect_recovery;
pub mod event_rows;

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
pub mod exchange_rows {
    pub use crate::decision_rows::{insert_provider_exchange, ProviderExchangeRow};
}
pub mod memory;
pub mod native_schema;
pub mod observation_rows;
pub mod plan_access;
pub mod plan_commit;
pub mod plan_hydrate;
pub mod plan_inspect {
    pub use crate::plan_access::application_tables;
}
mod plan_migrations;
pub mod plan_names;
pub mod plan_schema;
pub mod prompt_rows;
mod queue_access;
pub mod record_rows;
mod record_schema;
pub mod row_support;
pub mod state_edge_rows;
pub mod state_rows;
pub mod state_schema;
pub mod token_usage;
pub mod transactions;
pub mod workspace_rows;
pub mod workspace_search;
