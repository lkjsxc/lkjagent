use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;

use crate::maintenance::spend_cycle;
use crate::prompt::token_estimate;
use crate::step::frames::{append_notice, result};
use crate::step::{Effect, StepResult};
use crate::task::{spend_turn, RuntimeState, StopReason, TaskState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BudgetExhaustion {
    Task,
    Maintenance,
}

impl BudgetExhaustion {
    pub(super) fn notice(self) -> &'static str {
        match self {
            Self::Task => "turn budget exhausted",
            Self::Maintenance => "maintenance cycle turn budget exhausted",
        }
    }
}

pub(super) fn spend_active_budget(state: &mut RuntimeState) -> Option<BudgetExhaustion> {
    if matches!(state.task, TaskState::Open { .. }) {
        let (task, task_exhausted) = spend_turn(&state.task);
        state.task = task;
        if task_exhausted {
            state.maintenance = None;
            return Some(BudgetExhaustion::Task);
        }
        return None;
    }
    let (maintenance, maintenance_exhausted) = spend_cycle(&state.maintenance);
    state.maintenance = maintenance;
    if maintenance_exhausted {
        Some(BudgetExhaustion::Maintenance)
    } else {
        None
    }
}

pub(super) fn budget_exhausted_step(
    mut state: RuntimeState,
    exhausted: BudgetExhaustion,
) -> StepResult {
    let notice = exhausted.notice();
    state = append_notice(state, NoticeKind::Budget, notice);
    let effects = vec![Effect::RecordEvent {
        kind: EventKind::Notice,
        content: notice.to_string(),
        tokens: token_estimate(notice) as i64,
    }];
    if exhausted == BudgetExhaustion::Task {
        state.task = TaskState::Waiting {
            question: "Turn budget exhausted. Send guidance to continue.".to_string(),
        };
        return result(state, effects, Some(StopReason::Ask));
    }
    result(state, effects, Some(StopReason::BudgetNotice))
}
