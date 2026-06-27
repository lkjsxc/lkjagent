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
    pub last_output_kind: Option<OutputKind>,
    pub next_frame_ref: usize,
    pub repeat_count: usize,
    pub reads: Vec<ReadRecord>,
    pub graph_state: Option<String>,
    pub graph_evidence: Vec<GraphEvidenceRecord>,
    pub graph_completion_ready: bool,
    pub graph_missing: Vec<String>,
    pub graph_policy: Option<GraphDispatchPolicy>,
    pub effective_policy: Option<EffectivePolicy>,
    pub authority_view: Option<AuthorityAdmissionView>,
    pub control: ControlState,
}

impl Default for DispatchState {
    fn default() -> Self {
        Self {
            last_action_text: None,
            last_frame_ref: None,
            last_output_kind: None,
            next_frame_ref: 1,
            repeat_count: 0,
            reads: Vec::new(),
            graph_state: None,
            graph_evidence: Vec::new(),
            graph_completion_ready: true,
            graph_missing: Vec::new(),
            graph_policy: None,
            effective_policy: None,
            authority_view: None,
            control: ControlState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectivePolicy {
    pub mode: String,
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub shell_allowed: bool,
    pub completion_allowed: bool,
    pub reason: String,
    pub preferred_next_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityAdmissionView {
    pub decision_id: String,
    pub case_id: String,
    pub authority_fingerprint: String,
    pub active_mission: String,
    pub active_node: String,
    pub admitted_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub shell_allowed: bool,
    pub completion_allowed: bool,
    pub missing_evidence: Vec<String>,
    pub recovery_route: Option<String>,
    pub exact_valid_example: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDispatchPolicy {
    pub active_node: String,
    pub phase: String,
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub allowed_packages: Vec<String>,
    pub legal_transitions: Vec<String>,
    pub evidence_requirements: Vec<String>,
    pub blocked_reason: Option<String>,
    pub plan_ready: bool,
    pub completion_ready: bool,
    pub shell_allowed: bool,
}

impl DispatchState {
    pub fn reset_repeat_tracking(&mut self) {
        self.last_action_text = None;
        self.last_frame_ref = None;
        self.last_output_kind = None;
        self.repeat_count = 0;
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
