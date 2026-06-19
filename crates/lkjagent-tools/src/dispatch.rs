mod fs_tools;
mod graph_tools;
mod guards;
mod memory_tools;
mod params;
mod queue_tools;
mod routes;
mod state;
mod validate;

use lkjagent_protocol::{render_action, Action};
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};
use crate::observe::{self, OutputFrame};
use routes::route;
pub use state::{DispatchOutput, DispatchState, GraphEvidenceRecord, ReadRecord, ToolRuntime};
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
    route(validated, action_text, runtime, conn, state)
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
    finish(
        state,
        action_text,
        observe::error(error.to_string(), runtime.observation_tokens),
    )
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
