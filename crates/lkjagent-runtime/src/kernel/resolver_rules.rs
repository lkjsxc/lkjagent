use super::{
    artifact_next_requested, batch_write_requested, blocked, root_repair_plan, TotalResolverPlan,
};
use crate::kernel::decision::RuntimeMission;
use crate::kernel::next_action_simple::artifact_work_required;
use crate::kernel::obligation_facts::RuntimeFacts;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(super) fn mission_rule_plan(
    mission: RuntimeMission,
    snapshot: &RuntimeSnapshot,
    facts: &RuntimeFacts,
) -> TotalResolverPlan {
    match mission {
        RuntimeMission::OwnerRecovery => recovery_plan(snapshot, facts),
        RuntimeMission::SchemaRepair => schema_plan(snapshot, facts),
        RuntimeMission::ArtifactRepair => artifact_plan(snapshot, facts),
        RuntimeMission::VerificationRepair | RuntimeMission::OwnerVerification => {
            TotalResolverPlan::Audit {
                tool: "artifact.audit",
            }
        }
        RuntimeMission::OwnerExecution => owner_execution_plan(snapshot, facts),
        RuntimeMission::OwnerCompletion => completion_repair_plan(snapshot),
        RuntimeMission::IdleMaintenance => TotalResolverPlan::EvidenceRecording {
            tool: "memory.find",
        },
        RuntimeMission::HardRuntimeCompaction | RuntimeMission::ClosedIdle => {
            TotalResolverPlan::RuntimeEffect
        }
    }
}

fn owner_execution_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> TotalResolverPlan {
    if !artifact_work_required(snapshot) {
        if evidence_missing(snapshot, "observation") {
            return TotalResolverPlan::EvidenceRecording { tool: "fs.write" };
        }
        if evidence_missing(snapshot, "verification") {
            return TotalResolverPlan::EvidenceRecording {
                tool: "graph.evidence",
            };
        }
        return TotalResolverPlan::ExactInspection {
            tool: "workspace.summary",
        };
    }
    if snapshot.artifact.root.is_none() {
        return TotalResolverPlan::ExactInspection {
            tool: "artifact.plan",
        };
    }
    artifact_plan(snapshot, facts)
}

fn completion_repair_plan(snapshot: &RuntimeSnapshot) -> TotalResolverPlan {
    if evidence_missing(snapshot, "document-structure") {
        return TotalResolverPlan::Audit { tool: "doc.audit" };
    }
    if evidence_missing(snapshot, "artifact-readiness") || !snapshot.artifact.weak_paths.is_empty()
    {
        return TotalResolverPlan::Audit {
            tool: "artifact.audit",
        };
    }
    blocked("completion gate inputs missing")
}

fn recovery_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> TotalResolverPlan {
    if let Some(plan) = root_repair_plan(facts) {
        return plan;
    }
    if let Some(manuscript) = facts.manuscript.as_ref() {
        if manuscript.anomaly_shrink_level >= 2 {
            return blocked(&format!(
                "manuscript provider anomaly blocked next_path={}",
                manuscript.next_path.as_deref().unwrap_or("unknown")
            ));
        }
        if facts.write_contract.is_some() {
            return write_contract_plan(facts, "manuscript write contract missing");
        }
    }
    if artifact_next_requested(snapshot) || !facts.weak_paths.is_empty() {
        return TotalResolverPlan::ExactInspection {
            tool: "artifact.next",
        };
    }
    if facts.write_contract.is_some() && batch_write_requested(snapshot) {
        return write_contract_plan(facts, "write contract missing");
    }
    if facts.root.is_some() {
        return TotalResolverPlan::Audit {
            tool: "artifact.audit",
        };
    }
    TotalResolverPlan::ExactInspection {
        tool: "workspace.summary",
    }
}

fn schema_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> TotalResolverPlan {
    if repeated_child_tag_batch_fault(snapshot) {
        return TotalResolverPlan::ExactInspection {
            tool: "artifact.next",
        };
    }
    write_contract_plan(facts, "schema repair write contract missing")
}

fn artifact_plan(snapshot: &RuntimeSnapshot, facts: &RuntimeFacts) -> TotalResolverPlan {
    if batch_write_requested(snapshot) {
        return write_contract_plan(facts, "artifact write contract missing");
    }
    if artifact_next_requested(snapshot) || !snapshot.artifact.weak_paths.is_empty() {
        TotalResolverPlan::ExactInspection {
            tool: "artifact.next",
        }
    } else {
        TotalResolverPlan::Audit {
            tool: "artifact.audit",
        }
    }
}

fn write_contract_plan(facts: &RuntimeFacts, missing_reason: &str) -> TotalResolverPlan {
    facts.write_contract.clone().map_or_else(
        || blocked(missing_reason),
        |contract| TotalResolverPlan::SemanticWriteContract { contract },
    )
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
