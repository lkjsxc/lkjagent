use super::examples::registry_valid_example;
use super::join::join_or_none;
use super::state::{DispatchState, EffectivePolicy, GraphDispatchPolicy};
use lkjagent_protocol::{render_action, Action, Param};

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
    Some(render(tool, refusal_reason(policy, state), policy, state))
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

fn refusal_reason<'a>(policy: &'a EffectivePolicy, state: &DispatchState) -> &'a str {
    if preferred_blocked(policy) || plan_missing_but_blocked(policy, state) {
        return "policy contradiction: required or preferred action is blocked";
    }
    policy.reason.as_str()
}

fn preferred_blocked(policy: &EffectivePolicy) -> bool {
    let preferred = policy.preferred_next_action.as_str();
    !preferred.is_empty()
        && policy.blocked_tools.iter().any(|tool| tool == preferred)
        && !policy.allowed_tools.iter().any(|tool| tool == preferred)
}

fn plan_missing_but_blocked(policy: &EffectivePolicy, state: &DispatchState) -> bool {
    state.graph_missing.iter().any(|item| item == "plan")
        && policy.blocked_tools.iter().any(|tool| tool == "graph.plan")
        && !policy.allowed_tools.iter().any(|tool| tool == "graph.plan")
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
    if policy
        .allowed_tools
        .iter()
        .any(|tool| tool == &policy.preferred_next_action && blocked != Some(tool.as_str()))
    {
        return policy.preferred_next_action.clone();
    }
    priority_tool(policy, state.graph_policy.as_ref(), blocked)
        .or_else(|| {
            policy
                .allowed_tools
                .iter()
                .find(|tool| blocked != Some(tool.as_str()))
                .cloned()
        })
        .unwrap_or_else(|| "none".to_string())
}

fn priority_tool(
    policy: &EffectivePolicy,
    graph: Option<&GraphDispatchPolicy>,
    blocked: Option<&str>,
) -> Option<String> {
    let priority = [
        "graph.plan",
        "graph.recover",
        "graph.transition",
        "artifact.next",
        "artifact.audit",
        "doc.audit",
        "fs.batch_write",
        "fs.write",
        "fs.read",
        "fs.list",
        "workspace.summary",
    ];
    priority
        .iter()
        .find(|tool| admitted(policy, graph, blocked, tool))
        .map(|tool| (*tool).to_string())
}

fn admitted(
    policy: &EffectivePolicy,
    graph: Option<&GraphDispatchPolicy>,
    blocked: Option<&str>,
    tool: &str,
) -> bool {
    if blocked == Some(tool) || !policy.allowed_tools.iter().any(|allowed| allowed == tool) {
        return false;
    }
    tool != "graph.transition" || graph.is_some_and(|graph| !graph.legal_transitions.is_empty())
}

fn effective_example(
    policy: &EffectivePolicy,
    state: &DispatchState,
    blocked: Option<&str>,
) -> String {
    let preferred = effective_preferred_action(policy, state, blocked);
    if preferred == "graph.transition" {
        return transition_example(state.graph_policy.as_ref());
    }
    registry_valid_example(&preferred).unwrap_or_else(|| "none".to_string())
}

fn transition_example(graph: Option<&GraphDispatchPolicy>) -> String {
    let Some(target) = graph.and_then(|graph| graph.legal_transitions.first()) else {
        return "none".to_string();
    };
    render_action(&Action::new(
        "graph.transition",
        vec![
            Param::new("target", target),
            Param::new("reason", "Use an admitted graph transition"),
        ],
    ))
}
