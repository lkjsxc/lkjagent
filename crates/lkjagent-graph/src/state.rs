use crate::case_completion::CompletionState;
use crate::case_context::{CaseBudgetState, GraphContextState, WorkspaceState};
use crate::case_document::DocumentState;
use crate::case_evidence::EvidenceState;
use crate::case_fields::{
    AssumptionRecord, ConstraintRecord, DecisionRecord, InvariantRecord, QuestionRecord,
    RiskRecord, SuccessCriterion,
};
use crate::case_objective::ObjectiveState;
use crate::case_plan::PlanState;
use crate::case_recovery::RecoveryState;
use crate::model::{CaseStatus, EvidenceRecord, GraphNodeId, TaskFamily, TaskPhase};
use crate::transition_history::TransitionRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskGraphState {
    pub case_id: Option<i64>,
    pub objective: ObjectiveState,
    pub family: TaskFamily,
    pub phase: TaskPhase,
    pub status: CaseStatus,
    pub active_node: GraphNodeId,
    pub confidence: u8,
    pub constraints: Vec<ConstraintRecord>,
    pub assumptions: Vec<AssumptionRecord>,
    pub open_questions: Vec<QuestionRecord>,
    pub risks: Vec<RiskRecord>,
    pub invariants: Vec<InvariantRecord>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub decisions: Vec<DecisionRecord>,
    pub plan: PlanState,
    pub context: GraphContextState,
    pub workspace: WorkspaceState,
    pub evidence: EvidenceState,
    pub completion: CompletionState,
    pub recovery: RecoveryState,
    pub document: Option<DocumentState>,
    pub transitions: Vec<TransitionRecord>,
    pub budgets: CaseBudgetState,
}

impl TaskGraphState {
    pub fn objective_text(&self) -> &str {
        &self.objective.normalized
    }

    pub fn requirement_ids(&self) -> Vec<String> {
        self.evidence.requirement_ids()
    }

    pub fn selected_packages(&self) -> &[String] {
        &self.context.selected_packages
    }
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
    pub objective: String,
    pub plan_steps: Vec<String>,
    pub constraints: Vec<String>,
    pub evidence: Vec<EvidenceRecord>,
    pub missing_evidence: Vec<String>,
    pub touched_paths: Vec<String>,
    pub selected_packages: Vec<String>,
    pub recovery: RecoveryState,
    pub completion_ready: bool,
}
