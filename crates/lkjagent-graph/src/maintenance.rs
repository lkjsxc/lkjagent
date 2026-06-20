#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceDirective {
    Distill,
    RefineGraphPolicy,
    PruneMemory,
    AuditSelf,
}

impl MaintenanceDirective {
    pub fn all() -> &'static [Self] {
        &[
            Self::Distill,
            Self::RefineGraphPolicy,
            Self::PruneMemory,
            Self::AuditSelf,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Distill => "distill",
            Self::RefineGraphPolicy => "refine-graph-policy",
            Self::PruneMemory => "prune-memory",
            Self::AuditSelf => "audit-self",
        }
    }

    pub fn work(self) -> &'static str {
        match self {
            Self::Distill => "save reusable lessons, facts, task summaries, or incidents",
            Self::RefineGraphPolicy => "record graph policy and context package candidates",
            Self::PruneMemory => "merge, correct, or delete stale memory rows",
            Self::AuditSelf => "record mismatches between docs, code, graph state, and tests",
        }
    }
}
