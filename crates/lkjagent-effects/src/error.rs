pub type EffectResult<T> = Result<T, EffectError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    Path(String),
    Io(String),
    Utf8(String),
    Timeout(String),
    Invalid(String),
    Bound(String),
    Unsafe(String),
}

impl std::fmt::Display for EffectError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, message) = match self {
            Self::Path(message) => ("path", message),
            Self::Io(message) => ("io", message),
            Self::Utf8(message) => ("utf8", message),
            Self::Timeout(message) => ("timeout", message),
            Self::Invalid(message) => ("invalid", message),
            Self::Bound(message) => ("bound", message),
            Self::Unsafe(message) => ("unsafe", message),
        };
        write!(formatter, "{kind}: {message}")
    }
}

impl std::error::Error for EffectError {}

impl From<std::io::Error> for EffectError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
