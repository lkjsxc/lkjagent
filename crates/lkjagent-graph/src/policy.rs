#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextPressureLevel {
    Green,
    Yellow,
    Orange,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanningPolicy {
    pub required: bool,
    pub min_steps: usize,
    pub needs_checks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPolicy {
    pub max_parse_faults: u8,
    pub max_tool_faults: u8,
    pub max_repeat_faults: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionPolicy {
    pub requires_plan: bool,
    pub requires_observation: bool,
    pub requires_verification_for_code: bool,
    pub requires_document_audit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphPolicy {
    pub planning: PlanningPolicy,
    pub recovery: RecoveryPolicy,
    pub completion: CompletionPolicy,
    pub shell_allowed_nodes: &'static [&'static str],
    pub max_batch_files: usize,
    pub compaction_soft: ContextPressureLevel,
    pub compaction_hard: ContextPressureLevel,
    pub maintenance_cadence: &'static [&'static str],
    pub document_default_root: &'static str,
    pub compaction_preserve: &'static [&'static str],
}

pub const DEFAULT_POLICY: GraphPolicy = GraphPolicy {
    planning: PlanningPolicy {
        required: true,
        min_steps: 1,
        needs_checks: true,
    },
    recovery: RecoveryPolicy {
        max_parse_faults: 3,
        max_tool_faults: 3,
        max_repeat_faults: 3,
    },
    completion: CompletionPolicy {
        requires_plan: true,
        requires_observation: true,
        requires_verification_for_code: true,
        requires_document_audit: true,
    },
    shell_allowed_nodes: &["verify", "escape", "recover-tool"],
    max_batch_files: 20,
    compaction_soft: ContextPressureLevel::Orange,
    compaction_hard: ContextPressureLevel::Red,
    maintenance_cadence: &[
        "distill",
        "refine-graph-policy",
        "prune-memory",
        "audit-self",
    ],
    document_default_root: "structured-output",
    compaction_preserve: &[
        "objective",
        "constraints",
        "plan-steps",
        "context-packages",
        "evidence",
        "touched-paths",
        "recovery",
        "completion",
    ],
};
