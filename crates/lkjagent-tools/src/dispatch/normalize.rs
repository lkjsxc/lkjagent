use super::batch_write_normalize::normalize_batch_write_paths;
use lkjagent_protocol::registry::find_tool;
use lkjagent_protocol::{Action, Param};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizationDecision {
    Unchanged(Action),
    Normalized {
        action: Action,
        notes: Vec<NormalizationNote>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizationNote {
    pub tool: String,
    pub message: String,
}

pub fn normalize_action(action: &Action) -> NormalizationDecision {
    let mut next = action.clone();
    let mut notes = Vec::new();
    apply_aliases(&mut next, &mut notes);
    normalize_batch_write_paths(&mut next, &mut notes);
    drop_safe_no_param_locations(&mut next, &mut notes);
    if notes.is_empty() {
        NormalizationDecision::Unchanged(next)
    } else {
        NormalizationDecision::Normalized {
            action: next,
            notes,
        }
    }
}

impl NormalizationNote {
    pub fn render(&self) -> String {
        format!("tool={}\n{}", self.tool, self.message)
    }
}

fn apply_aliases(action: &mut Action, notes: &mut Vec<NormalizationNote>) {
    for (tool, from, to) in [
        ("doc.audit", "path", "root"),
        ("workspace.summary", "root", "path"),
        ("fs.list", "root", "path"),
    ] {
        if action.tool != tool || has_param(action, to) || !has_param(action, from) {
            continue;
        }
        for param in &mut action.params {
            if param.name == from {
                param.name = to.to_string();
            }
        }
        notes.push(NormalizationNote {
            tool: action.tool.clone(),
            message: format!(
                "action params normalized\nrenamed={from}->{to}\nreason={tool} uses {to}, not {from}"
            ),
        });
    }
}

fn drop_safe_no_param_locations(action: &mut Action, notes: &mut Vec<NormalizationNote>) {
    let Some(spec) = find_tool(&action.tool) else {
        return;
    };
    if !spec.params.is_empty() || action.params.is_empty() {
        return;
    }
    if !action.params.iter().all(ignorable_location) {
        return;
    }
    let dropped = action
        .params
        .iter()
        .map(|param| param.name.as_str())
        .collect::<Vec<_>>()
        .join(",");
    action.params.clear();
    notes.push(NormalizationNote {
        tool: action.tool.clone(),
        message: format!(
            "action params normalized\ndropped={dropped}\nreason={} accepts no parameters",
            action.tool
        ),
    });
}

fn has_param(action: &Action, name: &str) -> bool {
    action.params.iter().any(|param| param.name == name)
}

fn ignorable_location(param: &Param) -> bool {
    matches!(param.name.as_str(), "path" | "root" | "target")
        && matches!(
            param.value.trim(),
            "" | "." | "./" | "workspace" | "/workspace"
        )
}
