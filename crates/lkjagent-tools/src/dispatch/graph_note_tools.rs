use std::collections::BTreeMap;

use crate::dispatch::params::param;
use crate::dispatch::{
    finish, observe_error, DispatchOutput, DispatchState, GraphEvidenceRecord, ToolRuntime,
};
use crate::error::ToolError;
use crate::observe;

pub fn dispatch_graph_note(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let raw_kind = param(params, "kind");
    let summary = param(params, "summary");
    let Some(kind) = normalize_kind(&raw_kind, &summary) else {
        return observe_error(
            ToolError::invalid(unknown_kind_message()),
            action_text,
            runtime,
            state,
        );
    };
    let content = if kind == raw_kind {
        format!("graph note recorded\nkind={kind}")
    } else {
        format!("graph note recorded\nkind={kind}\nnormalized_from={raw_kind}")
    };
    let output = finish(
        state,
        action_text,
        observe::ok(content, runtime.observation_tokens, "inspect graph.state"),
    );
    if kind != raw_kind {
        state.graph_evidence.push(GraphEvidenceRecord {
            kind: "action-normalization".to_string(),
            summary: format!("normalized graph.note kind {raw_kind} to {kind}"),
            path: None,
            frame_ref: output.frame_ref,
        });
    }
    output
}

fn normalize_kind(kind: &str, summary: &str) -> Option<String> {
    if allowed().contains(&kind) {
        return Some(kind.to_string());
    }
    match kind {
        "planning" | "note" | "recovery" | "compaction-state" => Some("decision".to_string()),
        "progress" if completed(summary) => Some("success".to_string()),
        "progress" => Some("decision".to_string()),
        "policy-refinement" if constraint_like(summary) => Some("constraint".to_string()),
        "policy-refinement" => Some("decision".to_string()),
        _ => None,
    }
}

fn completed(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    ["complete", "completed", "done", "finished", "passed"]
        .iter()
        .any(|needle| lower.contains(needle))
}

fn constraint_like(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    ["must", "never", "only", "required", "constraint"]
        .iter()
        .any(|needle| lower.contains(needle))
}

fn unknown_kind_message() -> String {
    let example = "<act>\n<tool>graph.note</tool>\n<kind>decision</kind>\n<summary>Chose smaller recovery action</summary>\n</act>";
    format!(
        "unknown graph.note kind; allowed: {}; valid_example:\n{example}",
        allowed().join(", ")
    )
}

fn allowed() -> Vec<&'static str> {
    vec![
        "constraint",
        "assumption",
        "risk",
        "decision",
        "question",
        "invariant",
        "success",
        "path",
    ]
}
