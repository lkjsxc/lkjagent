use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{finish, DispatchOutput, DispatchState, GraphEvidenceRecord, ToolRuntime};
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

pub fn dispatch_graph_evidence(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let kind = param(params, "kind");
    let summary = param(params, "summary");
    let path = optional_path(params);
    let content = evidence_content(&kind, &summary, path.as_deref());
    let output = finish(
        state,
        action_text,
        observe::ok(
            content,
            runtime.observation_tokens,
            "inspect graph state with graph.state",
        ),
    );
    state.graph_evidence.push(GraphEvidenceRecord {
        kind,
        summary,
        path,
        frame_ref: output.frame_ref,
    });
    output
}

fn optional_path(params: &BTreeMap<String, String>) -> Option<String> {
    params
        .get("path")
        .filter(|value| !value.trim().is_empty())
        .cloned()
}

fn evidence_content(kind: &str, summary: &str, path: Option<&str>) -> String {
    match path {
        Some(path) => {
            format!("graph evidence recorded\nkind={kind}\npath={path}\nsummary={summary}")
        }
        None => format!("graph evidence recorded\nkind={kind}\nsummary={summary}"),
    }
}
