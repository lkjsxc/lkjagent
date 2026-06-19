use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::{parse_completion, render_action, render_owner};
use lkjagent_store::events::EventKind;

use crate::maintenance::spend_cycle;
use crate::prompt::token_estimate;
use crate::recovery::{parse_notice, parse_recovery_notice, stop_reason};
use crate::step::frames::{append_notice, result};
use crate::step::{Effect, StepResult};
use crate::task::{open_task, spend_turn, PendingAction, RuntimeState, StopReason, TaskState};

pub(super) fn owner_step(mut state: RuntimeState, content: String, tokens: usize) -> StepResult {
    if state.maintenance.is_some() {
        state = append_notice(
            state,
            NoticeKind::Maintenance,
            "maintenance preempted by owner",
        );
        state.maintenance = None;
    }
    state.context = append_frame(
        &state.context,
        Frame::new(FrameKind::Owner, render_owner(&content), tokens),
    );
    state.task = open_task(&state.task);
    StepResult {
        state,
        effects: vec![Effect::RecordEvent {
            kind: EventKind::Owner,
            content,
            tokens: tokens as i64,
        }],
        stop_reason: None,
    }
}

pub(super) fn completion_step(
    mut state: RuntimeState,
    content: String,
    tokens: usize,
) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let exhausted = spend_active_budget(&mut state);
    state.context = append_frame(
        &state.context,
        Frame::new(FrameKind::ModelTurn, content.clone(), tokens),
    );
    if let Some(exhausted) = exhausted {
        state = append_notice(state, NoticeKind::Budget, exhausted.notice());
        return result(state, vec![], Some(StopReason::BudgetNotice));
    }
    match parse_completion(&content) {
        Ok(action) => action_step(state, action),
        Err(fault) => parse_fault_step(state, &fault),
    }
}

pub(super) fn endpoint_oversize_step(mut state: RuntimeState) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let exhausted = spend_active_budget(&mut state);
    if let Some(exhausted) = exhausted {
        state = append_notice(state, NoticeKind::Budget, exhausted.notice());
        return result(state, vec![], Some(StopReason::BudgetNotice));
    }
    let error = "endpoint completion hit max tokens";
    let recovery = "recovery: completion hit max tokens; emit one short valid act block next; use shell.run heredoc/script for large generated output";
    state = append_notice(state, NoticeKind::Error, error);
    state = append_notice(state, NoticeKind::Error, recovery);
    result(
        state,
        vec![
            Effect::RecordEvent {
                kind: EventKind::Error,
                content: error.to_string(),
                tokens: token_estimate(error) as i64,
            },
            Effect::RecordEvent {
                kind: EventKind::Notice,
                content: recovery.to_string(),
                tokens: token_estimate(recovery) as i64,
            },
        ],
        Some(StopReason::InvalidAction),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BudgetExhaustion {
    Task,
    Maintenance,
}

impl BudgetExhaustion {
    fn notice(self) -> &'static str {
        match self {
            Self::Task => "turn budget exhausted",
            Self::Maintenance => "maintenance cycle turn budget exhausted",
        }
    }
}

fn spend_active_budget(state: &mut RuntimeState) -> Option<BudgetExhaustion> {
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

fn action_step(mut state: RuntimeState, action: lkjagent_protocol::Action) -> StepResult {
    let action_text = render_action(&action);
    state.pending_action = Some(PendingAction {
        action,
        action_text: action_text.clone(),
    });
    state.parse_faults = 0;
    result(
        state,
        vec![
            Effect::RecordEvent {
                kind: EventKind::Action,
                content: action_text.clone(),
                tokens: token_estimate(&action_text) as i64,
            },
            Effect::ExecuteTool { action_text },
        ],
        Some(StopReason::Acted),
    )
}

fn parse_fault_step(mut state: RuntimeState, fault: &lkjagent_protocol::ParseFault) -> StepResult {
    let notice = parse_notice(fault);
    state.parse_faults = state.parse_faults.saturating_add(1);
    state = append_notice(state, NoticeKind::Error, &notice);
    let recovery = parse_recovery_notice(state.parse_faults);
    state = append_notice(state, NoticeKind::Error, &recovery);
    let mut effects = vec![Effect::RecordEvent {
        kind: EventKind::Error,
        content: notice,
        tokens: 32,
    }];
    effects.push(Effect::RecordEvent {
        kind: EventKind::Notice,
        content: recovery,
        tokens: 32,
    });
    result(state, effects, Some(stop_reason(fault)))
}
