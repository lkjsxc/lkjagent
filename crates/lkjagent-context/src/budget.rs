pub const DEFAULT_WINDOW_TOKENS: usize = 24_576;
pub const MIN_CONTEXT_WINDOW: usize = 16_384;
pub const DEFAULT_GENERATION_RESERVE: usize = 2_048;
pub const WINDOW_TOKENS: usize = DEFAULT_WINDOW_TOKENS;
pub const GENERATION_RESERVE: usize = DEFAULT_GENERATION_RESERVE;
pub const PREFIX_IDENTITY: usize = 768;
pub const PREFIX_GRAMMAR_REGISTRY: usize = 1_024;
pub const PREFIX_GRAPH_STATE: usize = 512;
pub const PREFIX_WORKSPACE_BRIEF: usize = 1_024;
pub const PREFIX_MEMORY_DIGEST: usize = 2_048;
pub const LOG_OWNER_FRAME: usize = 4_096;
pub const LOG_OBSERVATION: usize = 2_048;
pub const LOG_GRAPH_NOTICE: usize = 2_048;
pub const DEFAULT_SOFT_TRIGGER: usize = 18_432;
pub const DEFAULT_HARD_TRIGGER: usize = 21_504;
pub const DEFAULT_POST_COMPACTION_TARGET: usize = 8_192;
pub const WHOLE_WINDOW_TRIGGER: usize = DEFAULT_HARD_TRIGGER;
pub const POST_COMPACTION_TARGET: usize = DEFAULT_POST_COMPACTION_TARGET;
pub const MIN_LOG_SPACE: usize = 4_096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetRow {
    pub region: &'static str,
    pub cap: usize,
}

pub const BUDGET_ROWS: &[BudgetRow] = &[
    BudgetRow {
        region: "generation reserve",
        cap: GENERATION_RESERVE,
    },
    BudgetRow {
        region: "prefix: identity and rules",
        cap: PREFIX_IDENTITY,
    },
    BudgetRow {
        region: "prefix: grammar and tool registry",
        cap: PREFIX_GRAMMAR_REGISTRY,
    },
    BudgetRow {
        region: "prefix: graph state",
        cap: PREFIX_GRAPH_STATE,
    },
    BudgetRow {
        region: "prefix: workspace brief",
        cap: PREFIX_WORKSPACE_BRIEF,
    },
    BudgetRow {
        region: "prefix: memory digest",
        cap: PREFIX_MEMORY_DIGEST,
    },
    BudgetRow {
        region: "log: owner frame",
        cap: LOG_OWNER_FRAME,
    },
    BudgetRow {
        region: "log: observation",
        cap: LOG_OBSERVATION,
    },
    BudgetRow {
        region: "log: graph notice",
        cap: LOG_GRAPH_NOTICE,
    },
    BudgetRow {
        region: "soft compaction trigger",
        cap: DEFAULT_SOFT_TRIGGER,
    },
    BudgetRow {
        region: "hard compaction trigger",
        cap: WHOLE_WINDOW_TRIGGER,
    },
    BudgetRow {
        region: "post-compaction target",
        cap: POST_COMPACTION_TARGET,
    },
];

pub fn prefix_cap_total() -> usize {
    PREFIX_IDENTITY
        + PREFIX_GRAMMAR_REGISTRY
        + PREFIX_GRAPH_STATE
        + PREFIX_WORKSPACE_BRIEF
        + PREFIX_MEMORY_DIGEST
}

pub fn initial_log_space() -> usize {
    ContextBudgetPolicy::default().available_log_space()
}

pub fn config_preserves_log_floor(window: usize, prefix_total: usize, reserve: usize) -> bool {
    window.saturating_sub(prefix_total).saturating_sub(reserve) >= MIN_LOG_SPACE
}
#[path = "budget_policy.rs"]
mod policy;

pub use policy::{budget_rows_for, ContextBudgetError, ContextBudgetPolicy, ContextPressure};
