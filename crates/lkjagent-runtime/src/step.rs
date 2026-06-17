use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::{parse_completion, render_action, render_owner};
use lkjagent_store::events::EventKind;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::prompt::token_estimate;
use crate::recovery::{parse_notice, should_escalate, stop_reason};
use crate::task::{open_task, spend_turn, PendingAction, RuntimeState, StopReason, TaskState};

mod compact;
mod frames;
mod output;

use compact::compact_step;
use frames::{append_notice, result};
use output::{append_output_frame, event_kind, handle_control_success, stop_for_output};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepInput {
    Owner {
        content: String,
        tokens: usize,
    },
    Completion {
        content: String,
        tokens: usize,
    },
    ToolOutput(DispatchOutput),
    Compact {
        prefix: Vec<Frame>,
        summary: Frame,
        memory_ids: Vec<i64>,
    },
}

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
        max_turns: u8,
    },
    Pause {
        reason: String,
    },
    CompactionRecorded {
        before_tokens: usize,
        after_tokens: usize,
        memory_ids: Vec<i64>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepResult {
    pub state: RuntimeState,
    pub effects: Vec<Effect>,
    pub stop_reason: Option<StopReason>,
}

pub fn step(state: RuntimeState, input: StepInput) -> StepResult {
    match input {
        StepInput::Owner { content, tokens } => owner_step(state, content, tokens),
        StepInput::Completion { content, tokens } => completion_step(state, content, tokens),
        StepInput::ToolOutput(output) => tool_output_step(state, output),
        StepInput::Compact {
            prefix,
            summary,
            memory_ids,
        } => compact_step(state, prefix, summary, memory_ids),
    }
}

fn owner_step(mut state: RuntimeState, content: String, tokens: usize) -> StepResult {
    if let TaskState::Waiting { question } = &state.task {
        let notice = format!("answering outstanding question: {question}");
        state = append_notice(state, NoticeKind::Delivery, &notice);
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

fn completion_step(mut state: RuntimeState, content: String, tokens: usize) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let (task, exhausted) = spend_turn(&state.task);
    state.task = task;
    state.context = append_frame(
        &state.context,
        Frame::new(FrameKind::ModelTurn, content.clone(), tokens),
    );
    if exhausted {
        state = append_notice(state, NoticeKind::Budget, "turn budget exhausted");
        return result(state, vec![], Some(StopReason::BudgetNotice));
    }
    match parse_completion(&content) {
        Ok(action) => {
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
        Err(fault) => parse_fault_step(state, &fault),
    }
}

fn parse_fault_step(mut state: RuntimeState, fault: &lkjagent_protocol::ParseFault) -> StepResult {
    let notice = parse_notice(fault);
    state.parse_faults = state.parse_faults.saturating_add(1);
    state = append_notice(state, NoticeKind::Error, &notice);
    let mut effects = vec![Effect::RecordEvent {
        kind: EventKind::Error,
        content: notice,
        tokens: 32,
    }];
    if should_escalate(state.parse_faults) {
        let reason = "three consecutive parse-class faults".to_string();
        state.task = TaskState::Paused {
            reason: reason.clone(),
        };
        effects.push(Effect::Pause { reason });
    }
    result(state, effects, Some(stop_reason(fault)))
}

fn tool_output_step(mut state: RuntimeState, output: DispatchOutput) -> StepResult {
    let Some(pending) = state.pending_action.take() else {
        state = append_notice(
            state,
            NoticeKind::Error,
            "tool output without pending action",
        );
        return result(state, vec![], Some(StopReason::ToolError));
    };
    state.context = append_output_frame(&state.context, &output);
    let kind = event_kind(&output.kind);
    let mut effects = vec![Effect::RecordEvent {
        kind,
        content: output.rendered.clone(),
        tokens: token_estimate(&output.rendered) as i64,
    }];
    let mut stop = stop_for_output(&output);
    if stop == StopReason::RepeatAction {
        state.repeat_faults = state.repeat_faults.saturating_add(1);
        if should_escalate(state.repeat_faults) {
            let reason = "three consecutive repeat actions".to_string();
            state.task = TaskState::Paused {
                reason: reason.clone(),
            };
            effects.push(Effect::Pause { reason });
        }
    } else if stop != StopReason::ToolError {
        state.repeat_faults = 0;
    }
    if let Some(control_stop) = handle_control_success(&mut state, &pending, &output, &mut effects)
    {
        stop = control_stop;
    }
    result(state, effects, Some(stop))
}
