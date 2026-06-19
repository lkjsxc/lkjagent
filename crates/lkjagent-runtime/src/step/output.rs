use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{ContextState, Frame, FrameKind, NoticeKind};
use lkjagent_protocol::render_notice;
use lkjagent_protocol::Action;
use lkjagent_store::events::EventKind;
use lkjagent_tools::dispatch::DispatchOutput;
use lkjagent_tools::observe::OutputKind;

use crate::maintenance::task_distillation_prompt;
use crate::prompt::token_estimate;
use crate::step::Effect;
use crate::task::{PendingAction, RuntimeState, StopReason, TaskState};

pub(super) fn append_output_frame(context: &ContextState, output: &DispatchOutput) -> ContextState {
    let kind = match output.kind {
        OutputKind::Observation { .. } => FrameKind::Observation,
        OutputKind::Notice { .. } => FrameKind::Notice(NoticeKind::Error),
        OutputKind::Skill { .. } => FrameKind::SkillBody,
    };
    append_frame(
        context,
        Frame::new(
            kind,
            output.rendered.clone(),
            token_estimate(&output.rendered),
        ),
    )
}

pub(super) fn event_kind(kind: &OutputKind) -> EventKind {
    match kind {
        OutputKind::Observation { .. } => EventKind::Observation,
        OutputKind::Notice { .. } => EventKind::Notice,
        OutputKind::Skill { .. } => EventKind::Observation,
    }
}

pub(super) fn stop_for_output(output: &DispatchOutput) -> StopReason {
    match &output.kind {
        OutputKind::Notice { .. } if output.content.contains("repeat action refused") => {
            StopReason::RepeatAction
        }
        OutputKind::Observation { status } if status == "error" => StopReason::ToolError,
        _ => StopReason::Acted,
    }
}

pub(super) fn handle_control_success(
    state: &mut RuntimeState,
    pending: &PendingAction,
    output: &DispatchOutput,
    effects: &mut Vec<Effect>,
) -> Option<StopReason> {
    if !matches!(&output.kind, OutputKind::Observation { status } if status == "ok") {
        return None;
    }
    match pending.action.tool.as_str() {
        "agent.done" => close_work(state, &pending.action, effects),
        "agent.ask" => wait_for_owner(state, &pending.action),
        _ => None,
    }
}

fn close_work(
    state: &mut RuntimeState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> Option<StopReason> {
    if state.maintenance.is_some() {
        state.maintenance = None;
        return Some(StopReason::Done);
    }
    close_task(state, action, effects)
}

fn close_task(
    state: &mut RuntimeState,
    action: &Action,
    effects: &mut Vec<Effect>,
) -> Option<StopReason> {
    let summary = action_param(action, "summary");
    state.task = TaskState::Closed {
        summary: summary.clone(),
    };
    let prompt = task_distillation_prompt(&summary);
    append_distillation_notice(state, &prompt);
    effects.push(Effect::DistillTask {
        summary,
        prompt,
        max_turns: 2,
    });
    Some(StopReason::Done)
}

fn append_distillation_notice(state: &mut RuntimeState, prompt: &str) {
    state.context = append_frame(
        &state.context,
        Frame::new(
            FrameKind::Notice(NoticeKind::Maintenance),
            render_notice("maintenance", prompt),
            token_estimate(prompt).saturating_add(8),
        ),
    );
}

fn wait_for_owner(state: &mut RuntimeState, action: &Action) -> Option<StopReason> {
    if state.maintenance.is_some() {
        state.maintenance = None;
        state.task = TaskState::Idle;
        return Some(StopReason::Done);
    }
    state.maintenance = None;
    state.task = TaskState::Waiting {
        question: action_param(action, "question"),
    };
    Some(StopReason::Ask)
}

fn action_param(action: &Action, name: &str) -> String {
    action
        .params
        .iter()
        .find(|param| param.name == name)
        .map_or_else(String::new, |param| param.value.clone())
}
