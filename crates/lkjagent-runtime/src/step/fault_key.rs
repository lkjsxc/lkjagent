use std::collections::BTreeSet;

use super::fault_wait::RecoveryFault;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FaultRetryKey {
    pub tool: String,
    pub parameter_shape: String,
    pub fault_class: String,
}

pub(super) fn retry_key(fault: RecoveryFault, action: Option<&str>) -> FaultRetryKey {
    FaultRetryKey {
        tool: tool_name(action),
        parameter_shape: parameter_shape(action),
        fault_class: fault_class(fault).to_string(),
    }
}

fn fault_class(fault: RecoveryFault) -> &'static str {
    match fault {
        RecoveryFault::Parse => "parse",
        RecoveryFault::Payload => "payload",
        RecoveryFault::Params => "parameter",
        RecoveryFault::Repeat => "repeat",
        RecoveryFault::Tool => "tool",
    }
}

fn tool_name(action: Option<&str>) -> String {
    action
        .and_then(|text| text.split("<tool>").nth(1))
        .and_then(|tail| tail.split("</tool>").next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("none")
        .to_string()
}

fn parameter_shape(action: Option<&str>) -> String {
    let Some(action) = action else {
        return "none".to_string();
    };
    let tags = action
        .split('<')
        .filter_map(|tail| tail.split('>').next())
        .filter_map(tag_name)
        .collect::<BTreeSet<_>>();
    if tags.is_empty() {
        "none".to_string()
    } else {
        tags.into_iter().collect::<Vec<_>>().join(",")
    }
}

fn tag_name(raw: &str) -> Option<String> {
    let name = raw.trim().trim_start_matches('/').trim();
    if name.is_empty()
        || raw.trim().starts_with('/')
        || matches!(name, "act" | "tool" | "content" | "file")
    {
        None
    } else {
        Some(name.to_string())
    }
}
