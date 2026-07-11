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
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
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

pub fn restore_stop_suffix(
    content: String,
    finish_reason: &FinishReason,
    closing_tag: &str,
) -> (String, ClosureMode) {
    if content.contains(closing_tag) {
        return (content, ClosureMode::Natural);
    }
    if matches!(finish_reason, FinishReason::Stop) && opening_seen(&content, closing_tag) {
        return (
            format!("{content}{closing_tag}"),
            ClosureMode::StopSequenceClosed,
        );
    }
    (content, ClosureMode::Unclosed)
}

fn opening_seen(content: &str, closing_tag: &str) -> bool {
    let Some(name) = closing_tag
        .strip_prefix("</")
        .and_then(|value| value.strip_suffix('>'))
    else {
        return false;
    };
    content.contains(&format!("<{name}>"))
}
