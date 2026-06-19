#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceDirective {
    Distill,
    ImproveGraph,
    PruneMemory,
    AuditSelf,
}

impl MaintenanceDirective {
    pub fn all() -> &'static [Self] {
        &[
            Self::Distill,
            Self::ImproveGraph,
            Self::PruneMemory,
            Self::AuditSelf,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Distill => "distill",
            Self::ImproveGraph => "improve-graph",
            Self::PruneMemory => "prune-memory",
            Self::AuditSelf => "audit-self",
        }
    }

    pub fn work(self) -> &'static str {
        match self {
            Self::Distill => "save reusable lessons, facts, task summaries, or incidents",
            Self::ImproveGraph => "improve graph patterns, context packages, or evidence rules",
            Self::PruneMemory => "merge, correct, or delete stale memory rows",
            Self::AuditSelf => "record mismatches between docs, code, graph state, and tests",
        }
    }
}
