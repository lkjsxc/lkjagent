use lkjagent_graph::{CaseStatus, EvidenceKind, GraphNodeId, TaskFamily, TaskPhase};

pub(crate) fn family(value: &str) -> TaskFamily {
    match value {
        "documentation" => TaskFamily::Documentation,
        "maintenance" => TaskFamily::Maintenance,
        "bug-fix" => TaskFamily::BugFix,
        "architecture" => TaskFamily::Architecture,
        "benchmark" => TaskFamily::Benchmark,
        "knowledge-base" => TaskFamily::KnowledgeBase,
        "verification" => TaskFamily::Verification,
        "recovery" => TaskFamily::Recovery,
        "compaction" => TaskFamily::Compaction,
        "idle-maintenance" => TaskFamily::IdleMaintenance,
        _ => TaskFamily::CodeChange,
    }
}

pub(crate) fn phase(value: &str) -> TaskPhase {
    match value {
        "planning" => TaskPhase::Planning,
        "context" => TaskPhase::Context,
        "execution" => TaskPhase::Execution,
        "verification" => TaskPhase::Verification,
        "recovery" => TaskPhase::Recovery,
        "compaction" => TaskPhase::Compaction,
        "completion" => TaskPhase::Completion,
        "maintenance" => TaskPhase::Maintenance,
        "waiting" => TaskPhase::Waiting,
        "closed" => TaskPhase::Closed,
        _ => TaskPhase::Intake,
    }
}

pub(crate) fn status(value: &str) -> CaseStatus {
    match value {
        "waiting" => CaseStatus::Waiting,
        "closed" => CaseStatus::Closed,
        "paused" => CaseStatus::Paused,
        _ => CaseStatus::Active,
    }
}

pub(crate) fn evidence_kind(value: &str) -> EvidenceKind {
    match value {
        "owner" => EvidenceKind::Owner,
        "plan" => EvidenceKind::Plan,
        "action" => EvidenceKind::Action,
        "verification" => EvidenceKind::Verification,
        "file" => EvidenceKind::File,
        "memory" => EvidenceKind::Memory,
        "note" => EvidenceKind::Note,
        _ => EvidenceKind::Observation,
    }
}

pub(crate) fn node_id(value: &str) -> GraphNodeId {
    match value {
        "intake" => GraphNodeId("intake"),
        "normalize-objective" => GraphNodeId("normalize-objective"),
        "extract-constraints" => GraphNodeId("extract-constraints"),
        "route" => GraphNodeId("route"),
        "survey" => GraphNodeId("survey"),
        "context" => GraphNodeId("context"),
        "review-plan" => GraphNodeId("review-plan"),
        "execute" => GraphNodeId("execute"),
        "observe" => GraphNodeId("observe"),
        "integrate-evidence" => GraphNodeId("integrate-evidence"),
        "verify" => GraphNodeId("verify"),
        "escape" => GraphNodeId("escape"),
        "recover" => GraphNodeId("recover"),
        "recover-parse" => GraphNodeId("recover-parse"),
        "recover-tool" => GraphNodeId("recover-tool"),
        "recover-repeat" => GraphNodeId("recover-repeat"),
        "compact-soft" => GraphNodeId("compact-soft"),
        "compact-hard" => GraphNodeId("compact-hard"),
        "rebuild-context" => GraphNodeId("rebuild-context"),
        "complete" => GraphNodeId("complete"),
        "document" => GraphNodeId("document"),
        "document-audit" => GraphNodeId("document-audit"),
        "benchmark" => GraphNodeId("benchmark"),
        "docs-code-consistency" => GraphNodeId("docs-code-consistency"),
        "memory" => GraphNodeId("memory"),
        "maintain" => GraphNodeId("maintain"),
        "refine-graph-policy" => GraphNodeId("refine-graph-policy"),
        "prune-memory" => GraphNodeId("prune-memory"),
        "audit-self" => GraphNodeId("audit-self"),
        _ => GraphNodeId("plan"),
    }
}
