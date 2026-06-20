use super::examples::registry_valid_example;
use super::refusal::{example_for, join_or_none, preferred_action};
use super::state::{DispatchState, EffectivePolicy};

pub fn effective_policy_refusal(
    tool: &str,
    policy: &EffectivePolicy,
    state: &DispatchState,
) -> Option<String> {
    if tool == "agent.done" {
        return completion_refusal(policy, state);
    }
    if policy.allowed_tools.iter().any(|allowed| allowed == tool) {
        if tool == "shell.run" && !policy.shell_allowed {
            return Some(render(
                tool,
                "shell is not admitted by this active mode",
                policy,
                state,
            ));
        }
        return None;
    }
    Some(render(tool, policy.reason.as_str(), policy, state))
}

fn completion_refusal(policy: &EffectivePolicy, state: &DispatchState) -> Option<String> {
    if policy.completion_allowed {
        return None;
    }
    let missing = if state.graph_missing.is_empty() {
        policy.reason.clone()
    } else {
        state.graph_missing.join(", ")
    };
    let graph_line = state
        .graph_state
        .as_deref()
        .and_then(|text| text.lines().find(|line| !line.trim().is_empty()))
        .unwrap_or("graph_state=unavailable");
    Some(format!(
        "completion refused\npartial_handoff=blocked-with-evidence\nattempted=agent.done\nactive_mode={}\nfailed_gate={}\nmissing={missing}\nexisting_graph={graph_line}\nnext_executable_action={}\nvalid_example:\n{}",
        policy.mode,
        completion_gate(policy),
        effective_preferred_action(policy, state, Some("agent.done")),
        effective_example(policy, state, Some("agent.done"))
    ))
}

fn completion_gate(policy: &EffectivePolicy) -> &'static str {
    match policy.mode.as_str() {
        "Compaction" => "runtime-compaction",
        "ClosedIdle" => "closed-idle",
        "Maintenance" => "maintenance-completion",
        "Recovery" => "recovery-completion",
        _ => "completion",
    }
}

fn render(tool: &str, reason: &str, policy: &EffectivePolicy, state: &DispatchState) -> String {
    let node = state
        .graph_policy
        .as_ref()
        .map_or("none", |graph| graph.active_node.as_str());
    let phase = state
        .graph_policy
        .as_ref()
        .map_or(policy.mode.as_str(), |graph| graph.phase.as_str());
    format!(
        "effective policy refused {tool}\nactive_mode={}\nnode={node}\nphase={phase}\nreason={reason}\nallowed_tools={}\npreferred_next_action={}\nvalid_example:\n{}",
        policy.mode,
        join_or_none(&policy.allowed_tools),
        effective_preferred_action(policy, state, Some(tool)),
        effective_example(policy, state, Some(tool))
    )
}

fn effective_preferred_action(
    policy: &EffectivePolicy,
    state: &DispatchState,
    blocked: Option<&str>,
) -> String {
    if let Some(graph) = state.graph_policy.as_ref() {
        return preferred_action(graph, blocked);
    }
    if policy
        .allowed_tools
        .iter()
        .any(|tool| tool == &policy.preferred_next_action)
    {
        return policy.preferred_next_action.clone();
    }
    policy
        .allowed_tools
        .iter()
        .find(|tool| blocked != Some(tool.as_str()))
        .cloned()
        .unwrap_or_else(|| "none".to_string())
}

fn effective_example(
    policy: &EffectivePolicy,
    state: &DispatchState,
    blocked: Option<&str>,
) -> String {
    if let Some(graph) = state.graph_policy.as_ref() {
        return example_for(graph, blocked);
    }
    let preferred = effective_preferred_action(policy, state, blocked);
    registry_valid_example(&preferred).unwrap_or_else(|| "none".to_string())
}
