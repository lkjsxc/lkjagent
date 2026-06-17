pub const WINDOW_TOKENS: usize = 32_768;
pub const GENERATION_RESERVE: usize = 1_024;
pub const PREFIX_IDENTITY: usize = 768;
pub const PREFIX_GRAMMAR_REGISTRY: usize = 1_024;
pub const PREFIX_SKILL_INDEX: usize = 512;
pub const PREFIX_WORKSPACE_BRIEF: usize = 1_024;
pub const PREFIX_MEMORY_DIGEST: usize = 2_048;
pub const LOG_OWNER_FRAME: usize = 4_096;
pub const LOG_OBSERVATION: usize = 2_048;
pub const LOG_SKILL_BODY: usize = 2_048;
pub const LOG_LOADED_SKILLS: usize = 6_144;
pub const WHOLE_WINDOW_TRIGGER: usize = 28_672;
pub const POST_COMPACTION_TARGET: usize = 8_192;
pub const MIN_LOG_SPACE: usize = 16_384;

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
        region: "prefix: skill index",
        cap: PREFIX_SKILL_INDEX,
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
        region: "log: skill body",
        cap: LOG_SKILL_BODY,
    },
    BudgetRow {
        region: "log: loaded skills concurrent",
        cap: LOG_LOADED_SKILLS,
    },
    BudgetRow {
        region: "whole window trigger",
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
        + PREFIX_SKILL_INDEX
        + PREFIX_WORKSPACE_BRIEF
        + PREFIX_MEMORY_DIGEST
}

pub fn initial_log_space() -> usize {
    WINDOW_TOKENS - GENERATION_RESERVE - prefix_cap_total()
}

pub fn config_preserves_log_floor(window: usize, prefix_total: usize, reserve: usize) -> bool {
    window.saturating_sub(prefix_total).saturating_sub(reserve) >= MIN_LOG_SPACE
}
