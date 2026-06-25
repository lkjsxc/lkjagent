use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_context::model::Frame;
use lkjagent_graph::TaskGraphState;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::maintenance::MaintenanceDirective;
use crate::task::PendingActionAuthority;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepInput {
    Owner {
        content: String,
        tokens: usize,
        graph: Option<Box<TaskGraphState>>,
        turn_budget: u16,
    },
    Completion {
        content: String,
        tokens: usize,
    },
    AuthorizedCompletion(String, usize, PendingActionAuthority),
    TurnBudgetCheckpoint,
    EndpointOversize {
        preview: String,
    },
    ProviderAnomaly(String, String),
    ToolOutput(DispatchOutput),
    Compact {
        prefix: Vec<Frame>,
        summary: Frame,
        memory_ids: Vec<i64>,
        policy: ContextBudgetPolicy,
    },
    StartMaintenance {
        directive: MaintenanceDirective,
        budget: u16,
    },
}
