use crate::wire::FinishReason;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    Assistant,
    User,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureMode {
    Natural,
    StopSequenceClosed,
    Unclosed,
}

impl ClosureMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Natural => "Natural",
            Self::StopSequenceClosed => "StopSequenceClosed",
            Self::Unclosed => "Unclosed",
        }
    }
}

/// Classifies content without repairing provider output.
pub fn restore_stop_suffix(
    content: String,
    _finish_reason: &FinishReason,
    closing_tag: &str,
) -> (String, ClosureMode) {
    let mode = if content.contains(closing_tag) {
        ClosureMode::Natural
    } else {
        ClosureMode::Unclosed
    };
    (content, mode)
}
