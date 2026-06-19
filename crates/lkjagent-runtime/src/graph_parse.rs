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
        "classify" => GraphNodeId("classify"),
        "context" => GraphNodeId("context"),
        "execute" => GraphNodeId("execute"),
        "verify" => GraphNodeId("verify"),
        "recover" => GraphNodeId("recover"),
        "compact" => GraphNodeId("compact"),
        "complete" => GraphNodeId("complete"),
        "document" => GraphNodeId("document"),
        "memory" => GraphNodeId("memory"),
        "maintain" => GraphNodeId("maintain"),
        _ => GraphNodeId("plan"),
    }
}
