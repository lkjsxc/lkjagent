use crate::kernel::decision::{ActionTemplate, RuntimeMission};
use crate::kernel::event::RuntimeEvent;
use crate::kernel::next_action_simple::{
    artifact_work_required, simple_write_body, simple_write_path,
};
use crate::kernel::obligation::obligations_for;
use crate::kernel::obligation_facts::runtime_facts;
use crate::kernel::render::example_for;
use crate::kernel::resolver::{action_for_plan, resolve_obligations};
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};

pub(crate) fn next_action_for(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    event: &RuntimeEvent,
) -> Option<ActionTemplate> {
    let facts = runtime_facts(snapshot, event);
    let obligations = obligations_for(&facts);
    if let Some(plan) = resolve_obligations(mission, snapshot, &facts, &obligations) {
        return action_for_plan(&plan, snapshot);
    }
    let tool = match mission {
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => return None,
        RuntimeMission::OwnerRecovery => recovery_tool(snapshot),
        RuntimeMission::SchemaRepair => schema_tool(snapshot),
        RuntimeMission::ArtifactRepair => artifact_tool(snapshot),
        RuntimeMission::VerificationRepair => "artifact.audit",
        RuntimeMission::OwnerExecution => return Some(owner_execution_action(snapshot)),
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

fn owner_execution_action(snapshot: &RuntimeSnapshot) -> ActionTemplate {
    let tool = owner_execution_tool(snapshot);
    if tool == "fs.write" {
        if let Some(path) = simple_write_path(snapshot) {
            return ActionTemplate::ExactTool {
                tool: ToolName::from_static("fs.write"),
                body: simple_write_body(&path, snapshot),
            };
        }
    }
    ActionTemplate::ExactTool {
        tool: ToolName::from_static(tool),
        body: example_for(tool, snapshot),
    }
}

pub(crate) fn owner_execution_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if plan_missing(snapshot) {
        return "graph.plan";
    }
    if !artifact_work_required(snapshot) {
        if evidence_missing(snapshot, "observation") {
            return "fs.write";
        }
        if evidence_missing(snapshot, "verification") {
            return "graph.evidence";
        }
        return "workspace.summary";
    }
    if snapshot.artifact.root.is_none() {
        return "artifact.plan";
    }
    if root_identity_observation(snapshot) || batch_write_requested(snapshot) {
        return "fs.batch_write";
    }
    if artifact_next_requested(snapshot) {
        return "artifact.next";
    }
    if evidence_missing(snapshot, "document-structure") {
        return "doc.audit";
    }
    if !snapshot.artifact.weak_paths.is_empty() {
        return "artifact.next";
    }
    if evidence_missing(snapshot, "artifact-readiness") {
        return "artifact.audit";
    }
    "artifact.audit"
}

fn recovery_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if root_identity_observation(snapshot) || batch_write_requested(snapshot) {
        return "fs.batch_write";
    }
    if artifact_next_requested(snapshot) {
        return "artifact.next";
    }
    if evidence_missing(snapshot, "document-structure") {
        return "doc.audit";
    }
    if evidence_missing(snapshot, "artifact-readiness") {
        return artifact_tool(snapshot);
    }
    if snapshot.retry_count > 0 {
        if !snapshot.artifact.weak_paths.is_empty() {
            return "artifact.next";
        }
        if snapshot.artifact.root.is_some() {
            return "artifact.audit";
        }
        return "workspace.summary";
    }
    "graph.state"
}

fn schema_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if repeated_child_tag_batch_fault(snapshot) {
        "artifact.next"
    } else {
        "fs.batch_write"
    }
}

fn artifact_tool(snapshot: &RuntimeSnapshot) -> &'static str {
    if batch_write_requested(snapshot) {
        "fs.batch_write"
    } else if artifact_next_requested(snapshot) || !snapshot.artifact.weak_paths.is_empty() {
        "artifact.next"
    } else {
        "artifact.audit"
    }
}

fn plan_missing(snapshot: &RuntimeSnapshot) -> bool {
    evidence_missing(snapshot, "plan")
        && !snapshot.evidence.existing.iter().any(|item| item == "plan")
}

fn batch_write_requested(snapshot: &RuntimeSnapshot) -> bool {
    candidate_action(snapshot, "fs.batch_write")
}

fn artifact_next_requested(snapshot: &RuntimeSnapshot) -> bool {
    candidate_action(snapshot, "artifact.next")
}

fn candidate_action(snapshot: &RuntimeSnapshot, action: &str) -> bool {
    let needle = format!("candidate_action={action}");
    [
        snapshot.observation.latest.as_deref(),
        snapshot.observation.latest_successful.as_deref(),
    ]
    .into_iter()
    .flatten()
    .any(|value| value.contains("next_decision_required=true") && value.contains(&needle))
}

fn root_identity_observation(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .observation
        .latest
        .as_deref()
        .is_some_and(|value| value.contains("missing_root") || value.contains("root_missing"))
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
