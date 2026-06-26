use crate::mode::{ActiveMode, EndpointDecision, RuntimeSnapshot};
use lkjagent_tools::dispatch::{registry_valid_example, valid_example_for, ExampleContext};

pub(super) fn valid_example_for_mode(
    mode: ActiveMode,
    endpoint_decision: EndpointDecision,
    snapshot: &RuntimeSnapshot,
) -> String {
    if endpoint_decision != EndpointDecision::CallModel {
        return "runtime action; no model action block".to_string();
    }
    match mode {
        ActiveMode::OwnerTask if plan_evidence_missing(snapshot) => {
            rendered_context_example("graph.plan", snapshot)
        }
        ActiveMode::OwnerTask if audit_evidence_missing(snapshot) => {
            rendered_context_example("artifact.audit", snapshot)
        }
        ActiveMode::OwnerTask => rendered_registry_example("graph.state"),
        ActiveMode::Recovery => rendered_registry_example("graph.recover"),
        ActiveMode::Maintenance => rendered_registry_example("memory.find"),
        ActiveMode::Compaction | ActiveMode::ClosedIdle => {
            "runtime action; no model action block".to_string()
        }
    }
}

fn plan_evidence_missing(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .missing_evidence
        .iter()
        .any(|evidence| evidence == "plan")
}

fn audit_evidence_missing(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.active_artifact.is_some()
        && snapshot.missing_evidence.iter().any(|evidence| {
            matches!(
                evidence.as_str(),
                "document-structure" | "artifact-readiness"
            )
        })
}

fn rendered_context_example(tool: &str, snapshot: &RuntimeSnapshot) -> String {
    let context = ExampleContext {
        artifact_root: snapshot.active_artifact.clone(),
        missing_evidence: snapshot.missing_evidence.clone(),
        ..ExampleContext::default()
    };
    valid_example_for(tool, context)
        .map(|example| example.render())
        .unwrap_or_else(|_| rendered_registry_example(tool))
}

fn rendered_registry_example(tool: &str) -> String {
    registry_valid_example(tool)
        .unwrap_or_else(|| format!("<action>\n<tool>{tool}</tool>\n</action>"))
}
