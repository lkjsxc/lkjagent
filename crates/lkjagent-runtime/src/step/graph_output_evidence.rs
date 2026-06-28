use lkjagent_graph::{EvidenceKind, TaskGraphState};
use lkjagent_protocol::Action;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::graph_state::{evidence_record, status_str};
use crate::step::action_params::action_param;
use crate::step::graph_phase::evidence_kind_for;
use crate::step::Effect;

pub(super) fn add_document_evidence(
    graph: &mut TaskGraphState,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> bool {
    let observed = ensure_evidence(
        graph,
        "observation",
        EvidenceKind::Observation,
        output,
        None,
        effects,
    );
    let structured = if output.content.contains("document audit passed")
        || output.content.contains("artifact audit passed")
    {
        ensure_evidence(
            graph,
            "document-structure",
            EvidenceKind::File,
            output,
            None,
            effects,
        )
    } else {
        false
    };
    let ready = if artifact_readiness_passed(&output.content) {
        ensure_evidence(
            graph,
            "artifact-readiness",
            EvidenceKind::File,
            output,
            None,
            effects,
        )
    } else {
        false
    };
    observed || structured || ready
}

fn artifact_readiness_passed(content: &str) -> bool {
    content.contains("readiness=content-bearing")
        || content.contains("readiness=story-semantic-content")
}

pub(super) fn add_shell_evidence(
    graph: &mut TaskGraphState,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> bool {
    let observed = ensure_evidence(
        graph,
        "observation",
        EvidenceKind::Observation,
        output,
        None,
        effects,
    );
    let verified = ensure_evidence(
        graph,
        "verification",
        EvidenceKind::Verification,
        output,
        None,
        effects,
    );
    observed || verified
}

pub(super) fn add_explicit_graph_evidence(
    graph: &mut TaskGraphState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> bool {
    let requirement = action_param(action, "kind");
    if matches!(
        requirement.as_str(),
        "document-structure" | "artifact-readiness"
    ) {
        return false;
    }
    if !graph.evidence.knows_requirement(&requirement) {
        return false;
    }
    let path = action_param(action, "path");
    add_evidence(
        graph,
        &requirement,
        evidence_kind_for(&requirement),
        action_param(action, "summary"),
        (!path.is_empty()).then_some(path),
        effects,
    )
}

pub(super) fn ensure_evidence(
    graph: &mut TaskGraphState,
    requirement: &str,
    kind: EvidenceKind,
    output: &DispatchOutput,
    path: Option<String>,
    effects: &mut Vec<Effect>,
) -> bool {
    if !graph.evidence.knows_requirement(requirement) {
        return false;
    }
    let summary = output
        .content
        .lines()
        .next()
        .unwrap_or("tool output")
        .to_string();
    add_evidence(graph, requirement, kind, summary, path, effects)
}

pub(super) fn push_case_update(graph: &TaskGraphState, effects: &mut Vec<Effect>) {
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::UpdateGraphCase {
            case_id,
            phase: graph.phase.as_str().to_string(),
            active_node: graph.active_node.0.to_string(),
            status: status_str(graph.status).to_string(),
        });
    }
}

fn add_evidence(
    graph: &mut TaskGraphState,
    requirement: &str,
    kind: EvidenceKind,
    summary: String,
    path: Option<String>,
    effects: &mut Vec<Effect>,
) -> bool {
    if graph.evidence.has(requirement) {
        return false;
    }
    let evidence = evidence_record(requirement, kind, summary, path);
    push_evidence_record(graph, &evidence, effects);
    graph.evidence.records.push(evidence);
    graph.evidence.pending_checks.retain(|check| {
        let satisfied = (requirement == "verification" && check == "focused verification")
            || (requirement == "document-structure" && check == "document audit")
            || (requirement == "artifact-readiness" && check == "artifact readiness audit");
        !satisfied
    });
    true
}

fn push_evidence_record(
    graph: &TaskGraphState,
    evidence: &lkjagent_graph::EvidenceRecord,
    effects: &mut Vec<Effect>,
) {
    if let Some(case_id) = graph.case_id {
        effects.push(Effect::RecordGraphEvidence {
            case_id,
            requirement: evidence.requirement.clone(),
            kind: evidence.kind.as_str().to_string(),
            summary: evidence.summary.clone(),
            path: evidence.path.clone(),
        });
    }
}
