use super::completion_gate::decide_completion;
use super::model::{ActiveMode, RuntimeSnapshot, ToolAdmission};
use super::policy::policy_for_mode;
use lkjagent_tools::dispatch::registry_valid_example;

pub fn admit_tool(snapshot: &RuntimeSnapshot, requested_tool: &str) -> ToolAdmission {
    let next_valid_tools = next_valid_tools(snapshot);
    let completion = decide_completion(snapshot);
    let completion_blocked = requested_tool == "agent.done" && !completion.allowed;
    let owner_question_blocked =
        requested_tool == "agent.ask" && !snapshot.external_owner_input_required;
    let repeated_blocked = snapshot.repeated_action
        && snapshot
            .last_tool_attempt
            .as_deref()
            .is_some_and(|tool| tool == requested_tool);
    let example_tool = example_tool(requested_tool, &next_valid_tools, repeated_blocked);
    let exact_valid_example = example_tool.map(valid_example);
    let admitted = !completion_blocked
        && !owner_question_blocked
        && !repeated_blocked
        && next_valid_tools.iter().any(|tool| tool == requested_tool);
    let reason = reason(
        snapshot,
        requested_tool,
        admitted,
        completion_blocked,
        owner_question_blocked,
        repeated_blocked,
    );
    ToolAdmission {
        admitted,
        reason,
        active_mode: snapshot.active_mode,
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
    let policy = policy_for_mode(snapshot.active_mode);
    let mut tools = policy
        .allowed_tools
        .iter()
        .map(|tool| (*tool).to_string())
        .collect::<Vec<_>>();
    if snapshot.active_mode == ActiveMode::OwnerTask {
        if snapshot.missing_evidence.is_empty() {
            extend_unique(
                &mut tools,
                &["agent.done", "verify.xtask", "fs.read", "workspace.summary"],
            );
        } else {
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
    }
    if snapshot.external_owner_input_required {
        extend_unique(&mut tools, &["agent.ask"]);
    }
    if tools.is_empty() && snapshot.active_mode == ActiveMode::ClosedIdle {
        tools.push("runtime.wait".to_string());
    }
    tools
}

fn reason(
    snapshot: &RuntimeSnapshot,
    requested_tool: &str,
    admitted: bool,
    completion_blocked: bool,
    owner_question_blocked: bool,
    repeated_blocked: bool,
) -> String {
    if admitted {
        return format!("{requested_tool} admitted by runtime authority");
    }
    if completion_blocked {
        return "completion missing required evidence".to_string();
    }
    if owner_question_blocked {
        return "owner question requires concrete external missing input".to_string();
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
    if snapshot.active_mode == ActiveMode::Recovery && next_valid_tools(snapshot).is_empty() {
        return Some("recovery has no escape tools".to_string());
    }
    None
}

fn example_tool<'a>(
    requested_tool: &'a str,
    next_valid_tools: &'a [String],
    repeated_blocked: bool,
) -> Option<&'a str> {
    if repeated_blocked {
        return next_valid_tools
            .iter()
            .find(|tool| tool.as_str() != requested_tool)
            .map(String::as_str);
    }
    if next_valid_tools.iter().any(|tool| tool == requested_tool) {
        return Some(requested_tool);
    }
    next_valid_tools.first().map(String::as_str)
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
