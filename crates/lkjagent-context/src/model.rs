#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenCount(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixSection {
    Identity,
    GrammarRegistry,
    SkillIndex,
    WorkspaceBrief,
    MemoryDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoticeKind {
    Truncation,
    Budget,
    Error,
    Compaction,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameKind {
    Prefix(PrefixSection),
    ModelTurn,
    Observation,
    Owner,
    Notice(NoticeKind),
    SkillBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub kind: FrameKind,
    pub content: String,
    pub tokens: TokenCount,
}

impl Frame {
    pub fn new(kind: FrameKind, content: impl Into<String>, tokens: usize) -> Self {
        Self {
            kind,
            content: content.into(),
            tokens: TokenCount(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextState {
    pub prefix: Vec<Frame>,
    pub log: Vec<Frame>,
}

impl ContextState {
    pub fn new(prefix: Vec<Frame>, log: Vec<Frame>) -> Self {
        Self { prefix, log }
    }

    pub fn used_tokens(&self) -> usize {
        self.prefix
            .iter()
            .chain(self.log.iter())
            .map(|frame| frame.tokens.0)
            .sum()
    }
}
