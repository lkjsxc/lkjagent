use std::collections::BTreeMap;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolFieldSpec, ToolValueClass};
use crate::runtime_fingerprint::FingerprintError;
use crate::runtime_tool_view::EffectKey;

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
    let fingerprint = decision.tool_view_fingerprint()?;
    let rejection = rejection_reason(decision, action);
    let admitted = rejection.is_none();
    Ok(ToolAdmission {
        decision_id: decision.id.clone(),
        tool_view_fingerprint: fingerprint,
        action_tool: action.tool.clone(),
        status: if admitted {
            AdmissionStatus::Admitted
        } else {
            AdmissionStatus::Rejected
        },
        reason: rejection.unwrap_or_else(|| "admitted".into()),
    })
}

pub fn admitted_effect_key(
    decision: &RuntimeDecision,
    admission: &ToolAdmission,
) -> Result<EffectKey, &'static str> {
    if admission.status != AdmissionStatus::Admitted || admission.decision_id != decision.id {
        return Err("admission-not-current");
    }
    let fingerprint = decision
        .tool_view_fingerprint()
        .map_err(|_| "invalid-tool-view")?;
    if admission.tool_view_fingerprint != fingerprint {
        return Err("stale-tool-view");
    }
    let entry = decision
        .tool_view
        .entry(&admission.action_tool)
        .ok_or("hidden-tool")?;
    Ok(entry.effect_key.clone())
}

pub fn dispatch_effect_key(
    decision: &RuntimeDecision,
    admission: &ToolAdmission,
    persisted_key: &EffectKey,
) -> Result<EffectKey, &'static str> {
    let projected = admitted_effect_key(decision, admission)?;
    if &projected != persisted_key {
        return Err("stale-effect-key");
    }
    Ok(projected)
}

fn rejection_reason(decision: &RuntimeDecision, action: &ModelAction) -> Option<String> {
    if decision.expected_envelope != OutputEnvelope::Action {
        return Some("decision does not admit actions".into());
    }
    let Some(entry) = decision.tool_view.entry(&action.tool) else {
        return Some("hidden-tool".into());
    };
    if entry.effect_key.0.is_empty() || entry.result_max_bytes == 0 || entry.denial_code.is_empty()
    {
        return Some("incomplete persisted tool projection".into());
    }
    if entry.effect_key.0 == "workspace.record"
        && !matches!(
            action.params.get("family").map(String::as_str),
            Some("journal" | "memory")
        )
    {
        return Some("record family is not admitted".into());
    }
    for spec in entry.field_specs.iter().filter(|spec| spec.required) {
        if !action.params.contains_key(&spec.name) {
            return Some(format!("missing required parameter {}", spec.name));
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
    let value = value.trim();
    matches!(
        value.to_ascii_uppercase().as_str(),
        "..." | "PATH" | "TOOL" | "TODO" | "VALUE" | "FIELD_VALUE" | "REPLACE_ME"
    ) || [('<', '>'), ('[', ']'), ('{', '}')]
        .iter()
        .any(|(open, close)| value.starts_with(*open) && value.ends_with(*close) && value.len() > 2)
}

fn value_class_rejection(name: &str, value: &str, spec: &ToolFieldSpec) -> Option<String> {
    if value.trim().is_empty() && spec.min_bytes > 0 {
        return Some(format!("empty value for {name}"));
    }
    if !spec.accepts_size(value) {
        return Some(format!("value size out of bounds for {name}"));
    }
    match spec.value_class {
        ToolValueClass::WorkspacePath if !workspace_relative_path(value) => {
            Some("path escapes workspace".into())
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
    !path.is_absolute()
        && (path == Path::new(".")
            || path
                .components()
                .all(|component| matches!(component, Component::Normal(_))))
}
