use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_graph::TaskGraphState;
use lkjagent_protocol::{parse_completion, render_action, render_owner};
use lkjagent_store::events::EventKind;

use crate::graph_state::graph_notice_frame;
use crate::prompt::token_estimate;
use crate::recovery::{parse_notice, parse_recovery_notice, should_escalate, stop_reason};
use crate::step::budget::{budget_exhausted_step, spend_active_budget};
use crate::step::fault_wait::{enter_recovery_route, record_recoverable_fault, RecoveryFault};
use crate::step::frames::{append_notice, result};
use crate::step::oversize::{oversize_error, oversize_recovery};
use crate::step::{Effect, StepResult};
use crate::task::{open_task_with_budget, PendingAction, RuntimeState, StopReason};

pub(super) fn owner_step(
    mut state: RuntimeState,
    content: String,
    tokens: usize,
    graph: Option<TaskGraphState>,
    turn_budget: u16,
) -> StepResult {
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
    if let Some(graph) = graph {
        state.context = append_frame(&state.context, graph_notice_frame(&graph));
        state.graph = Some(graph);
    }
    state.parse_faults = 0;
    state.repeat_faults = 0;
    state.tool_faults = 0;
    state.continuation_epoch.checkpoint_turns = turn_budget.max(1);
    state.task = open_task_with_budget(&state.task, turn_budget);
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
        return budget_exhausted_step(state, exhausted);
    }
    match parse_completion(&content) {
        Ok(action) => action_step(state, action),
        Err(fault) => parse_fault_step(state, &fault),
    }
}

pub(super) fn endpoint_oversize_step(mut state: RuntimeState, preview: &str) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let exhausted = spend_active_budget(&mut state);
    if let Some(exhausted) = exhausted {
        state = append_notice(state, NoticeKind::Budget, exhausted.notice());
        return result(state, vec![], Some(StopReason::BudgetNotice));
    }
    let error = oversize_error(preview);
    let recovery = oversize_recovery(preview);
    state = append_notice(state, NoticeKind::Error, &error);
    state = append_notice(state, NoticeKind::Error, &recovery);
    let mut effects = vec![
        Effect::RecordEvent {
            kind: EventKind::Error,
            content: error.clone(),
            tokens: token_estimate(&error) as i64,
        },
        Effect::RecordEvent {
            kind: EventKind::Notice,
            content: recovery.clone(),
            tokens: token_estimate(&recovery) as i64,
        },
    ];
    if payload_risk(preview) {
        state.parse_faults = state.parse_faults.saturating_add(1);
        let count = state.parse_faults;
        let fault = RecoveryFault::Payload;
        record_recoverable_fault(&mut state, fault, count, None, &recovery, &mut effects);
        state = enter_recovery_route(state, fault, count, None, &mut effects);
    }
    result(state, effects, Some(StopReason::InvalidAction))
}

fn payload_risk(preview: &str) -> bool {
    preview.contains("<tool>fs.write</tool>") || preview.contains("<content>")
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
    let recovery = parse_recovery_notice(fault, state.parse_faults);
    state = append_notice(state, NoticeKind::Error, &recovery);
    let mut effects = vec![Effect::RecordEvent {
        kind: EventKind::Error,
        content: notice,
        tokens: 32,
    }];
    effects.push(Effect::RecordEvent {
        kind: EventKind::Notice,
        content: recovery.clone(),
        tokens: 32,
    });
    let count = state.parse_faults;
    record_recoverable_fault(
        &mut state,
        recovery_fault(fault),
        count,
        None,
        &recovery,
        &mut effects,
    );
    if should_escalate(state.parse_faults) {
        let count = state.parse_faults;
        state = enter_recovery_route(state, recovery_fault(fault), count, None, &mut effects);
    }
    result(state, effects, Some(stop_reason(fault)))
}

fn recovery_fault(fault: &lkjagent_protocol::ParseFault) -> RecoveryFault {
    match fault {
        lkjagent_protocol::ParseFault::BadParams { .. }
        | lkjagent_protocol::ParseFault::DuplicateParam { .. } => RecoveryFault::Params,
        _ => RecoveryFault::Parse,
    }
}
