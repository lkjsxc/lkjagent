use lkjagent_context::model::ContextState;
use lkjagent_graph::TaskGraphState;
use lkjagent_protocol::Action;

use crate::maintenance::MaintenanceCycle;

pub const DEFAULT_TURN_BUDGET: u16 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationEpoch {
    pub epoch_index: u64,
    pub turns_used: u16,
    pub checkpoint_turns: u16,
    pub last_checkpoint_reason: Option<String>,
    pub continuation_decision: Option<String>,
    pub no_progress_count: u16,
}

impl ContinuationEpoch {
    pub fn new(checkpoint_turns: u16) -> Self {
        Self {
            epoch_index: 0,
            turns_used: 0,
            checkpoint_turns: checkpoint_turns.max(1),
            last_checkpoint_reason: None,
            continuation_decision: None,
            no_progress_count: 0,
        }
    }

    pub fn open_next(&mut self, reason: &str, decision: &str) {
        self.epoch_index = self.epoch_index.saturating_add(1);
        self.turns_used = 0;
        self.last_checkpoint_reason = Some(reason.to_string());
        self.continuation_decision = Some(decision.to_string());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Idle,
    Open { turns_remaining: u16 },
    Waiting { question: String },
    Closed { summary: String },
    Paused { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingActionAuthority {
    pub authority_decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
    pub staleness_fingerprint: Option<String>,
}

impl PendingActionAuthority {
    pub fn empty() -> Self {
        Self {
            authority_decision_id: None,
            prompt_frame_id: None,
            staleness_fingerprint: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingAction {
    pub action: Action,
    pub action_text: String,
    pub authority_decision_id: Option<String>,
    pub prompt_frame_id: Option<String>,
    pub staleness_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionCycle {
    pub before_tokens: usize,
    pub turns_remaining: u8,
    pub task_summary_required: bool,
    pub task_summary_saved: bool,
    pub memory_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeState {
    pub context: ContextState,
    pub task: TaskState,
    pub graph: Option<TaskGraphState>,
    pub maintenance: Option<MaintenanceCycle>,
    pub compaction: Option<CompactionCycle>,
    pub pending_action: Option<PendingAction>,
    pub parse_faults: u8,
    pub repeat_faults: u8,
    pub tool_faults: u8,
    pub turn: i64,
    pub continuation_epoch: ContinuationEpoch,
}

impl RuntimeState {
    pub fn new(context: ContextState) -> Self {
        Self {
            context,
            task: TaskState::Idle,
            graph: None,
            maintenance: None,
            compaction: None,
            pending_action: None,
            parse_faults: 0,
            repeat_faults: 0,
            tool_faults: 0,
            turn: 0,
            continuation_epoch: ContinuationEpoch::new(DEFAULT_TURN_BUDGET),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    Acted,
    Done,
    Ask,
    InvalidAction,
    UnknownTool,
    BadParams,
    RepeatAction,
    EndpointError,
    ToolError,
    BudgetNotice,
    Compaction,
    Maintenance,
}

pub fn open_task(task: &TaskState) -> TaskState {
    open_task_with_budget(task, DEFAULT_TURN_BUDGET)
}

pub fn open_task_with_budget(task: &TaskState, turn_budget: u16) -> TaskState {
    let budget = turn_budget.max(1);
    match task {
        TaskState::Idle | TaskState::Closed { .. } | TaskState::Waiting { .. } => TaskState::Open {
            turns_remaining: budget,
        },
        TaskState::Open { turns_remaining: 0 } => TaskState::Open {
            turns_remaining: budget,
        },
        current => current.clone(),
    }
}

pub fn spend_turn(task: &TaskState) -> (TaskState, bool) {
    match task {
        TaskState::Open { turns_remaining } if *turns_remaining > 1 => (
            TaskState::Open {
                turns_remaining: turns_remaining.saturating_sub(1),
            },
            false,
        ),
        TaskState::Open { turns_remaining: 1 } => (TaskState::Open { turns_remaining: 0 }, false),
        TaskState::Open { .. } => (TaskState::Open { turns_remaining: 0 }, true),
        current => (current.clone(), false),
    }
}
