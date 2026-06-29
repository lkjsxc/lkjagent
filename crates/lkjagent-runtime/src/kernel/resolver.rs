use crate::kernel::decision::{ActionTemplate, RuntimeMission};
use crate::kernel::obligation::{root_identity_needed, Obligation};
use crate::kernel::obligation_facts::{RuntimeFacts, WriteContractFacts};
use crate::kernel::render::example_for;
use crate::kernel::snapshot::{RuntimeSnapshot, ToolName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverPlan {
    RuntimeEffect,
    ExactInspection { tool: &'static str },
    SemanticWriteContract { contract: WriteContractFacts },
    Audit { tool: &'static str },
    EvidenceRecording { tool: &'static str },
    OwnerWait,
    BlockedHandoff { reason: String },
    CloseCase,
}

pub fn resolve_obligations(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
    obligations: &[Obligation],
) -> Option<ResolverPlan> {
    if matches!(
        mission,
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle
    ) {
        return Some(ResolverPlan::RuntimeEffect);
    }
    if mission == RuntimeMission::OwnerCompletion && facts.missing_evidence.is_empty() {
        return Some(ResolverPlan::CloseCase);
    }
    root_repair_plan(facts).or_else(|| first_obligation_plan(mission, snapshot, facts, obligations))
}

pub fn action_for_plan(plan: &ResolverPlan, snapshot: &RuntimeSnapshot) -> Option<ActionTemplate> {
    match plan {
        ResolverPlan::RuntimeEffect
        | ResolverPlan::OwnerWait
        | ResolverPlan::BlockedHandoff { .. } => None,
        ResolverPlan::CloseCase => exact("agent.done", snapshot),
        ResolverPlan::ExactInspection { tool }
        | ResolverPlan::Audit { tool }
        | ResolverPlan::EvidenceRecording { tool } => exact(tool, snapshot),
        ResolverPlan::SemanticWriteContract { .. } => exact("fs.batch_write", snapshot),
    }
}

fn root_repair_plan(facts: &RuntimeFacts) -> Option<ResolverPlan> {
    if !root_identity_needed(facts.root_status) {
        return None;
    }
    let contract = facts.write_contract.clone()?;
    Some(ResolverPlan::SemanticWriteContract { contract })
}

fn first_obligation_plan(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
    obligations: &[Obligation],
) -> Option<ResolverPlan> {
    for obligation in obligations {
        if let Some(plan) = obligation_plan(*obligation, mission, snapshot, facts) {
            return Some(plan);
        }
    }
    None
}

fn obligation_plan(
    obligation: Obligation,
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
) -> Option<ResolverPlan> {
    match obligation {
        Obligation::Compaction => Some(ResolverPlan::RuntimeEffect),
        Obligation::Recovery => recovery_plan(snapshot, facts),
        Obligation::Plan => Some(ResolverPlan::ExactInspection { tool: "graph.plan" }),
        Obligation::ArtifactIdentity => Some(ResolverPlan::ExactInspection {
            tool: "artifact.plan",
        }),
        Obligation::RootIdentity => root_repair_plan(facts),
        Obligation::ContentBatch => facts
            .write_contract
            .clone()
            .map(|contract| ResolverPlan::SemanticWriteContract { contract }),
        Obligation::DocumentStructure => Some(ResolverPlan::Audit { tool: "doc.audit" }),
        Obligation::ArtifactReadiness | Obligation::Verification => Some(ResolverPlan::Audit {
            tool: "artifact.audit",
        }),
        Obligation::Completion if mission == RuntimeMission::OwnerCompletion => {
            Some(ResolverPlan::Audit {
                tool: "artifact.audit",
            })
        }
        Obligation::BlockedHandoff => Some(ResolverPlan::BlockedHandoff {
            reason: "no resolver route remains".to_string(),
        }),
        Obligation::Completion => None,
    }
}

fn recovery_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> Option<ResolverPlan> {
    if let Some(plan) = root_repair_plan(facts) {
        return Some(plan);
    }
    if facts.write_contract.is_some() && artifact_next_candidate(snapshot) {
        return facts
            .write_contract
            .clone()
            .map(|contract| ResolverPlan::SemanticWriteContract { contract });
    }
    if !facts.weak_paths.is_empty() {
        return Some(ResolverPlan::ExactInspection {
            tool: "artifact.next",
        });
    }
    if facts.root.is_some() {
        return Some(ResolverPlan::Audit {
            tool: "artifact.audit",
        });
    }
    Some(ResolverPlan::ExactInspection {
        tool: "workspace.summary",
    })
}

fn exact(tool: &'static str, snapshot: &RuntimeSnapshot) -> Option<ActionTemplate> {
    Some(ActionTemplate::ExactTool {
        tool: ToolName::from_static(tool),
        body: example_for(tool, snapshot),
    })
}

fn artifact_next_candidate(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .observation
        .latest
        .as_deref()
        .is_some_and(|value| value.contains("next_decision_required=true"))
}
