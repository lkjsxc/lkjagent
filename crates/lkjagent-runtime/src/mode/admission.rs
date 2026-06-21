use super::model::{ActiveMode, RuntimeSnapshot, ToolAdmission};
use super::policy::policy_for_mode;

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
    match tool {
        "artifact.next" => {
            "<act>\n<tool>artifact.next</tool>\n<root>dictionary</root>\n</act>".to_string()
        }
        "artifact.audit" => {
            "<act>\n<tool>artifact.audit</tool>\n<root>dictionary</root>\n</act>".to_string()
        }
        "doc.audit" => "<act>\n<tool>doc.audit</tool>\n<root>docs</root>\n</act>".to_string(),
        "fs.batch_write" => {
            "<act>\n<tool>fs.batch_write</tool>\n<files>\ndictionary/bread.md|# Bread\n</files>\n</act>"
                .to_string()
        }
        "fs.read" => "<act>\n<tool>fs.read</tool>\n<path>README.md</path>\n</act>".to_string(),
        "fs.list" => "<act>\n<tool>fs.list</tool>\n<path>.</path>\n</act>".to_string(),
        "fs.stat" => "<act>\n<tool>fs.stat</tool>\n<path>README.md</path>\n</act>".to_string(),
        "graph.evidence" => {
            "<act>\n<tool>graph.evidence</tool>\n<kind>verification</kind>\n<summary>Observed required evidence</summary>\n</act>".to_string()
        }
        "graph.recover" => "<act>\n<tool>graph.recover</tool>\n</act>".to_string(),
        "graph.transition" => {
            "<act>\n<tool>graph.transition</tool>\n<target>recover-by-bounded-write</target>\n<reason>Use admitted recovery path</reason>\n</act>".to_string()
        }
        "workspace.summary" => "<act>\n<tool>workspace.summary</tool>\n</act>".to_string(),
        "runtime.compact" => "runtime action; no model act block".to_string(),
        "runtime.wait" => "runtime action; no model act block".to_string(),
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
