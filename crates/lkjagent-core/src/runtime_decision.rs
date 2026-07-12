use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
pub use crate::runtime_operation::{derive_harness_state, RuntimeHarnessState};
pub use crate::runtime_tool_view::{
    ToolExampleParam, ToolFieldSpec, ToolSetView, ToolValueClass, ToolViewEntry,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperationKey(pub String);

pub const DECISION_SPEC_FIELDS: &[&str] = &[
    "selected-state",
    "operation",
    "tool-descriptors",
    "grammar",
    "information-needs",
    "context-caps",
    "model-budget",
    "recovery-policy",
    "check-requirements",
    "exit-policy",
];
pub const DECISION_SETTLEMENTS: &[&str] = &[
    "selected",
    "compilation-complete",
    "provider-intent",
    "effect-prepared",
    "settled",
    "blocked",
];
pub const MODEL_GRAMMARS: &[&str] = &["tool-call", "final", "none"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputEnvelope {
    Content,
    Plan,
    Action,
    Message,
    Verdict,
    None,
}

impl RuntimeHarnessState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intake => "intake",
            Self::Clarify => "clarify",
            Self::Plan => "plan",
            Self::Act => "act",
            Self::Observe => "observe",
            Self::Recover => "recover",
            Self::Record => "record",
            Self::Maintain => "maintain",
            Self::Idle => "idle",
        }
    }

    pub fn purpose(self) -> &'static str {
        match self {
            Self::Intake => "classify owner turn and write transcript or inbox evidence",
            Self::Clarify => "ask or answer one bounded missing-information question",
            Self::Plan => "produce a bounded plan or content shape before effects",
            Self::Act => "execute selected model action, content write, or native effect",
            Self::Observe => "run checks and evaluate completion evidence",
            Self::Recover => "repair parse, admission, endpoint, effect, or check failure",
            Self::Record => "write owner-readable personal or work records",
            Self::Maintain => "rebuild indexes, rebalance paths, or collect proof",
            Self::Idle => "wait only when no executable unresolved work exists",
        }
    }

    pub fn prompt_fragment(self) -> String {
        format!(
            "Harness state: {}\nState purpose: {}\nContext policy: {}\nWorkspace policy: {}\nFailure policy: {}",
            self.as_str(),
            self.purpose(),
            self.context_policy(),
            self.workspace_policy(),
            self.failure_policy()
        )
    }

    fn context_policy(self) -> &'static str {
        match self {
            Self::Intake | Self::Record => "recent owner turn plus workspace maps",
            Self::Plan | Self::Act => "canonical docs plus selected workspace evidence",
            Self::Observe => "checks, artifacts, fingerprints, and proof refs",
            Self::Recover => "bounded fault diagnosis without raw failed output",
            Self::Clarify => "missing fact and prior question only",
            Self::Maintain => "workspace indexes, manifests, aliases, and proof refs",
            Self::Idle => "no model context unless new work arrives",
        }
    }

    fn workspace_policy(self) -> &'static str {
        match self {
            Self::Intake => "write transcript or inbox trace",
            Self::Record => "write record, history, fingerprint, README, and index evidence",
            Self::Act | Self::Maintain => "path-checked workspace effects only",
            _ => "read bounded selected workspace refs only",
        }
    }

    fn failure_policy(self) -> &'static str {
        match self {
            Self::Recover => "narrow tools and retry the smallest valid envelope",
            Self::Idle => "stay idle only with blocker, closure, or no-work evidence",
            _ => "write recovery.failure before any happy response",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectCommand {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDecision {
    pub id: String,
    pub case_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_state_key: Option<String>,
    #[serde(default = "default_harness_state")]
    pub harness_state: RuntimeHarnessState,
    pub operation: OperationKey,
    pub snapshot_fingerprint: String,
    pub state_vector_fingerprint: String,
    pub context_frame_fingerprint: String,
    pub tool_view: ToolSetView,
    pub expected_envelope: OutputEnvelope,
    pub model_budget_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_command: Option<EffectCommand>,
    pub evidence_requirements: Vec<String>,
    pub recovery_policy: String,
}

impl RuntimeDecision {
    pub fn new(
        id: impl Into<String>,
        case_id: impl Into<String>,
        operation: OperationKey,
        tool_view: ToolSetView,
        expected_envelope: OutputEnvelope,
    ) -> Self {
        let harness_state = derive_harness_state(None, &operation.0, expected_envelope, "default");
        Self {
            id: id.into(),
            case_id: case_id.into(),
            selected_state_key: None,
            harness_state,
            operation,
            snapshot_fingerprint: String::new(),
            state_vector_fingerprint: String::new(),
            context_frame_fingerprint: String::new(),
            tool_view,
            expected_envelope,
            model_budget_tokens: None,
            effect_command: None,
            evidence_requirements: Vec::new(),
            recovery_policy: "default".to_string(),
        }
    }

    pub fn refresh_harness_state(&mut self) {
        self.harness_state = derive_harness_state(
            self.selected_state_key.as_deref(),
            &self.operation.0,
            self.expected_envelope,
            &self.recovery_policy,
        );
    }

    pub fn tool_view_fingerprint(&self) -> Result<String, FingerprintError> {
        self.tool_view.fingerprint()
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}

fn default_harness_state() -> RuntimeHarnessState {
    RuntimeHarnessState::Idle
}
