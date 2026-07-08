use serde::{Deserialize, Serialize};

use crate::runtime_decision::{EffectCommand, OutputEnvelope, ToolSetView};

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
