use crate::model::EvidenceRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRequirementState {
    pub id: String,
    pub description: String,
    pub required_for_completion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceState {
    pub requirements: Vec<EvidenceRequirementState>,
    pub records: Vec<EvidenceRecord>,
    pub pending_checks: Vec<String>,
}

impl EvidenceState {
    pub fn new(requirements: Vec<String>, pending_checks: Vec<String>) -> Self {
        Self {
            requirements: requirements
                .into_iter()
                .map(|id| EvidenceRequirementState {
                    description: format!("{id} evidence"),
                    id,
                    required_for_completion: true,
                })
                .collect(),
            records: Vec::new(),
            pending_checks,
        }
    }

    pub fn requirement_ids(&self) -> Vec<String> {
        self.requirements
            .iter()
            .map(|item| item.id.clone())
            .collect()
    }

    pub fn has(&self, requirement: &str) -> bool {
        self.records
            .iter()
            .any(|record| record.requirement == requirement && record.satisfies_completion)
    }

    pub fn knows_requirement(&self, requirement: &str) -> bool {
        self.requirements.iter().any(|item| item.id == requirement)
    }
}
