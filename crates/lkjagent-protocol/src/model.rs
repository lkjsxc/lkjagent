pub const ACTION_OPEN: &str = "<action>";
pub const ACTION_CLOSE: &str = "</action>";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub tool: String,
    pub params: Vec<Param>,
}

impl Action {
    pub fn new(tool: impl Into<String>, params: Vec<Param>) -> Self {
        Self {
            tool: tool.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

impl Param {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParseSettings {
    pub allow_implicit_envelope: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOutcome {
    pub action: Option<Action>,
    pub fault: Option<ParseFault>,
    pub envelope_mode: EnvelopeMode,
    pub normalized_text_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeMode {
    Natural,
    StopClosed,
    Implicit,
    Unclosed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MalformedTagReason {
    AttributeSyntax,
    BadAngleSyntax,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseFault {
    MissingActionEnvelope,
    MultipleActionEnvelopes,
    UnclosedActionEnvelope,
    MissingTool,
    UnknownTool {
        tool: String,
    },
    UnclosedTag {
        tag: String,
    },
    DuplicateParam {
        name: String,
    },
    MalformedTag {
        line: String,
        reason: MalformedTagReason,
    },
    AttributeLikeTag {
        tag_name: String,
        value_hint: Option<String>,
    },
    BadParams {
        tool: String,
        missing: Vec<String>,
        unknown: Vec<String>,
    },
    BadEnvelope {
        reason: String,
    },
    JsonActionRejected,
}
