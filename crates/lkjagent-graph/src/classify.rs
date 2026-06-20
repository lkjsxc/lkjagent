use crate::case_completion::CompletionState;
use crate::case_context::{CaseBudgetState, GraphContextState, WorkspaceState};
use crate::case_evidence::EvidenceState;
use crate::case_fields::{ConstraintRecord, FieldStatus, RiskRecord};
use crate::case_objective::ObjectiveState;
use crate::case_plan::PlanState;
use crate::case_recovery::RecoveryState;
use crate::classify_signals::{
    documentation_request, knowledge_request, priority_counted_content_request,
};
use crate::model::{CaseStatus, GraphNodeId, TaskFamily, TaskPhase};
use crate::state::TaskGraphState;

pub fn classify_intent(content: &str) -> TaskFamily {
    let lower = content.to_ascii_lowercase();
    if lower.contains("compact") || lower.contains("context pressure") {
        TaskFamily::Compaction
    } else if lower.contains("recover") || lower.contains("failure") {
        TaskFamily::Recovery
    } else if lower.contains("benchmark") {
        TaskFamily::Benchmark
    } else if priority_counted_content_request(&lower, content) {
        TaskFamily::Documentation
    } else if lower.contains("architecture") || lower.contains("redesign") {
        TaskFamily::Architecture
    } else if lower.contains("bug")
        || lower.contains("fix")
        || content.contains("バグ")
        || content.contains("修正")
    {
        TaskFamily::BugFix
    } else if lower.contains("verify") || lower.contains("test") {
        TaskFamily::Verification
    } else if knowledge_request(&lower, content) {
        TaskFamily::KnowledgeBase
    } else if documentation_request(&lower, content) {
        TaskFamily::Documentation
    } else if lower.contains("maintain") || lower.contains("cleanup") {
        TaskFamily::Maintenance
    } else {
        TaskFamily::CodeChange
    }
}

pub fn initial_state(objective: &str, case_id: Option<i64>) -> TaskGraphState {
    let family = classify_intent(objective);
    let objective_state = ObjectiveState::new(objective);
    let requirements = requirements_for(family);
    let pending_checks = checks_for(family);
    TaskGraphState {
        case_id,
        objective: objective_state.clone(),
        family,
        phase: TaskPhase::Planning,
        status: CaseStatus::Active,
        active_node: GraphNodeId("plan"),
        confidence: confidence_for(family),
        constraints: constraints_from_objective(&objective_state),
        assumptions: Vec::new(),
        open_questions: Vec::new(),
        risks: initial_risks(),
        invariants: Vec::new(),
        success_criteria: Vec::new(),
        decisions: Vec::new(),
        plan: PlanState::empty(objective_state.normalized.clone()),
        context: GraphContextState::new(packages_for(family)),
        workspace: WorkspaceState::default(),
        evidence: EvidenceState::new(requirements.clone(), pending_checks.clone()),
        completion: CompletionState::new(requirements, pending_checks),
        recovery: RecoveryState::default(),
        document: None,
        transitions: Vec::new(),
        budgets: CaseBudgetState::default(),
    }
}

fn confidence_for(family: TaskFamily) -> u8 {
    match family {
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => 70,
        TaskFamily::Architecture => 75,
        TaskFamily::Verification => 65,
        TaskFamily::CodeChange => 55,
        _ => 60,
    }
}

fn constraints_from_objective(objective: &ObjectiveState) -> Vec<ConstraintRecord> {
    objective
        .owner_constraints
        .iter()
        .map(|summary| ConstraintRecord::hard(summary.clone(), "owner"))
        .collect()
}

fn initial_risks() -> Vec<RiskRecord> {
    vec![RiskRecord {
        summary: "first owner message is not a sufficient plan".to_string(),
        mitigation: "record graph.plan before mutating tools".to_string(),
        status: FieldStatus::Open,
    }]
}

fn packages_for(family: TaskFamily) -> Vec<String> {
    let mut packages = vec![
        "planning-checklist".to_string(),
        "context-slice".to_string(),
    ];
    if matches!(
        family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    ) {
        packages.push("doc-construction".to_string());
    }
    packages
}

fn requirements_for(family: TaskFamily) -> Vec<String> {
    let mut required = vec!["plan".to_string(), "observation".to_string()];
    if matches!(
        family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    ) {
        required.push("document-structure".to_string());
    } else {
        required.push("verification".to_string());
    }
    required
}

fn checks_for(family: TaskFamily) -> Vec<String> {
    if matches!(
        family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    ) {
        vec!["document audit".to_string()]
    } else {
        vec!["focused verification".to_string()]
    }
}
