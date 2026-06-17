#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
    pub code: i32,
}

impl CliError {
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 2,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 1,
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        Self::failure(error.to_string())
    }
}

impl From<lkjagent_store::error::StoreError> for CliError {
    fn from(error: lkjagent_store::error::StoreError) -> Self {
        Self::failure(error.to_string())
    }
}

impl From<rusqlite::Error> for CliError {
    fn from(error: rusqlite::Error) -> Self {
        Self::failure(error.to_string())
    }
}

impl From<lkjagent_runtime::error::RuntimeError> for CliError {
    fn from(error: lkjagent_runtime::error::RuntimeError) -> Self {
        Self::failure(error.to_string())
    }
}
