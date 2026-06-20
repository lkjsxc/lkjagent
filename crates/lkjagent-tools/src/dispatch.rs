mod fs_extra_tools;
mod fs_tools;
mod graph_evidence_tools;
mod graph_tools;
mod guards;
mod memory_tools;
mod params;
mod queue_tools;
mod routes;
mod routes_doc;
mod routes_verify;
mod routes_workspace;
mod state;
mod validate;

use lkjagent_protocol::{render_action, Action};
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};
use crate::observe::{self, OutputFrame};
use routes::route;
pub use state::{
    DispatchOutput, DispatchState, GraphDispatchPolicy, GraphEvidenceRecord, ReadRecord,
    ToolRuntime,
};
use validate::validate_action;

pub fn dispatch(
    action: &Action,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    dispatch_with_text(action, &render_action(action), runtime, conn, state)
}

pub fn dispatch_with_text(
    action: &Action,
    action_text: &str,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    state: &mut DispatchState,
) -> DispatchOutput {
    if state.last_action_text.as_deref() == Some(action_text) {
        state.repeat_count = state.repeat_count.saturating_add(1);
        let prior = state
            .last_frame_ref
            .map_or_else(|| "previous frame".to_string(), |id| format!("frame {id}"));
        return finish(
            state,
            action_text,
            observe::notice("error", format!("repeat action refused; see {prior}")),
        );
    }
    state.repeat_count = 0;
    let validated = match validate_action(action) {
        Ok(validated) => validated,
        Err(message) => return finish(state, action_text, observe::notice("error", message)),
    };
    if let Some(message) = graph_policy_refusal(&validated.tool, state) {
        return finish(state, action_text, observe::notice("error", message));
    }
    route(validated, action_text, runtime, conn, state)
}

fn graph_policy_refusal(tool: &str, state: &DispatchState) -> Option<String> {
    if tool == "agent.done" {
        return None;
    }
    let policy = state.graph_policy.as_ref()?;
    if policy.allowed_tools.iter().any(|allowed| allowed == tool) {
        return None;
    }
    let allowed = policy
        .allowed_tools
        .iter()
        .take(4)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "graph policy refused {tool}; node={} phase={}; allowed next tools: {allowed}",
        policy.active_node, policy.phase
    ))
}

pub(crate) fn observe_result(
    result: ToolResult<String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    match result {
        Ok(content) => finish(
            state,
            action_text,
            observe::ok(
                content,
                runtime.observation_tokens,
                "rerun a narrower tool action",
            ),
        ),
        Err(error) => observe_error(error, action_text, runtime, state),
    }
}

pub(crate) fn observe_error(
    error: ToolError,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let mut message = error.to_string();
    if let Some(hint) = shell_error_hint(action_text) {
        message.push_str("; ");
        message.push_str(hint);
    }
    finish(
        state,
        action_text,
        observe::error(message, runtime.observation_tokens),
    )
}

fn shell_error_hint(action_text: &str) -> Option<&'static str> {
    if !action_text.contains("<tool>shell.run</tool>") {
        return None;
    }
    if action_text.contains("/workspace") {
        return Some("shell.run already starts in the workspace; do not cd /workspace");
    }
    if action_text.contains('{') && action_text.contains('}') {
        return Some(
            "/bin/sh does not expand brace lists; spell directories explicitly or loop over words",
        );
    }
    None
}

pub(crate) fn finish(
    state: &mut DispatchState,
    action_text: &str,
    frame: OutputFrame,
) -> DispatchOutput {
    let frame_ref = state.next_frame_ref;
    state.next_frame_ref = state.next_frame_ref.saturating_add(1);
    state.last_action_text = Some(action_text.to_string());
    state.last_frame_ref = Some(frame_ref);
    DispatchOutput {
        frame_ref,
        kind: frame.kind,
        content: frame.content,
        rendered: frame.rendered,
    }
}
