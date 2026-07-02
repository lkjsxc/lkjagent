pub type EffectResult<T> = Result<T, EffectError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    Path(String),
    Io(String),
    Utf8(String),
    Timeout(String),
    Invalid(String),
}

impl std::fmt::Display for EffectError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(message) => write!(formatter, "path: {message}"),
            Self::Io(message) => write!(formatter, "io: {message}"),
            Self::Utf8(message) => write!(formatter, "utf8: {message}"),
            Self::Timeout(message) => write!(formatter, "timeout: {message}"),
            Self::Invalid(message) => write!(formatter, "invalid: {message}"),
        }
    }
}

impl std::error::Error for EffectError {}

impl From<std::io::Error> for EffectError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
