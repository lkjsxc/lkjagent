use std::fmt;
use std::io;

pub type BenchResult<T> = Result<T, BenchError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchError {
    InvalidTask(String),
    UnknownTask(String),
    Io(String),
    Judge(String),
    Process(String),
}

impl BenchError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidTask(message)
            | Self::UnknownTask(message)
            | Self::Io(message)
            | Self::Judge(message)
            | Self::Process(message) => message,
        }
    }
}

impl fmt::Display for BenchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for BenchError {}

impl From<io::Error> for BenchError {
    fn from(error: io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
