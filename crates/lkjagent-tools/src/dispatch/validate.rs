use std::collections::{BTreeMap, BTreeSet};

use lkjagent_protocol::registry::{find_tool, TOOLS};
use lkjagent_protocol::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedAction {
    pub tool: String,
    pub params: BTreeMap<String, String>,
}

pub fn validate_action(action: &Action) -> Result<ValidatedAction, String> {
    let Some(spec) = find_tool(&action.tool) else {
        return Err(format!(
            "unknown tool: {}; valid tools: {}",
            action.tool,
            TOOLS
                .iter()
                .map(|tool| tool.name)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    };
    let duplicate = duplicate_params(action);
    let missing = spec
        .params
        .iter()
        .filter(|param| param.required)
        .filter(|param| !action.params.iter().any(|given| given.name == param.name))
        .map(|param| param.name)
        .collect::<Vec<_>>();
    let unknown = action
        .params
        .iter()
        .filter(|given| !spec.params.iter().any(|param| param.name == given.name))
        .map(|param| param.name.as_str())
        .collect::<Vec<_>>();
    if !duplicate.is_empty() || !missing.is_empty() || !unknown.is_empty() {
        return Err(validation_message(&duplicate, &missing, &unknown));
    }
    Ok(ValidatedAction {
        tool: action.tool.clone(),
        params: defaulted_params(action, spec.params),
    })
}

fn defaulted_params(
    action: &Action,
    specs: &[lkjagent_protocol::registry::ParamSpec],
) -> BTreeMap<String, String> {
    let mut params = BTreeMap::new();
    for spec in specs {
        if let Some(given) = action.params.iter().find(|given| given.name == spec.name) {
            params.insert(spec.name.to_string(), given.value.clone());
        } else if let Some(default) = spec.default {
            params.insert(spec.name.to_string(), default.to_string());
        }
    }
    params
}

fn duplicate_params(action: &Action) -> Vec<&str> {
    let mut seen = BTreeSet::new();
    let mut duplicate = Vec::new();
    for param in &action.params {
        if !seen.insert(param.name.as_str()) {
            duplicate.push(param.name.as_str());
        }
    }
    duplicate
}

fn validation_message(duplicate: &[&str], missing: &[&str], unknown: &[&str]) -> String {
    let mut parts = Vec::new();
    if !duplicate.is_empty() {
        parts.push(format!("duplicate params: {}", duplicate.join(", ")));
    }
    if !missing.is_empty() {
        parts.push(format!("missing params: {}", missing.join(", ")));
    }
    if !unknown.is_empty() {
        parts.push(format!("unknown params: {}", unknown.join(", ")));
    }
    parts.join("; ")
}
