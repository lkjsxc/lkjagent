use std::path::PathBuf;

use lkjagent_context::budget::LOG_OBSERVATION;

use crate::control::ControlState;
use crate::observe::OutputKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRuntime {
    pub workspace: PathBuf,
    pub now: String,
    pub observation_tokens: usize,
    pub shell_timeout_max: u64,
}

impl ToolRuntime {
    pub fn new(workspace: PathBuf, now: impl Into<String>) -> Self {
        Self {
            workspace,
            now: now.into(),
            observation_tokens: LOG_OBSERVATION,
            shell_timeout_max: 600,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchState {
    pub last_action_text: Option<String>,
    pub last_frame_ref: Option<usize>,
    pub next_frame_ref: usize,
    pub repeat_count: usize,
    pub reads: Vec<ReadRecord>,
    pub graph_state: Option<String>,
    pub graph_evidence: Vec<GraphEvidenceRecord>,
    pub graph_completion_ready: bool,
    pub graph_missing: Vec<String>,
    pub control: ControlState,
}

impl Default for DispatchState {
    fn default() -> Self {
        Self {
            last_action_text: None,
            last_frame_ref: None,
            next_frame_ref: 1,
            repeat_count: 0,
            reads: Vec::new(),
            graph_state: None,
            graph_evidence: Vec::new(),
            graph_completion_ready: true,
            graph_missing: Vec::new(),
            control: ControlState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadRecord {
    pub path: String,
    pub start: usize,
    pub count: usize,
    pub total_lines: usize,
    pub body: String,
    pub frame_ref: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEvidenceRecord {
    pub kind: String,
    pub summary: String,
    pub path: Option<String>,
    pub frame_ref: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchOutput {
    pub frame_ref: usize,
    pub kind: OutputKind,
    pub content: String,
    pub rendered: String,
}
