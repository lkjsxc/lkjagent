use std::collections::{BTreeMap, BTreeSet};

use lkjagent_protocol::registry::{find_tool, TOOLS};
use lkjagent_protocol::Action;
use lkjagent_protocol::{render_action, Param};

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
        return Err(validation_message(
            &action.tool,
            spec.params,
            &duplicate,
            &missing,
            &unknown,
        ));
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

fn validation_message(
    tool: &str,
    specs: &[lkjagent_protocol::registry::ParamSpec],
    duplicate: &[&str],
    missing: &[&str],
    unknown: &[&str],
) -> String {
    let mut lines = vec![
        "action params refused".to_string(),
        format!("tool={tool}"),
        format!("expected={}", expected(specs)),
        format!("received={}", received(duplicate, unknown)),
    ];
    if !duplicate.is_empty() {
        lines.push(format!("duplicate={}", duplicate.join(",")));
    }
    if !missing.is_empty() {
        lines.push(format!("missing={}", missing.join(",")));
    }
    if !unknown.is_empty() {
        lines.push(format!("unknown={}", unknown.join(",")));
    }
    lines.push(format!("hint={}", hint(tool, specs, unknown)));
    lines.push("valid_example:".to_string());
    lines.push(valid_example(tool, specs));
    lines.join("\n")
}

fn expected(specs: &[lkjagent_protocol::registry::ParamSpec]) -> String {
    if specs.is_empty() {
        return "no parameters".to_string();
    }
    specs
        .iter()
        .map(|spec| {
            if spec.required {
                format!("{} required", spec.name)
            } else if let Some(default) = spec.default {
                format!("{} optional default={default}", spec.name)
            } else {
                format!("{} optional", spec.name)
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn received(duplicate: &[&str], unknown: &[&str]) -> String {
    let mut names = duplicate
        .iter()
        .chain(unknown.iter())
        .copied()
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(",")
    }
}

fn hint(tool: &str, specs: &[lkjagent_protocol::registry::ParamSpec], unknown: &[&str]) -> String {
    if specs.is_empty() && unknown.contains(&"path") {
        return format!(
            "{tool} never takes path; use fs.list or workspace.summary for path inspection"
        );
    }
    if matches!(tool, "doc.scaffold" | "doc.audit") && unknown.contains(&"path") {
        return format!("{tool} uses root, not path");
    }
    "emit the valid_example exactly, or choose a tool whose schema names the received parameter"
        .to_string()
}

fn valid_example(tool: &str, specs: &[lkjagent_protocol::registry::ParamSpec]) -> String {
    let params = specs
        .iter()
        .filter(|spec| spec.required)
        .map(|spec| Param::new(spec.name, example_value(tool, spec.name)))
        .collect::<Vec<_>>();
    render_action(&Action::new(tool, params))
}

fn example_value(tool: &str, name: &str) -> &'static str {
    match (tool, name) {
        ("doc.scaffold", "root") | ("doc.audit", "root") => "docs",
        ("doc.scaffold", "title") => "Project Documentation",
        ("fs.read", "path") | ("fs.stat", "path") | ("fs.mkdir", "path") => "README.md",
        ("graph.evidence", "kind") | ("graph.note", "kind") => "observation",
        ("graph.evidence", "summary") | ("graph.note", "summary") => "Read README.md",
        ("agent.done", "summary") => "Completed with evidence",
        ("agent.ask", "question") => "What specific target should I use?",
        ("queue.enqueue", "content") | ("queue.edit", "content") => "Owner task",
        ("queue.enqueue", "reason") | ("queue.edit", "reason") => "owner request",
        ("shell.run", "command") => "pwd",
        ("verify.cargo", "gate") => "test",
        ("verify.xtask", "gate") => "check-docs",
        _ => "value",
    }
}
