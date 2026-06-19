use lkjagent_context::model::ContextState;
use lkjagent_graph::TaskGraphState;
use lkjagent_protocol::Action;

use crate::maintenance::MaintenanceCycle;

pub const DEFAULT_TURN_BUDGET: u16 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Idle,
    Open { turns_remaining: u16 },
    Waiting { question: String },
    Closed { summary: String },
    Paused { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingAction {
    pub action: Action,
    pub action_text: String,
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
    pub turn: i64,
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
            turn: 0,
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
    match task {
        TaskState::Idle | TaskState::Closed { .. } | TaskState::Waiting { .. } => TaskState::Open {
            turns_remaining: DEFAULT_TURN_BUDGET,
        },
        TaskState::Open { turns_remaining: 0 } => TaskState::Open {
            turns_remaining: DEFAULT_TURN_BUDGET,
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
