use std::collections::BTreeMap;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::runtime_decision::{OutputEnvelope, RuntimeDecision};
use crate::runtime_fingerprint::FingerprintError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelAction {
    pub tool: String,
    pub params: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdmissionStatus {
    Admitted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolAdmission {
    pub decision_id: String,
    pub tool_view_fingerprint: String,
    pub action_tool: String,
    pub status: AdmissionStatus,
    pub reason: String,
}

pub fn admit_action(
    decision: &RuntimeDecision,
    action: &ModelAction,
) -> Result<ToolAdmission, FingerprintError> {
    let view_fingerprint = decision.tool_view_fingerprint()?;
    let rejection = rejection_reason(decision, action);
    let (status, reason) = match rejection {
        Some(reason) => (AdmissionStatus::Rejected, reason),
        None => (AdmissionStatus::Admitted, "admitted".to_string()),
    };
    Ok(ToolAdmission {
        decision_id: decision.id.clone(),
        tool_view_fingerprint: view_fingerprint,
        action_tool: action.tool.clone(),
        status,
        reason,
    })
}

fn rejection_reason(decision: &RuntimeDecision, action: &ModelAction) -> Option<String> {
    if decision.expected_envelope != OutputEnvelope::Action {
        return Some("decision does not admit actions".to_string());
    }
    let entry = match decision.tool_view.entry(&action.tool) {
        Some(entry) => entry,
        None => return Some("tool absent from decision view".to_string()),
    };
    for required in &entry.required_params {
        if !action.params.contains_key(required) {
            return Some(format!("missing required parameter {required}"));
        }
    }
    for name in action.params.keys() {
        if !entry.accepts_param(name) {
            return Some(format!("unknown parameter {name}"));
        }
    }
    if let Some(path) = action.params.get("path") {
        if !workspace_relative_path(path) {
            return Some("path escapes workspace".to_string());
        }
    }
    None
}

pub fn workspace_relative_path(path: &str) -> bool {
    let path = Path::new(path);
    if path.is_absolute() {
        return false;
    }
    path.components()
        .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}
