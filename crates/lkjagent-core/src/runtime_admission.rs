use std::collections::BTreeMap;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolFieldSpec, ToolValueClass};
use crate::runtime_fingerprint::FingerprintError;
use crate::runtime_tool_catalog::effect_for_tool;

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
    if effect_for_tool(&action.tool).is_none() {
        return Some(format!("tool catalog excludes {}", action.tool));
    }
    let entry = match decision.tool_view.entry(&action.tool) {
        Some(entry) => entry,
        None => {
            return Some(format!(
                "tool-view mismatch: {} absent from decision view",
                action.tool
            ))
        }
    };
    for required in &entry.required_params {
        if !action.params.contains_key(required) {
            return Some(format!("missing required parameter {required}"));
        }
    }
    for (name, value) in &action.params {
        let Some(spec) = entry.field_spec(name) else {
            return Some(format!("unknown parameter {name}"));
        };
        if placeholder_value(value) {
            return Some(format!("placeholder value for {name}"));
        }
        if let Some(reason) = value_class_rejection(name, value, spec) {
            return Some(reason);
        }
    }
    None
}

fn placeholder_value(value: &str) -> bool {
    let trimmed = value.trim();
    let upper = trimmed.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "..." | "PATH" | "TOOL" | "TODO" | "VALUE" | "FIELD_VALUE" | "REPLACE_ME"
    ) || wrapped_placeholder(trimmed, '<', '>')
        || wrapped_placeholder(trimmed, '[', ']')
        || wrapped_placeholder(trimmed, '{', '}')
}

fn wrapped_placeholder(value: &str, open: char, close: char) -> bool {
    value.starts_with(open) && value.ends_with(close) && value.len() > 2
}

fn value_class_rejection(name: &str, value: &str, spec: &ToolFieldSpec) -> Option<String> {
    if value.trim().is_empty() {
        return Some(format!("empty value for {name}"));
    }
    if !spec.accepts_size(value) {
        return Some(format!("value size out of bounds for {name}"));
    }
    match spec.value_class {
        ToolValueClass::WorkspacePath if !workspace_relative_path(value) => {
            Some("path escapes workspace".to_string())
        }
        ToolValueClass::Count if spec.canonical_count(value).is_none() => {
            Some(format!("invalid count for {name}"))
        }
        _ => None,
    }
}

pub fn workspace_relative_path(path: &str) -> bool {
    if path.is_empty() || path.len() > 1024 || path.chars().any(char::is_control) {
        return false;
    }
    let path = Path::new(path);
    if path.is_absolute() {
        return false;
    }
    path == Path::new(".")
        || path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
