use lkjagent_protocol::{ACTION_CLOSE, ACTION_OPEN};

use crate::wire::FinishReason;

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

pub fn restore_stop_suffix(content: String, finish_reason: &FinishReason) -> (String, ClosureMode) {
    if content.contains(ACTION_CLOSE) {
        return (content, ClosureMode::Natural);
    }
    if matches!(finish_reason, FinishReason::Stop) && content.contains(ACTION_OPEN) {
        return (
            format!("{content}{ACTION_CLOSE}"),
            ClosureMode::StopSequenceClosed,
        );
    }
    (content, ClosureMode::Unclosed)
}
