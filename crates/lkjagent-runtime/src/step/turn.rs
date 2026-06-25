use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_graph::TaskGraphState;
use lkjagent_protocol::{
    parse_live_completion, render_action, render_owner, EnvelopeMode, ParseFault,
};
use lkjagent_store::events::EventKind;

use crate::graph_state::graph_notice_frame;
use crate::prompt::token_estimate;
use crate::recovery::{parse_notice, parse_recovery_notice, should_escalate, stop_reason};
use crate::step::budget::{budget_exhausted_step, spend_active_budget};
use crate::step::fault_wait::{enter_recovery_route, record_recoverable_fault, RecoveryFault};
use crate::step::frames::{append_notice, result};
use crate::step::owner_guidance::apply_owner_guidance;
use crate::step::{Effect, StepResult};
use crate::task::{
    open_task_with_budget, PendingAction, PendingActionAuthority, RuntimeState, StopReason,
};

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
    apply_owner_guidance(&mut state, &content);
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
    authority: Option<PendingActionAuthority>,
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
    let outcome = parse_live_completion(&content, Default::default());
    let normalization = implicit_notice(&outcome);
    match (outcome.action, outcome.fault) {
        (Some(action), None) => action_step(state, action, normalization, authority),
        (_, Some(fault)) => parse_fault_step(state, &fault),
        (None, None) => parse_fault_step(state, &ParseFault::MissingActionEnvelope),
    }
}

fn action_step(
    mut state: RuntimeState,
    action: lkjagent_protocol::Action,
    normalization: Option<String>,
    authority: Option<PendingActionAuthority>,
) -> StepResult {
    let action_text = render_action(&action);
    let authority = authority.unwrap_or_else(PendingActionAuthority::empty);
    state.pending_action = Some(PendingAction {
        action,
        action_text: action_text.clone(),
        authority_decision_id: authority.authority_decision_id,
        prompt_frame_id: authority.prompt_frame_id,
        staleness_fingerprint: authority.staleness_fingerprint,
    });
    state.parse_faults = 0;
    let mut effects = Vec::new();
    if let Some(notice) = normalization {
        state = append_notice(state, NoticeKind::Error, &notice);
        effects.push(Effect::RecordEvent {
            kind: EventKind::Notice,
            tokens: token_estimate(&notice) as i64,
            content: notice,
        });
    }
    effects.push(Effect::RecordEvent {
        kind: EventKind::Action,
        content: action_text.clone(),
        tokens: token_estimate(&action_text) as i64,
    });
    effects.push(Effect::ExecuteTool { action_text });
    result(state, effects, Some(StopReason::Acted))
}

fn implicit_notice(outcome: &lkjagent_protocol::ParseOutcome) -> Option<String> {
    (outcome.envelope_mode == EnvelopeMode::Implicit).then(|| {
        format!(
            "parse normalization recorded\nenvelope_mode=ImplicitActionEnvelope\nnormalized_text_hash={}",
            outcome.normalized_text_hash
        )
    })
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
