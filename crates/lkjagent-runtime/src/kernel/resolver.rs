#[path = "resolver_label.rs"]
mod resolver_label;
#[path = "resolver_rules.rs"]
mod resolver_rules;

use crate::kernel::decision::{ActionTemplate, RuntimeMission};
use crate::kernel::next_action_simple::{simple_write_body, simple_write_path};
use crate::kernel::obligation::{root_identity_needed, Obligation};
use crate::kernel::obligation_facts::{ArtifactRootStatus, RuntimeFacts, WriteContractFacts};
use crate::kernel::render::example_for;
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};
pub use resolver_label::{resolver_label, resolver_rule_id};
use resolver_rules::mission_rule_plan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TotalResolverPlan {
    RuntimeEffect,
    ExactInspection { tool: &'static str },
    SemanticWriteContract { contract: WriteContractFacts },
    Audit { tool: &'static str },
    EvidenceRecording { tool: &'static str },
    OwnerWait,
    BlockedHandoff { reason: String },
    CloseCase,
}

pub type ResolverPlan = TotalResolverPlan;

pub fn resolve_obligations(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
    obligations: &[Obligation],
    completion_allowed: bool,
) -> TotalResolverPlan {
    if matches!(mission, RuntimeMission::HardRuntimeCompaction) {
        return TotalResolverPlan::RuntimeEffect;
    }
    if matches!(mission, RuntimeMission::ClosedIdle) {
        return TotalResolverPlan::RuntimeEffect;
    }
    if let Some(conflict) = facts.generic_root_conflict.as_deref() {
        return blocked(&format!(
            "generic artifact root conflicts with owner target: {conflict}"
        ));
    }
    if facts.owner_work_exists && completion_allowed && facts.missing_evidence.is_empty() {
        return TotalResolverPlan::CloseCase;
    }
    if let Some(plan) = root_repair_plan(facts) {
        return plan;
    }
    first_obligation_plan(mission, snapshot, facts, obligations)
        .unwrap_or_else(|| mission_rule_plan(mission, snapshot, facts))
}

pub fn action_for_plan(
    plan: &TotalResolverPlan,
    snapshot: &RuntimeSnapshot,
) -> Option<ActionTemplate> {
    match plan {
        TotalResolverPlan::RuntimeEffect
        | TotalResolverPlan::OwnerWait
        | TotalResolverPlan::BlockedHandoff { .. } => None,
        TotalResolverPlan::CloseCase => exact("agent.done", snapshot),
        TotalResolverPlan::ExactInspection { tool } | TotalResolverPlan::Audit { tool } => {
            exact(tool, snapshot)
        }
        TotalResolverPlan::EvidenceRecording { tool } if *tool == "fs.write" => fs_write(snapshot),
        TotalResolverPlan::EvidenceRecording { tool } => exact(tool, snapshot),
        TotalResolverPlan::SemanticWriteContract { .. } => exact("fs.batch_write", snapshot),
    }
}

pub(super) fn root_repair_plan(facts: &RuntimeFacts) -> Option<TotalResolverPlan> {
    if !root_identity_needed(facts.root_status) {
        return None;
    }
    let contract = facts.write_contract.clone()?;
    Some(TotalResolverPlan::SemanticWriteContract { contract })
}

fn first_obligation_plan(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
    obligations: &[Obligation],
) -> Option<TotalResolverPlan> {
    obligations
        .iter()
        .find_map(|obligation| obligation_plan(*obligation, mission, snapshot, facts))
}

fn obligation_plan(
    obligation: Obligation,
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
) -> Option<TotalResolverPlan> {
    match obligation {
        Obligation::Compaction => Some(TotalResolverPlan::RuntimeEffect),
        Obligation::Recovery => Some(mission_rule_plan(mission, snapshot, facts)),
        Obligation::Plan => Some(TotalResolverPlan::EvidenceRecording { tool: "graph.plan" }),
        Obligation::ArtifactIdentity => Some(TotalResolverPlan::ExactInspection {
            tool: "artifact.plan",
        }),
        Obligation::RootIdentity => root_repair_plan(facts),
        Obligation::ContentBatch => facts
            .write_contract
            .clone()
            .map(|contract| TotalResolverPlan::SemanticWriteContract { contract }),
        Obligation::DocumentStructure => Some(document_structure_plan(facts)),
        Obligation::ArtifactReadiness => Some(artifact_readiness_plan(snapshot, facts)),
        Obligation::Verification => Some(TotalResolverPlan::Audit {
            tool: "artifact.audit",
        }),
        Obligation::Completion if mission == RuntimeMission::OwnerCompletion => {
            Some(TotalResolverPlan::Audit {
                tool: "artifact.audit",
            })
        }
        Obligation::BlockedHandoff => Some(blocked("no resolver route remains")),
        Obligation::Completion => None,
    }
}

fn exact(tool: &'static str, snapshot: &RuntimeSnapshot) -> Option<ActionTemplate> {
    Some(ActionTemplate::ExactTool {
        tool: ToolName::from_static(tool),
        body: example_for(tool, snapshot),
    })
}

fn fs_write(snapshot: &RuntimeSnapshot) -> Option<ActionTemplate> {
    let path = simple_write_path(snapshot)?;
    Some(ActionTemplate::ExactTool {
        tool: ToolName::from_static("fs.write"),
        body: simple_write_body(&path, snapshot),
    })
}

fn document_structure_plan(facts: &RuntimeFacts) -> TotalResolverPlan {
    if facts.root.is_some() && facts.root_status == ArtifactRootStatus::StructureFailed {
        TotalResolverPlan::ExactInspection {
            tool: "artifact.next",
        }
    } else {
        TotalResolverPlan::Audit { tool: "doc.audit" }
    }
}

fn artifact_readiness_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> TotalResolverPlan {
    if artifact_next_requested(snapshot)
        || !facts.weak_paths.is_empty()
        || facts.content_atoms.missing_count > 0
    {
        TotalResolverPlan::ExactInspection {
            tool: "artifact.next",
        }
    } else {
        TotalResolverPlan::Audit {
            tool: "artifact.audit",
        }
    }
}

pub(super) fn batch_write_requested(snapshot: &RuntimeSnapshot) -> bool {
    candidate_action(snapshot, "fs.batch_write")
}

pub(super) fn artifact_next_requested(snapshot: &RuntimeSnapshot) -> bool {
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

pub(super) fn blocked(reason: &str) -> TotalResolverPlan {
    TotalResolverPlan::BlockedHandoff {
        reason: reason.to_string(),
    }
}
