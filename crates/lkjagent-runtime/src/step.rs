use lkjagent_context::model::{Frame, NoticeKind};
use lkjagent_store::events::EventKind;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::maintenance::MaintenanceDirective;
use crate::prompt::token_estimate;
use crate::recovery::{repeat_recovery_notice, tool_recovery_notice};
use crate::task::{RuntimeState, StopReason};

mod compact;
mod cycle;
mod frames;
mod output;
mod turn;

use compact::compact_step;
use cycle::maintenance_start_step;
use frames::{append_notice, result};
use output::{append_output_frame, event_kind, handle_control_success, stop_for_output};
use turn::{completion_step, owner_step};

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
    EndpointOversize,
    ToolOutput(DispatchOutput),
    Compact {
        prefix: Vec<Frame>,
        summary: Frame,
        memory_ids: Vec<i64>,
    },
    StartMaintenance {
        directive: MaintenanceDirective,
        budget: u16,
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
        prompt: String,
        max_turns: u8,
    },
    DistillCompaction {
        prompt: String,
        max_turns: u8,
        task_summary_required: bool,
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
        StepInput::EndpointOversize => turn::endpoint_oversize_step(state),
        StepInput::ToolOutput(output) => tool_output_step(state, output),
        StepInput::Compact {
            prefix,
            summary,
            memory_ids,
        } => compact_step(state, prefix, summary, memory_ids),
        StepInput::StartMaintenance { directive, budget } => {
            maintenance_start_step(state, directive, budget)
        }
    }
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
        let recovery = repeat_recovery_notice(state.repeat_faults);
        state = append_recovery_notice(state, &recovery, &mut effects);
    } else {
        state.repeat_faults = 0;
        if stop == StopReason::ToolError {
            let recovery = tool_recovery_notice(&output.content);
            state = append_recovery_notice(state, &recovery, &mut effects);
        }
    }
    if let Some(control_stop) = handle_control_success(&mut state, &pending, &output, &mut effects)
    {
        stop = control_stop;
    }
    result(state, effects, Some(stop))
}

fn append_recovery_notice(
    state: RuntimeState,
    content: &str,
    effects: &mut Vec<Effect>,
) -> RuntimeState {
    effects.push(Effect::RecordEvent {
        kind: EventKind::Notice,
        content: content.to_string(),
        tokens: token_estimate(content) as i64,
    });
    append_notice(state, NoticeKind::Error, content)
}
