use crate::kernel::decision::{ActionTemplate, RuntimeMission};
use crate::kernel::render::example_for;
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};

pub(crate) fn next_action_for(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
) -> Option<ActionTemplate> {
    let tool = match mission {
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => return None,
        RuntimeMission::OwnerRecovery => "graph.state",
        RuntimeMission::SchemaRepair => schema_tool(snapshot),
        RuntimeMission::ArtifactRepair => artifact_tool(snapshot),
        RuntimeMission::VerificationRepair => "artifact.audit",
        RuntimeMission::OwnerExecution => owner_execution_tool(snapshot),
        RuntimeMission::OwnerVerification => "artifact.audit",
        RuntimeMission::OwnerCompletion if snapshot.evidence.missing.is_empty() => "agent.done",
        RuntimeMission::OwnerCompletion => "artifact.audit",
        RuntimeMission::IdleMaintenance => "memory.find",
    };
    Some(ActionTemplate::ExactTool {
        tool: ToolName::from_static(tool),
        body: example_for(tool, snapshot),
    })
}

pub(crate) fn owner_execution_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if plan_missing(snapshot) {
        return "graph.plan";
    }
    if snapshot.artifact.root.is_none() {
        return "artifact.plan";
    }
    if evidence_missing(snapshot, "document-structure") {
        return "doc.audit";
    }
    if artifact_next_candidate(snapshot) {
        return "fs.batch_write";
    }
    if !snapshot.artifact.weak_paths.is_empty() {
        return "artifact.next";
    }
    if evidence_missing(snapshot, "artifact-readiness") {
        return "artifact.audit";
    }
    "artifact.audit"
}

fn schema_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if repeated_child_tag_batch_fault(snapshot) {
        "artifact.next"
    } else {
        "fs.batch_write"
    }
}

fn artifact_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if artifact_next_candidate(snapshot) {
        "fs.batch_write"
    } else if !snapshot.artifact.weak_paths.is_empty() {
        "artifact.next"
    } else {
        "artifact.audit"
    }
}

fn plan_missing(snapshot: &RuntimeSnapshot) -> bool {
    evidence_missing(snapshot, "plan")
        && !snapshot.evidence.existing.iter().any(|item| item == "plan")
}

fn artifact_next_candidate(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .observation
        .latest
        .as_deref()
        .is_some_and(|value| value.contains("next_decision_required=true"))
}

fn repeated_child_tag_batch_fault(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.retry_count > 0
        && snapshot
            .parameter_shape_fingerprint
            .as_deref()
            .is_some_and(|value| value.contains("child-file-tags"))
}

fn evidence_missing(snapshot: &RuntimeSnapshot, requirement: &str) -> bool {
    snapshot
        .evidence
        .missing
        .iter()
        .any(|missing| missing == requirement)
}
