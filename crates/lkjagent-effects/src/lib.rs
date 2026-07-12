pub mod checks;
pub mod shell;

pub mod error {
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
}
pub mod workspace;
mod workspace_capability;
pub mod workspace_edit;
mod workspace_scan;

mod workspace_edit_types;

pub mod observation {
    const OBSERVATION_CHARS: usize = 6_000;

    pub fn observation(status: &str, content: &str) -> String {
        format!(
            "<observation>\n<status>{status}</status>\n<content>\n{}\n</content>\n</observation>",
            bound(content, OBSERVATION_CHARS)
        )
    }

    pub fn bound(text: &str, cap: usize) -> String {
        if text.len() <= cap {
            return text.to_string();
        }
        let keep = cap.saturating_sub(5) / 2;
        let head = text.chars().take(keep).collect::<String>();
        let tail = text
            .chars()
            .rev()
            .take(keep)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        format!("{head}[...]{tail}")
    }
}
