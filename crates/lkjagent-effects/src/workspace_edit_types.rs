use crate::error::EffectError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Revision {
    Sha256(String),
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileValue {
    pub bytes: Vec<u8>,
    pub revision: String,
    pub mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservedTarget {
    Present(FileValue),
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedEdit {
    pub path: String,
    pub prior_bytes: Option<Vec<u8>>,
    pub intended_bytes: Vec<u8>,
    pub expected_mode: Option<u32>,
    pub intended_mode: u32,
    pub stage_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedPathCreate {
    pub edit: PreparedEdit,
    pub missing_parents: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurablePhase {
    Staged,
    Exchanged,
    Settled,
    Compensated,
    Cleaned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Prepared,
    Staged,
    Exchanged,
    ThirdValue,
    ModeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifiedOutcome {
    Settled,
    Compensated,
}

#[derive(Debug)]
pub enum EditError {
    Effect(EffectError),
    Conflict(&'static str),
    Unsupported,
}

pub type EditResult<T> = Result<T, EditError>;

impl std::fmt::Display for EditError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Effect(error) => write!(formatter, "{error}"),
            Self::Conflict(message) => write!(formatter, "conflict: {message}"),
            Self::Unsupported => write!(formatter, "unsupported atomic edit"),
        }
    }
}

impl std::error::Error for EditError {}

impl From<EffectError> for EditError {
    fn from(value: EffectError) -> Self {
        Self::Effect(value)
    }
}

pub fn classify(edit: &PreparedEdit, target: &ObservedTarget, stage: &ObservedTarget) -> Layout {
    let prior = edit.prior_bytes.as_deref().zip(edit.expected_mode);
    let intended = Some((edit.intended_bytes.as_slice(), edit.intended_mode));
    let target = pair(target);
    let stage = pair(stage);
    if changed_mode(target, prior)
        || changed_mode(target, intended)
        || changed_mode(stage, intended)
    {
        return Layout::ModeMismatch;
    }
    match (target, stage) {
        (target, None) if target == prior => Layout::Prepared,
        (target, stage) if target == prior && stage == intended => Layout::Staged,
        (target, stage) if target == intended && stage == prior => Layout::Exchanged,
        _ => Layout::ThirdValue,
    }
}

fn pair(value: &ObservedTarget) -> Option<(&[u8], u32)> {
    match value {
        ObservedTarget::Present(value) => Some((&value.bytes, value.mode)),
        ObservedTarget::Absent => None,
    }
}

fn changed_mode(left: Option<(&[u8], u32)>, right: Option<(&[u8], u32)>) -> bool {
    matches!((left, right), (Some(a), Some(b)) if a.0 == b.0 && a.1 != b.1)
}

pub(crate) fn exact_matches(text: &str, old: &str) -> usize {
    text.as_bytes()
        .windows(old.len())
        .filter(|part| *part == old.as_bytes())
        .count()
}
