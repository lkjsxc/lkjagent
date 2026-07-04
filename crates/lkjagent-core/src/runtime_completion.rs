use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRequirement {
    pub check_name: String,
    pub artifact_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckEvidence {
    pub check_name: String,
    pub artifact_fingerprint: String,
    pub passed: bool,
    pub decision_id: String,
    pub created_at: String,
}

pub fn can_close(requirements: &[CompletionRequirement], evidence: &[CheckEvidence]) -> bool {
    !requirements.is_empty()
        && requirements
            .iter()
            .all(|requirement| has_fresh_pass(requirement, evidence))
}

fn has_fresh_pass(requirement: &CompletionRequirement, evidence: &[CheckEvidence]) -> bool {
    evidence.iter().any(|item| {
        item.passed
            && item.check_name == requirement.check_name
            && item.artifact_fingerprint == requirement.artifact_fingerprint
    })
}
