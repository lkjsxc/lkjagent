use crate::model::{CaseStatus, EvidenceRecord, GraphNodeId, TaskFamily, TaskPhase};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskGraphState {
    pub case_id: Option<i64>,
    pub objective: String,
    pub family: TaskFamily,
    pub phase: TaskPhase,
    pub status: CaseStatus,
    pub active_node: GraphNodeId,
    pub confidence: u8,
    pub plan: String,
    pub risks: Vec<String>,
    pub candidate_paths: Vec<String>,
    pub touched_paths: Vec<String>,
    pub selected_packages: Vec<String>,
    pub evidence_requirements: Vec<String>,
    pub evidence: Vec<EvidenceRecord>,
    pub pending_checks: Vec<String>,
    pub recovery: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionDecision {
    Admit { target: GraphNodeId },
    Defer { missing: Vec<String> },
    Recover { reason: String, target: GraphNodeId },
    Refuse { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionPlan {
    pub case_id: Option<i64>,
    pub phase: TaskPhase,
    pub active_node: GraphNodeId,
    pub plan: String,
    pub evidence: Vec<EvidenceRecord>,
    pub missing_evidence: Vec<String>,
    pub touched_paths: Vec<String>,
    pub selected_packages: Vec<String>,
    pub recovery: Option<String>,
}
