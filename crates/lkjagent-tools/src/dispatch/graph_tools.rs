use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{finish, observe_error, DispatchOutput, DispatchState, ToolRuntime};
use crate::error::ToolError;
use crate::observe;

pub fn dispatch_graph_state(
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let content = state
        .graph_state
        .clone()
        .unwrap_or_else(|| "no active graph case".to_string());
    finish(
        state,
        action_text,
        observe::ok(
            content,
            runtime.observation_tokens,
            "wait for next graph notice",
        ),
    )
}

pub fn dispatch_graph_plan(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    if param(params, "objective").trim().is_empty() || useful_lines(&param(params, "steps")) == 0 {
        return observe_error(
            ToolError::invalid("graph.plan needs objective and at least one step"),
            action_text,
            runtime,
            state,
        );
    }
    if param(params, "checks").trim().is_empty() && param(params, "paths").trim().is_empty() {
        return observe_error(
            ToolError::invalid("graph.plan needs checks or paths"),
            action_text,
            runtime,
            state,
        );
    }
    finish(
        state,
        action_text,
        observe::ok(
            "graph plan recorded",
            runtime.observation_tokens,
            "inspect graph.state",
        ),
    )
}

pub fn dispatch_graph_transition(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let target = param(params, "target");
    if let Some(policy) = &state.graph_policy {
        if !policy.legal_transitions.iter().any(|item| item == &target) {
            return observe_error(
                ToolError::invalid(format!(
                    "graph transition refused from {} to {}; legal: {}",
                    policy.active_node,
                    target,
                    policy.legal_transitions.join(", ")
                )),
                action_text,
                runtime,
                state,
            );
        }
    }
    finish(
        state,
        action_text,
        observe::ok(
            format!("graph transition admitted\ntarget={target}"),
            runtime.observation_tokens,
            "inspect graph.state",
        ),
    )
}

pub fn dispatch_graph_context(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let packages = param(params, "packages");
    if useful_lines(&packages) == 0 {
        return observe_error(
            ToolError::invalid("graph.context needs at least one package"),
            action_text,
            runtime,
            state,
        );
    }
    if let Some(policy) = &state.graph_policy {
        if let Some(invalid) = package_lines(&packages).into_iter().find(|package| {
            !policy
                .allowed_packages
                .iter()
                .any(|allowed| allowed == package)
        }) {
            return observe_error(
                ToolError::invalid(format!(
                    "graph.context refused package {invalid}; allowed: {}",
                    policy.allowed_packages.join(", ")
                )),
                action_text,
                runtime,
                state,
            );
        }
    }
    finish(
        state,
        action_text,
        observe::ok(
            "graph context selected",
            runtime.observation_tokens,
            "continue graph transition",
        ),
    )
}

pub fn dispatch_graph_note(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let kind = param(params, "kind");
    let allowed = [
        "constraint",
        "assumption",
        "risk",
        "decision",
        "question",
        "invariant",
        "success",
        "path",
    ];
    if !allowed.contains(&kind.as_str()) {
        return observe_error(
            ToolError::invalid("unknown graph.note kind"),
            action_text,
            runtime,
            state,
        );
    }
    finish(
        state,
        action_text,
        observe::ok(
            format!("graph note recorded\nkind={kind}"),
            runtime.observation_tokens,
            "inspect graph.state",
        ),
    )
}

fn useful_lines(value: &str) -> usize {
    value.lines().filter(|line| !line.trim().is_empty()).count()
}

fn package_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
