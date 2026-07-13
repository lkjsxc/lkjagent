use crate::runtime_decision::{EffectCommand, OutputEnvelope, ToolSetView};
use crate::runtime_state::RuntimeSnapshot;
use serde::{Deserialize, Serialize};
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatterLifecycle { Open, Waiting, Blocked, Closed }
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimePhase { Orient, Modify, Review, Respond, Idle }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeState {
    pub snapshot: RuntimeSnapshot,
    pub causal_sequence: u64,
    pub lifecycle: MatterLifecycle,
    pub phase: RuntimePhase,
    pub obligations: Vec<String>,
}

#[rustfmt::skip]
impl RuntimeState {
    pub fn from_snapshot(snapshot: RuntimeSnapshot) -> Self {
        let causal_sequence = snapshot.cells.values()
            .filter_map(|cell| crate::runtime_eligibility::causal_number(&cell.source_event_id)).max().unwrap_or(0);
        let mut state = Self { snapshot, causal_sequence, lifecycle: MatterLifecycle::Open,
            phase: RuntimePhase::Orient, obligations: Vec::new() };
        crate::runtime_event::derive_projections(&mut state); state
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeHarnessState { Clarify, Act, Observe, Recover, Idle }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeOperation {
    pub key: String,
    pub expected_envelope: OutputEnvelope,
    pub tool_view: ToolSetView,
    pub model_budget_tokens: Option<u32>,
    pub effect_command: Option<EffectCommand>,
    pub evidence_requirements: Vec<String>,
    pub recovery_policy: String,
}

impl RuntimeOperation {
    pub fn idle() -> Self {
        Self {
            key: "runtime.idle".into(),
            expected_envelope: OutputEnvelope::None,
            tool_view: ToolSetView::empty(),
            model_budget_tokens: None,
            effect_command: None,
            evidence_requirements: Vec::new(),
            recovery_policy: "none".into(),
        }
    }

    pub fn model_call(
        key: impl Into<String>,
        expected_envelope: OutputEnvelope,
        tool_view: ToolSetView,
        model_budget_tokens: Option<u32>,
        evidence_requirements: Vec<String>,
    ) -> Self {
        Self {
            key: key.into(),
            expected_envelope,
            tool_view,
            model_budget_tokens,
            effect_command: None,
            evidence_requirements,
            recovery_policy: "retry-same-decision".into(),
        }
    }

    pub fn model_free(key: impl Into<String>, evidence_requirements: Vec<String>) -> Self {
        Self::model_free_effect(key, evidence_requirements, None)
    }

    pub fn model_free_effect(
        key: impl Into<String>,
        evidence_requirements: Vec<String>,
        effect_command: Option<EffectCommand>,
    ) -> Self {
        Self {
            key: key.into(),
            expected_envelope: OutputEnvelope::None,
            tool_view: ToolSetView::empty(),
            model_budget_tokens: None,
            effect_command,
            evidence_requirements,
            recovery_policy: "commit-or-recover".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePolicy {
    pub model_budget_tokens: u32,
    pub prior_progress_fingerprint: Option<String>,
    pub current_progress_fingerprint: Option<String>,
    pub recovery_attempt: usize,
    pub intended_recovery_tool: Option<String>,
}

impl Default for RuntimePolicy {
    fn default() -> Self {
        Self {
            model_budget_tokens: 1_024,
            prior_progress_fingerprint: None,
            current_progress_fingerprint: None,
            recovery_attempt: 0,
            intended_recovery_tool: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDecisionSpec {
    pub phase: RuntimePhase,
    pub operation_key: String,
    pub causal_sequence: u64,
    pub model_required: bool,
    pub expected_envelope: OutputEnvelope,
    pub tool_view: ToolSetView,
    pub model_budget_tokens: Option<u32>,
    pub recovery_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WakeCondition {
    OwnerInput { matter_id: String },
    Time { at: String },
    FileChange { revision: String },
    ConfigChange,
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeWait { pub reason: String, pub wake: WakeCondition }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockReason {
    Conflict(Vec<String>),
    MissingEvidence(Vec<String>),
    InvalidCooldown(String),
    Stasis,
    UnsettledEffects(Vec<String>),
    UnknownExecutable(String),
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeBlock { pub reason: BlockReason }

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Selection { Decision(RuntimeDecisionSpec), Wait(RuntimeWait), Block(RuntimeBlock), Idle }

#[rustfmt::skip]
pub fn derive_harness_state(selected_state_key: Option<&str>, operation: &str,
    envelope: OutputEnvelope, recovery_policy: &str) -> RuntimeHarnessState {
    let namespace = selected_state_key.and_then(|label| label.split_once(':').map(|parts| parts.0));
    if namespace == Some("recovery") || operation.starts_with("recovery.") {
        return RuntimeHarnessState::Recover;
    }
    if operation.starts_with("check.") || operation.starts_with("completion.") {
        return RuntimeHarnessState::Observe;
    }
    if operation == "runtime.idle"
        || envelope == OutputEnvelope::None && recovery_policy == "none"
    {
        return RuntimeHarnessState::Idle;
    }
    match envelope {
        OutputEnvelope::Message => RuntimeHarnessState::Clarify,
        OutputEnvelope::Action | OutputEnvelope::None => RuntimeHarnessState::Act,
    }
}
