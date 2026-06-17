use crate::budget::{LOG_LOADED_SKILLS, LOG_OBSERVATION, LOG_OWNER_FRAME, LOG_SKILL_BODY};
use crate::model::{Frame, FrameKind, NoticeKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingFrame {
    pub kind: FrameKind,
    pub content: String,
    pub tokens: usize,
    pub retrieval_path: Option<String>,
}

impl PendingFrame {
    pub fn new(kind: FrameKind, content: impl Into<String>, tokens: usize) -> Self {
        Self {
            kind,
            content: content.into(),
            tokens,
            retrieval_path: None,
        }
    }

    pub fn with_retrieval_path(mut self, path: impl Into<String>) -> Self {
        self.retrieval_path = Some(path.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionDecision {
    Admit(Frame),
    Truncate { frame: Frame, notice: Frame },
    Refuse { notice: Frame },
}

pub fn admit(frame: PendingFrame) -> AdmissionDecision {
    let cap = cap_for(&frame.kind);
    if frame.tokens <= cap {
        return AdmissionDecision::Admit(Frame::new(frame.kind, frame.content, frame.tokens));
    }
    match frame.kind {
        FrameKind::Owner | FrameKind::Observation => truncate(frame, cap),
        FrameKind::SkillBody => {
            refuse("skill body exceeds 2,048 tokens; use skill.save with a smaller body")
        }
        _ => refuse("frame exceeds its budget owner cap"),
    }
}

pub fn admit_loaded_skill(current_tokens: usize, frame: PendingFrame) -> AdmissionDecision {
    if current_tokens.saturating_add(frame.tokens) > LOG_LOADED_SKILLS {
        refuse("loaded skills exceed 6,144 tokens; reload after compaction")
    } else {
        admit(frame)
    }
}

fn truncate(frame: PendingFrame, cap: usize) -> AdmissionDecision {
    let Some(path) = frame.retrieval_path else {
        return refuse("truncation requires a retrieval path notice");
    };
    let notice = Frame::new(
        FrameKind::Notice(NoticeKind::Truncation),
        format!("truncated over-budget frame; retrieve the rest with {path}"),
        32,
    );
    AdmissionDecision::Truncate {
        frame: Frame::new(frame.kind, frame.content, cap),
        notice,
    }
}

fn refuse(message: &str) -> AdmissionDecision {
    AdmissionDecision::Refuse {
        notice: Frame::new(FrameKind::Notice(NoticeKind::Error), message, 32),
    }
}

fn cap_for(kind: &FrameKind) -> usize {
    match kind {
        FrameKind::Owner => LOG_OWNER_FRAME,
        FrameKind::Observation => LOG_OBSERVATION,
        FrameKind::SkillBody => LOG_SKILL_BODY,
        _ => usize::MAX,
    }
}
