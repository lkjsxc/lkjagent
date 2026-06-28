use std::collections::{BTreeMap, BTreeSet};

use super::examples::valid_example;
use lkjagent_protocol::registry::{
    find_tool, missing_required, missing_required_any, unknown_params, ParamSpec, ToolSpec, TOOLS,
};
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
    let names = param_names(action);
    let missing = missing_required(spec, &names);
    let missing_any = missing_required_any(spec, &names);
    let unknown = unknown_params(spec, &names);
    if !duplicate.is_empty()
        || !missing.is_empty()
        || !missing_any.is_empty()
        || !unknown.is_empty()
    {
        return Err(validation_message(
            spec,
            &duplicate,
            &missing,
            &missing_any,
            &unknown,
        ));
    }
    Ok(ValidatedAction {
        tool: action.tool.clone(),
        params: defaulted_params(action, spec.params),
    })
}

fn defaulted_params(action: &Action, specs: &[ParamSpec]) -> BTreeMap<String, String> {
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

fn param_names(action: &Action) -> Vec<&str> {
    action
        .params
        .iter()
        .map(|param| param.name.as_str())
        .collect()
}

fn validation_message(
    spec: &ToolSpec,
    duplicate: &[&str],
    missing: &[String],
    missing_any: &[String],
    unknown: &[String],
) -> String {
    let mut lines = vec![
        "action params refused".to_string(),
        format!("tool={}", spec.name),
        format!("expected={}", expected(spec)),
        format!("received={}", received(duplicate, unknown)),
    ];
    if !duplicate.is_empty() {
        lines.push(format!("duplicate={}", duplicate.join(",")));
    }
    if !missing.is_empty() {
        lines.push(format!("missing={}", missing.join(",")));
    }
    if !missing_any.is_empty() {
        lines.push(format!("missing_any={}", missing_any.join(",")));
    }
    if !unknown.is_empty() {
        lines.push(format!("unknown={}", unknown.join(",")));
    }
    lines.push(format!("hint={}", hint(spec, unknown)));
    lines.push("valid_example:".to_string());
    lines.push(valid_example(spec.name, spec.params));
    lines.join("\n")
}

fn expected(spec: &ToolSpec) -> String {
    if spec.params.is_empty() && spec.required_any.is_empty() {
        return "no parameters".to_string();
    }
    let mut rendered = spec
        .params
        .iter()
        .map(|param| {
            if param.required {
                format!("{} required", param.name)
            } else if let Some(default) = param.default {
                format!("{} optional default={default}", param.name)
            } else {
                format!("{} optional", param.name)
            }
        })
        .collect::<Vec<_>>();
    rendered.extend(
        spec.required_any
            .iter()
            .map(|group| format!("{} any required", group.label)),
    );
    rendered.join("; ")
}

fn received(duplicate: &[&str], unknown: &[String]) -> String {
    let mut names = duplicate
        .iter()
        .map(|name| (*name).to_string())
        .collect::<Vec<_>>();
    names.extend(unknown.iter().cloned());
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(",")
    }
}

fn hint(spec: &ToolSpec, unknown: &[String]) -> String {
    if spec.params.is_empty() && unknown.iter().any(|name| name == "path") {
        return format!(
            "{} never takes path; use fs.list or workspace.summary for path inspection",
            spec.name
        );
    }
    if spec.name == "doc.audit" && unknown.iter().any(|name| name == "path") {
        return format!("{} uses root, not path", spec.name);
    }
    "emit the valid_example exactly, or choose a tool whose schema names the received parameter"
        .to_string()
}
