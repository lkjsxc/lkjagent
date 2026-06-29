use crate::kernel::obligation_facts::{ArtifactRootStatus, RuntimeFacts};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Obligation {
    Plan,
    ArtifactIdentity,
    RootIdentity,
    DocumentStructure,
    ContentBatch,
    ArtifactReadiness,
    Verification,
    Completion,
    Recovery,
    Compaction,
    BlockedHandoff,
}

pub fn obligations_for(facts: &RuntimeFacts) -> Vec<Obligation> {
    let mut obligations = Vec::new();
    if facts.hard_compaction {
        obligations.push(Obligation::Compaction);
    }
    if facts.recovery_required {
        obligations.push(Obligation::Recovery);
    }
    if missing(facts, "plan") {
        obligations.push(Obligation::Plan);
    }
    if facts.root.is_none() && artifact_work_required(facts) {
        obligations.push(Obligation::ArtifactIdentity);
    }
    if root_identity_needed(facts.root_status) {
        obligations.push(Obligation::RootIdentity);
    }
    if facts.write_contract.is_some() && !root_identity_needed(facts.root_status) {
        obligations.push(Obligation::ContentBatch);
    }
    if missing(facts, "document-structure") && !root_identity_needed(facts.root_status) {
        obligations.push(Obligation::DocumentStructure);
    }
    if missing(facts, "artifact-readiness") {
        obligations.push(Obligation::ArtifactReadiness);
    }
    obligations
}

pub fn root_identity_needed(status: ArtifactRootStatus) -> bool {
    matches!(
        status,
        ArtifactRootStatus::Missing
            | ArtifactRootStatus::EmptyDirectory
            | ArtifactRootStatus::IdentityIncomplete
    )
}

fn missing(facts: &RuntimeFacts, requirement: &str) -> bool {
    facts
        .missing_evidence
        .iter()
        .any(|item| item == requirement)
}

fn artifact_work_required(facts: &RuntimeFacts) -> bool {
    facts
        .missing_evidence
        .iter()
        .any(|item| matches!(item.as_str(), "document-structure" | "artifact-readiness"))
        || !facts.weak_paths.is_empty()
}
