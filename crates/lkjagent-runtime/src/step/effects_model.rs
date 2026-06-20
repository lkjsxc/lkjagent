use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_store::events::EventKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    RecordEvent {
        kind: EventKind,
        content: String,
        tokens: i64,
    },
    ExecuteTool {
        action_text: String,
    },
    DistillTask {
        summary: String,
        prompt: String,
        max_turns: u8,
    },
    RecordGraphEvidence {
        case_id: i64,
        requirement: String,
        kind: String,
        summary: String,
        path: Option<String>,
    },
    RecordGraphPlan {
        case_id: i64,
        steps: Vec<GraphPlanStepEffect>,
    },
    RecordGraphContext {
        case_id: i64,
        packages: Vec<String>,
        reason: String,
    },
    RecordGraphNote {
        case_id: i64,
        kind: String,
        summary: String,
    },
    RecordGraphTransition {
        case_id: i64,
        from_node: String,
        to_node: String,
        decision: String,
        reason: String,
    },
    RecordGraphFault {
        case_id: i64,
        kind: String,
        action_fingerprint: Option<String>,
        summary: String,
        count: u8,
    },
    UpdateGraphRecovery {
        case_id: i64,
        ladder_position: u8,
        strategy: String,
    },
    UpdateGraphCase {
        case_id: i64,
        phase: String,
        active_node: String,
        status: String,
    },
    Pause {
        reason: String,
    },
    CompactionRecorded {
        before_tokens: usize,
        after_tokens: usize,
        memory_ids: Vec<i64>,
        policy: ContextBudgetPolicy,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphPlanStepEffect {
    pub step_id: String,
    pub title: String,
    pub rationale: String,
    pub status: String,
    pub node: String,
    pub target_paths: Vec<String>,
    pub checks: Vec<String>,
}
