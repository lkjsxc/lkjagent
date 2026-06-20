#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionState {
    pub missing_requirements: Vec<String>,
    pub pending_checks: Vec<String>,
    pub ready: bool,
    pub refusal_reason: Option<String>,
    pub final_summary_evidence: Vec<String>,
}

impl CompletionState {
    pub fn new(requirements: Vec<String>, pending_checks: Vec<String>) -> Self {
        Self {
            missing_requirements: requirements,
            pending_checks,
            ready: false,
            refusal_reason: Some("required evidence is missing".to_string()),
            final_summary_evidence: Vec::new(),
        }
    }
}
