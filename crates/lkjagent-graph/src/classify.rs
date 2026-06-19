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
    TaskGraphState {
        case_id,
        objective: objective.to_string(),
        family,
        phase: TaskPhase::Planning,
        status: CaseStatus::Active,
        active_node: GraphNodeId("plan"),
        confidence: confidence_for(family),
        plan: plan_for(objective, family),
        risks: vec!["first owner message is not a sufficient plan".to_string()],
        candidate_paths: Vec::new(),
        touched_paths: Vec::new(),
        selected_packages: packages_for(family),
        evidence_requirements: requirements_for(family),
        evidence: Vec::new(),
        pending_checks: vec!["focused verification".to_string()],
        recovery: None,
    }
}

fn confidence_for(family: TaskFamily) -> u8 {
    match family {
        TaskFamily::CodeChange => 55,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => 70,
        TaskFamily::Architecture => 75,
        TaskFamily::Verification => 65,
        _ => 60,
    }
}

fn plan_for(objective: &str, family: TaskFamily) -> String {
    format!(
        "objective={objective}\nfamily={}\nnext=inspect relevant files before editing",
        family.as_str()
    )
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
        TaskFamily::CodeChange
            | TaskFamily::BugFix
            | TaskFamily::Architecture
            | TaskFamily::Benchmark
            | TaskFamily::Verification
    ) {
        required.push("verification".to_string());
    }
    if matches!(
        family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    ) {
        required.push("document-structure".to_string());
    }
    required
}
