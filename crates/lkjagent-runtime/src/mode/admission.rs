use super::model::{ActiveMode, RuntimeSnapshot, ToolAdmission};
use super::policy::policy_for_mode;
use lkjagent_tools::dispatch::registry_valid_example;

pub fn admit_tool(snapshot: &RuntimeSnapshot, requested_tool: &str) -> ToolAdmission {
    let next_valid_tools = next_valid_tools(snapshot);
    let exact_valid_example = next_valid_tools.first().map(|tool| valid_example(tool));
    let completion_blocked =
        requested_tool == "agent.done" && !snapshot.missing_evidence.is_empty();
    let repeated_blocked = snapshot.repeated_action
        && snapshot
            .last_tool_attempt
            .as_deref()
            .is_some_and(|tool| tool == requested_tool);
    let admitted = !completion_blocked
        && !repeated_blocked
        && next_valid_tools.iter().any(|tool| tool == requested_tool);
    let reason = reason(
        snapshot,
        requested_tool,
        admitted,
        completion_blocked,
        repeated_blocked,
    );
    ToolAdmission {
        admitted,
        reason,
        active_mission: snapshot.active_mission,
        required_evidence: snapshot.required_evidence.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        next_valid_tools,
        exact_valid_example,
        contradiction: contradiction(snapshot, requested_tool, admitted),
    }
}

pub fn next_valid_tools(snapshot: &RuntimeSnapshot) -> Vec<String> {
    if snapshot.context_pressure_active {
        return vec!["runtime.compact".to_string()];
    }
    let policy = policy_for_mode(snapshot.active_mission);
    let mut tools = policy
        .allowed_tools
        .iter()
        .map(|tool| (*tool).to_string())
        .collect::<Vec<_>>();
    if snapshot.active_mission == ActiveMode::OwnerTask && !snapshot.missing_evidence.is_empty() {
        extend_unique(
            &mut tools,
            &[
                "fs.read",
                "fs.list",
                "fs.stat",
                "artifact.audit",
                "artifact.next",
                "doc.audit",
                "fs.batch_write",
                "graph.evidence",
                "workspace.summary",
            ],
        );
    }
    if tools.is_empty() && snapshot.active_mission == ActiveMode::ClosedIdle {
        tools.push("runtime.wait".to_string());
    }
    tools
}

fn reason(
    snapshot: &RuntimeSnapshot,
    requested_tool: &str,
    admitted: bool,
    completion_blocked: bool,
    repeated_blocked: bool,
) -> String {
    if admitted {
        return format!("{requested_tool} admitted by runtime authority");
    }
    if completion_blocked {
        return "completion missing required evidence".to_string();
    }
    if repeated_blocked {
        return "repeat action suppressed by runtime authority".to_string();
    }
    if snapshot.context_pressure_active {
        return "runtime compaction must run before model tool execution".to_string();
    }
    format!("{requested_tool} is not admitted by active mission")
}

fn contradiction(
    snapshot: &RuntimeSnapshot,
    requested_tool: &str,
    admitted: bool,
) -> Option<String> {
    if admitted || requested_tool == "agent.done" {
        return None;
    }
    if snapshot.active_mission == ActiveMode::Recovery && next_valid_tools(snapshot).is_empty() {
        return Some("recovery has no escape tools".to_string());
    }
    None
}

fn valid_example(tool: &str) -> String {
    registry_valid_example(tool).unwrap_or_else(|| runtime_only_example(tool))
}

fn runtime_only_example(tool: &str) -> String {
    match tool {
        "runtime.compact" | "runtime.wait" => "runtime action; no model act block".to_string(),
        other => format!("<act>\n<tool>{other}</tool>\n</act>"),
    }
}

fn extend_unique(target: &mut Vec<String>, tools: &[&str]) {
    for tool in tools {
        if !target.iter().any(|existing| existing == tool) {
            target.push((*tool).to_string());
        }
    }
}
