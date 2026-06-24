use std::collections::BTreeMap;

use crate::dispatch::examples::{valid_example_for, ExampleContext};
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
    if let Some((tool, reason)) = audit_owned_requirement(&kind) {
        let example = audit_example(tool, params);
        return observe_error(
            ToolError::invalid(format!(
                "audit-owned graph evidence requirement: {kind}\nreason={reason}\nvalid_example:\n{example}"
            )),
            action_text,
            runtime,
            state,
        );
    }
    let known = known_requirements(state);
    if !known.iter().any(|item| item == &kind) {
        let example_kind = known
            .first()
            .cloned()
            .unwrap_or_else(|| "observation".to_string());
        let example = evidence_example(example_kind);
        return observe_error(
            ToolError::invalid(format!(
                "unknown graph evidence requirement: {kind}\nallowed_values={}\nvalid_example:\n{example}",
                known.join(", ")
            )),
            action_text,
            runtime,
            state,
        );
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

fn known_requirements(state: &DispatchState) -> Vec<String> {
    state
        .graph_policy
        .as_ref()
        .map(|policy| policy.evidence_requirements.clone())
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| {
            ["plan", "observation", "verification"]
                .iter()
                .map(|item| (*item).to_string())
                .collect()
        })
        .into_iter()
        .filter(|item| audit_owned_requirement(item).is_none())
        .collect()
}

fn audit_owned_requirement(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind {
        "document-structure" => Some(("doc.audit", "document structure comes from doc.audit")),
        "artifact-readiness" => Some((
            "artifact.audit",
            "artifact readiness comes from content-bearing artifact.audit",
        )),
        _ => None,
    }
}

fn audit_example(tool: &str, params: &BTreeMap<String, String>) -> String {
    valid_example_for(
        tool,
        ExampleContext {
            artifact_root: params.get("path").filter(|path| !path.is_empty()).cloned(),
            ..ExampleContext::default()
        },
    )
    .map(|example| example.render())
    .unwrap_or_else(|_| {
        "<action>\n<tool>doc.audit</tool>\n<root>docs</root>\n</action>".to_string()
    })
}

fn evidence_example(kind: String) -> String {
    valid_example_for(
        "graph.evidence",
        ExampleContext {
            evidence_requirement: Some(kind),
            ..ExampleContext::default()
        },
    )
    .map(|example| example.render())
    .unwrap_or_else(|_| {
        "<action>\n<tool>graph.evidence</tool>\n<kind>plan</kind>\n<summary>Recorded structured plan with checks.</summary>\n</action>".to_string()
    })
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
