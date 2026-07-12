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
    Unclosed,
}

impl ClosureMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Natural => "Natural",
            Self::Unclosed => "Unclosed",
        }
    }
}

pub fn classify_closure(content: &str) -> ClosureMode {
    let content = content.trim_end();
    if content.ends_with("</tool_call>") || content.ends_with("</final>") {
        ClosureMode::Natural
    } else {
        ClosureMode::Unclosed
    }
}
