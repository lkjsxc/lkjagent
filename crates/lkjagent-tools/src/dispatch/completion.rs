use std::collections::BTreeMap;

use super::observe_result;
use super::params::param;
use super::state::{AuthorityAdmissionView, DispatchOutput, DispatchState, ToolRuntime};
use crate::control;
use crate::error::ToolError;

pub(crate) fn dispatch_done(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    if state.graph_state.is_some() && !state.graph_completion_ready {
        return observe_result(
            Err(ToolError::invalid(done_refusal(state))),
            action_text,
            runtime,
            state,
        );
    }
    observe_result(
        control::done(
            &mut state.control,
            &runtime.workspace,
            &param(params, "summary"),
        ),
        action_text,
        runtime,
        state,
    )
}

fn done_refusal(state: &DispatchState) -> String {
    let listed = state.graph_missing.join(", ");
    let first = state
        .graph_missing
        .first()
        .cloned()
        .unwrap_or_else(|| "required-evidence".to_string());
    let next = next_completion_action(&first, state.authority_view.as_ref());
    let graph_line = state
        .graph_state
        .as_deref()
        .and_then(|text| text.lines().find(|line| !line.trim().is_empty()))
        .unwrap_or("graph_state=unavailable");
    format!(
        "graph completion refused\npartial_handoff=blocked-with-evidence\nattempted=agent.done\nfailed_gate=completion\nmissing={listed}\nexisting_graph={graph_line}\nnext_executable_action={}\nvalid_example:\n{}",
        next.label, next.example
    )
}

struct CompletionNextAction {
    label: String,
    example: String,
}

fn next_completion_action(
    first: &str,
    view: Option<&AuthorityAdmissionView>,
) -> CompletionNextAction {
    if let Some(example) = authority_example(view) {
        return CompletionNextAction {
            label: format!("run admitted repair for missing {first}"),
            example,
        };
    }
    CompletionNextAction {
        label: "run graph.state before retrying agent.done".to_string(),
        example: "<action>\n<tool>graph.state</tool>\n</action>".to_string(),
    }
}

fn authority_example(view: Option<&AuthorityAdmissionView>) -> Option<String> {
    let example = view?.exact_valid_example.trim();
    if example.is_empty()
        || example == "runtime action; no model action block"
        || example.contains("<tool>agent.done</tool>")
    {
        return None;
    }
    Some(example.to_string())
}
