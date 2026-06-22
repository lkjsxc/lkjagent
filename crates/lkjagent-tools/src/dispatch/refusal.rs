use super::examples::registry_valid_example;
use super::state::DispatchState;
use lkjagent_protocol::{render_action, Action, Param};

pub fn repeat_refusal(action_text: &str, state: &mut DispatchState) -> Option<String> {
    if state.last_action_text.as_deref() != Some(action_text) {
        state.repeat_count = 0;
        return None;
    }
    state.repeat_count = state.repeat_count.saturating_add(1);
    let prior = state
        .last_frame_ref
        .map_or_else(|| "previous frame".to_string(), |id| format!("frame {id}"));
    if action_text.contains("<tool>graph.next</tool>") {
        return Some(graph_next_repeat_refusal(&prior, state));
    }
    Some(repeat_route_refusal(action_text, &prior, state))
}

fn repeat_route_refusal(action_text: &str, prior: &str, state: &DispatchState) -> String {
    let repeated = repeated_tool(action_text).unwrap_or("unknown");
    if let Some(policy) = state.effective_policy.as_ref() {
        let preferred = policy
            .allowed_tools
            .iter()
            .find(|tool| tool.as_str() != repeated)
            .cloned()
            .unwrap_or_else(|| "none".to_string());
        let example = registry_valid_example(&preferred).unwrap_or_else(|| "none".to_string());
        return format!(
            "repeat action refused; see {prior}\nactive_mode={}\nnext_action_must_change_shape=true\nforbidden_tool={repeated}\npreferred_next_action={preferred}\nvalid_example:\n{example}",
            policy.mode
        );
    }
    if let Some(policy) = state.graph_policy.as_ref() {
        let preferred = preferred_action(policy, Some(repeated));
        return format!(
            "repeat action refused; see {prior}\nnode={}\nphase={}\nnext_action_must_change_shape=true\nforbidden_tool={repeated}\npreferred_next_action={preferred}\nvalid_example:\n{}",
            policy.active_node,
            policy.phase,
            example_for(policy, Some(repeated))
        );
    }
    format!("repeat action refused; see {prior}\nnext_action_must_change_shape=true")
}

fn repeated_tool(action_text: &str) -> Option<&str> {
    action_text
        .split("<tool>")
        .nth(1)
        .and_then(|tail| tail.split("</tool>").next())
}

pub fn policy_refusal(tool: &str, state: &DispatchState) -> Option<String> {
    if let Some(policy) = state.effective_policy.as_ref() {
        return super::effective_refusal::effective_policy_refusal(tool, policy, state);
    }
    graph_policy_refusal(tool, state)
}

fn graph_policy_refusal(tool: &str, state: &DispatchState) -> Option<String> {
    if tool == "agent.done" {
        return None;
    }
    let policy = state.graph_policy.as_ref()?;
    if policy.allowed_tools.iter().any(|allowed| allowed == tool) {
        if tool == "shell.run" && !policy.shell_allowed {
            return Some(format!(
                "graph policy refused shell.run\nnode={}\nphase={}\nreason=shell is not admitted by this node\nallowed_tools={}\npreferred_next_action={}\nvalid_example:\n{}",
                policy.active_node,
                policy.phase,
                join_or_none(&policy.allowed_tools),
                preferred_action(policy, Some(tool)),
                example_for(policy, Some(tool))
            ));
        }
        return None;
    }
    Some(format!(
        "graph policy refused {tool}\nnode={}\nphase={}\nreason={}\nallowed_tools={}\npreferred_next_action={}\nvalid_example:\n{}",
        policy.active_node,
        policy.phase,
        policy
            .blocked_reason
            .as_deref()
            .unwrap_or("tool is not admitted by the active graph node"),
        join_or_none(&policy.allowed_tools),
        preferred_action(policy, Some(tool)),
        example_for(policy, Some(tool))
    ))
}

fn graph_next_repeat_refusal(prior: &str, state: &DispatchState) -> String {
    let Some(policy) = state.graph_policy.as_ref() else {
        return format!("repeat action refused; graph.next already inspected; see {prior}");
    };
    if !policy.active_node.starts_with("recover") {
        return format!("repeat action refused; graph.next already inspected; see {prior}");
    }
    format!(
        "repeat action refused; graph.next already inspected for this fault; see {prior}\nnode={}\nphase={}\nnext_action_must_be=graph.recover, graph.transition to {}, unused non-mutating inspection, smaller graph.plan when admitted, or agent.ask only with owner_required question\npreferred_next_action={}\nvalid_example:\n{}",
        policy.active_node,
        policy.phase,
        first_or(&policy.legal_transitions, "a legal target"),
        preferred_action(policy, Some("graph.next")),
        example_for(policy, Some("graph.next"))
    )
}

pub(super) fn preferred_action(
    policy: &super::state::GraphDispatchPolicy,
    blocked: Option<&str>,
) -> String {
    let priority = [
        "graph.recover",
        "graph.transition",
        "artifact.next",
        "artifact.plan",
        "artifact.apply",
        "artifact.audit",
        "doc.scaffold",
        "doc.audit",
        "fs.batch_write",
        "graph.plan",
        "fs.list",
        "fs.tree",
        "fs.search",
        "verify.xtask",
        "graph.state",
        "graph.next",
    ];
    if let Some(tool) = priority.iter().find(|tool| allowed(policy, blocked, tool)) {
        return (*tool).to_string();
    }
    policy
        .allowed_tools
        .iter()
        .find(|tool| blocked != Some(tool.as_str()) && tool.as_str() != "graph.next")
        .cloned()
        .or_else(|| {
            policy
                .allowed_tools
                .iter()
                .find(|tool| blocked != Some(tool.as_str()))
                .cloned()
        })
        .unwrap_or_else(|| "none".to_string())
}

fn allowed(policy: &super::state::GraphDispatchPolicy, blocked: Option<&str>, tool: &str) -> bool {
    if tool == "graph.transition" && policy.legal_transitions.is_empty() {
        return false;
    }
    blocked != Some(tool) && policy.allowed_tools.iter().any(|allowed| allowed == tool)
}

pub(super) fn example_for(
    policy: &super::state::GraphDispatchPolicy,
    blocked: Option<&str>,
) -> String {
    let preferred = preferred_action(policy, blocked);
    if preferred == "graph.transition" {
        return transition_example(policy);
    }
    registry_valid_example(&preferred).unwrap_or_else(|| "none".to_string())
}

fn transition_example(policy: &super::state::GraphDispatchPolicy) -> String {
    let Some(target) = policy.legal_transitions.first() else {
        return "No legal transition is currently admitted.".to_string();
    };
    render_action(&Action::new(
        "graph.transition",
        vec![
            Param::new("target", target),
            Param::new("reason", "Use an admitted graph transition"),
        ],
    ))
}

pub(super) fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn first_or(values: &[String], fallback: &str) -> String {
    values
        .iter()
        .find(|value| !value.is_empty())
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}
