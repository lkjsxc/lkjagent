mod effective_refusal;
mod examples;
mod fs_extra_tools;
mod fs_more_tools;
mod fs_tools;
mod graph_evidence_tools;
mod graph_inspect_tools;
mod graph_note_tools;
mod graph_tools;
mod guards;
mod memory_tools;
mod normalize;
mod params;
mod queue_tools;
mod refusal;
mod routes;
mod routes_artifact;
mod routes_doc;
mod routes_verify;
mod routes_workspace;
mod state;
mod validate;

use lkjagent_protocol::{render_action, render_notice, render_observation, Action};
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};
use crate::observe::{self, OutputFrame};
pub use examples::{registry_valid_example, valid_example_for, ActionExample, ExampleContext};
use normalize::{normalize_action, NormalizationDecision, NormalizationNote};
use refusal::{policy_refusal, repeat_refusal};
use routes::route;
pub use state::{
    DispatchOutput, DispatchState, EffectivePolicy, GraphDispatchPolicy, GraphEvidenceRecord,
    ReadRecord, ToolRuntime,
};
pub use validate::validate_action;

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
    if let Some(message) = repeat_refusal(action_text, state) {
        return finish(state, action_text, observe::notice("error", message));
    }
    let (normalized, notes) = match normalize_action(action) {
        NormalizationDecision::Unchanged(action) => (action, Vec::new()),
        NormalizationDecision::Normalized { action, notes } => (action, notes),
    };
    let validated = match validate_action(&normalized) {
        Ok(validated) => validated,
        Err(message) => return finish(state, action_text, observe::notice("error", message)),
    };
    if let Some(message) = policy_refusal(&validated.tool, state) {
        return finish(state, action_text, observe::notice("error", message));
    }
    let output = route(validated, action_text, runtime, conn, state);
    with_normalization(output, notes, state)
}

fn with_normalization(
    mut output: DispatchOutput,
    notes: Vec<NormalizationNote>,
    state: &mut DispatchState,
) -> DispatchOutput {
    if notes.is_empty() {
        return output;
    }
    let note_text = notes
        .iter()
        .map(NormalizationNote::render)
        .collect::<Vec<_>>()
        .join("\n-- normalization --\n");
    state.graph_evidence.push(GraphEvidenceRecord {
        kind: "action-normalization".to_string(),
        summary: note_text.clone(),
        path: None,
        frame_ref: output.frame_ref,
    });
    output.content = format!("{note_text}\n\n{}", output.content);
    output.rendered = match &output.kind {
        observe::OutputKind::Observation { status } => render_observation(status, &output.content),
        observe::OutputKind::Notice { kind } => render_notice(kind, &output.content),
    };
    output
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
