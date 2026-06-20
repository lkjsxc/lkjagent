use crate::dispatch::{finish, DispatchOutput, DispatchState, ToolRuntime};
use crate::observe;

pub fn dispatch_graph_next(
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let content = state.graph_policy.as_ref().map_or_else(
        || "no active graph case".to_string(),
        |policy| {
            format!(
                "node={}\nphase={}\nlegal_transitions={}\nmissing={}\nallowed_tools={}\nblocked_tools={}\npreferred_next_action={}",
                policy.active_node,
                policy.phase,
                join_or_none(&policy.legal_transitions),
                missing_line(state),
                join_or_none(&policy.allowed_tools),
                join_or_none(&policy.blocked_tools),
                preferred_next_action(policy.plan_ready, policy.completion_ready)
            )
        },
    );
    finish(
        state,
        action_text,
        observe::ok(
            content,
            runtime.observation_tokens,
            "follow graph.next guidance",
        ),
    )
}

pub fn dispatch_graph_audit(
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let content = state.graph_policy.as_ref().map_or_else(
        || "graph_audit=no_active_case".to_string(),
        |policy| {
            format!(
                "graph_audit=ok\nnode={}\nplan_ready={}\ncompletion_ready={}\nmissing={}\nshell_allowed={}",
                policy.active_node,
                policy.plan_ready,
                policy.completion_ready,
                missing_line(state),
                policy.shell_allowed
            )
        },
    );
    finish(
        state,
        action_text,
        observe::ok(content, runtime.observation_tokens, "inspect graph.next"),
    )
}

pub fn dispatch_graph_recover(
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let policy_line = state.graph_policy.as_ref().map_or_else(
        || "node=none".to_string(),
        |policy| format!("node={}\nphase={}", policy.active_node, policy.phase),
    );
    let content = format!(
        "{policy_line}\nrecovery_ladder=inspect-state,smaller-scope,alternate-native-tool,replan,admitted-shell-escape,block-step-and-continue\nrepeat_count={}\nnext=use graph.next or a non-repeating native tool",
        state.repeat_count
    );
    finish(
        state,
        action_text,
        observe::ok(
            content,
            runtime.observation_tokens,
            "choose a different action",
        ),
    )
}

fn join_or_none(values: &[String]) -> String {
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

fn missing_line(state: &DispatchState) -> String {
    if state.graph_missing.is_empty() {
        "none".to_string()
    } else {
        join_or_none(&state.graph_missing)
    }
}

fn preferred_next_action(plan_ready: bool, completion_ready: bool) -> &'static str {
    if !plan_ready {
        "record graph.plan after survey/context"
    } else if completion_ready {
        "agent.done with evidence summary"
    } else {
        "execute or verify the active step"
    }
}
