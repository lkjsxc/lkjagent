use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{
    finish, observe_error, DispatchOutput, DispatchState, GraphEvidenceRecord, ToolRuntime,
};
use crate::error::ToolError;
use crate::observe;

pub fn dispatch_graph_evidence(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let kind = param(params, "kind");
    if let Some(policy) = &state.graph_policy {
        if !policy
            .evidence_requirements
            .iter()
            .any(|item| item == &kind)
        {
            return observe_error(
                ToolError::invalid(format!("unknown graph evidence requirement: {kind}")),
                action_text,
                runtime,
                state,
            );
        }
    }
    let summary = param(params, "summary");
    let path = params
        .get("path")
        .filter(|value| !value.trim().is_empty())
        .cloned();
    let content = match path.as_deref() {
        Some(path) => {
            format!("graph evidence recorded\nkind={kind}\npath={path}\nsummary={summary}")
        }
        None => format!("graph evidence recorded\nkind={kind}\nsummary={summary}"),
    };
    let output = finish(
        state,
        action_text,
        observe::ok(content, runtime.observation_tokens, "inspect graph.state"),
    );
    state.graph_evidence.push(GraphEvidenceRecord {
        kind,
        summary,
        path,
        frame_ref: output.frame_ref,
    });
    output
}

pub fn dispatch_graph_compact(
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    finish(
        state,
        action_text,
        observe::ok(
            "graph compaction checkpoint requested",
            runtime.observation_tokens,
            "wait for compaction notice",
        ),
    )
}
