use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};
pub use crate::runtime_tool_view::{
    ToolExampleParam, ToolFieldSpec, ToolSetView, ToolValueClass, ToolViewEntry,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperationKey(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputEnvelope {
    Content,
    Plan,
    Action,
    Message,
    Verdict,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDecision {
    pub id: String,
    pub case_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_state_key: Option<String>,
    pub operation: OperationKey,
    pub snapshot_fingerprint: String,
    pub state_vector_fingerprint: String,
    pub context_frame_fingerprint: String,
    pub tool_view: ToolSetView,
    pub expected_envelope: OutputEnvelope,
    pub model_budget_tokens: Option<u32>,
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
        Self {
            id: id.into(),
            case_id: case_id.into(),
            selected_state_key: None,
            operation,
            snapshot_fingerprint: String::new(),
            state_vector_fingerprint: String::new(),
            context_frame_fingerprint: String::new(),
            tool_view,
            expected_envelope,
            model_budget_tokens: None,
            evidence_requirements: Vec::new(),
            recovery_policy: "default".to_string(),
        }
    }

    pub fn tool_view_fingerprint(&self) -> Result<String, FingerprintError> {
        self.tool_view.fingerprint()
    }

    pub fn fingerprint(&self) -> Result<String, FingerprintError> {
        stable_fingerprint(self)
    }
}
