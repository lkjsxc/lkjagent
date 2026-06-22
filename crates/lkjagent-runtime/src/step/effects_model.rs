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
        node: String,
        tool: String,
        parameter_shape: String,
        fault_class: String,
        action_fingerprint: Option<String>,
        summary: String,
        count: u8,
    },
    UpdateGraphRecovery {
        case_id: i64,
        ladder_position: u8,
        strategy: String,
    },
    ReplaceGraphStateTracks {
        case_id: i64,
        tracks: Vec<GraphStateTrackEffect>,
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
    DeferMaintenance,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphStateTrackEffect {
    pub track_id: String,
    pub label: String,
    pub posture: String,
    pub intensity: u8,
    pub confidence: u8,
    pub phase: String,
    pub active_node: String,
    pub evidence_gap: Vec<String>,
    pub next_affordances: Vec<String>,
    pub risk: Vec<String>,
    pub last_update_turn: Option<u64>,
    pub rank_score: u8,
}
