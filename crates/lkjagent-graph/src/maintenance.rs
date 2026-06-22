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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceCycleObservation {
    pub found_only_known_lessons: bool,
    pub listed_same_queue_state: bool,
    pub pruned_rows: usize,
    pub changed_paths: Vec<String>,
    pub new_structural_findings: Vec<String>,
    pub repeated_action_signatures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuppressionRecord {
    pub key: String,
    pub reason: String,
    pub expires_after_cycles: u8,
}

pub fn no_op_suppression(observation: &MaintenanceCycleObservation) -> Option<SuppressionRecord> {
    if observation.found_only_known_lessons
        && observation.listed_same_queue_state
        && observation.pruned_rows == 0
        && observation.changed_paths.is_empty()
        && observation.new_structural_findings.is_empty()
    {
        Some(SuppressionRecord {
            key: "maintenance:distill:no-new-evidence".to_string(),
            reason: "maintenance cycle produced no new evidence or effect".to_string(),
            expires_after_cycles: 3,
        })
    } else {
        None
    }
}
