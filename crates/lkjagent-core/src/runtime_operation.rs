use serde::{Deserialize, Serialize};

use crate::runtime_decision::{EffectCommand, OutputEnvelope, ToolSetView};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeHarnessState {
    Intake,
    Clarify,
    Plan,
    Act,
    Observe,
    Recover,
    Record,
    Maintain,
    Idle,
}

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
            key: "runtime.idle".to_string(),
            expected_envelope: OutputEnvelope::None,
            tool_view: ToolSetView::empty(),
            model_budget_tokens: None,
            effect_command: None,
            evidence_requirements: Vec::new(),
            recovery_policy: "none".to_string(),
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
            recovery_policy: "retry-same-decision".to_string(),
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
            recovery_policy: "commit-or-recover".to_string(),
        }
    }
}

pub fn derive_harness_state(
    selected_state_key: Option<&str>,
    operation: &str,
    envelope: OutputEnvelope,
    recovery_policy: &str,
) -> RuntimeHarnessState {
    let ns = selected_state_key.and_then(|label| label.split_once(':').map(|(left, _)| left));
    if ns == Some("recovery") || operation.starts_with("recovery.") {
        return RuntimeHarnessState::Recover;
    }
    if matches!(selected_state_key, Some("case:owner-intake")) || operation == "owner.intake" {
        return RuntimeHarnessState::Intake;
    }
    if matches!(selected_state_key, Some("case:waiting-answer")) || operation == "owner.answer" {
        return RuntimeHarnessState::Clarify;
    }
    if record_state(ns) || record_operation(operation) {
        return RuntimeHarnessState::Record;
    }
    if maintain_state(ns) || maintain_operation(operation) {
        return RuntimeHarnessState::Maintain;
    }
    if operation.starts_with("check.run/") || operation.starts_with("completion.") {
        return RuntimeHarnessState::Observe;
    }
    if operation == "runtime.idle" || envelope == OutputEnvelope::None && recovery_policy == "none"
    {
        return RuntimeHarnessState::Idle;
    }
    match envelope {
        OutputEnvelope::Plan => RuntimeHarnessState::Plan,
        OutputEnvelope::Message => RuntimeHarnessState::Clarify,
        OutputEnvelope::Action | OutputEnvelope::Content | OutputEnvelope::Verdict => {
            RuntimeHarnessState::Act
        }
        OutputEnvelope::None => RuntimeHarnessState::Act,
    }
}

fn record_state(ns: Option<&str>) -> bool {
    matches!(
        ns,
        Some(
            "journal"
                | "todo"
                | "calendar"
                | "finance"
                | "note"
                | "contact"
                | "reference"
                | "routine"
                | "dev"
                | "project"
        )
    )
}

fn record_operation(operation: &str) -> bool {
    matches!(
        operation
            .split_once('/')
            .map_or(operation, |(head, _)| head),
        "journal.record"
            | "todo.review"
            | "calendar.review"
            | "finance.review"
            | "note.record"
            | "contact.record"
            | "reference.record"
            | "routine.run"
            | "dev.review"
            | "project.advance"
    )
}

fn maintain_state(ns: Option<&str>) -> bool {
    matches!(ns, Some("index" | "proof" | "maintenance" | "workspace"))
}

fn maintain_operation(operation: &str) -> bool {
    matches!(
        operation
            .split_once('/')
            .map_or(operation, |(head, _)| head),
        "index.rebuild" | "proof.collect" | "workspace.rebalance" | "workspace.maintain"
    )
}
