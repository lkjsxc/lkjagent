use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_context::model::{Frame, NoticeKind};
use lkjagent_graph::TaskGraphState;
use lkjagent_store::events::EventKind;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::maintenance::MaintenanceDirective;
use crate::prompt::token_estimate;
use crate::recovery::{repeat_recovery_notice, should_escalate, tool_recovery_notice};
use crate::task::{RuntimeState, StopReason};

mod action_params;
mod budget;
mod compact;
mod cycle;
mod effects_model;
mod fault_wait;
mod frames;
mod graph_output;
mod graph_output_evidence;
mod graph_output_plan;
mod graph_output_plan_helpers;
mod graph_phase;
mod output;
mod oversize;
mod turn;

use compact::compact_step;
use cycle::maintenance_start_step;
pub use effects_model::{Effect, GraphPlanStepEffect, GraphStateTrackEffect};
use fault_wait::{enter_recovery_route, record_recoverable_fault, RecoveryFault};
use frames::{append_notice, result};
use output::{append_output_frame, event_kind, handle_control_success, stop_for_output};
use turn::{completion_step, owner_step};

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
    EndpointOversize {
        preview: String,
    },
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepResult {
    pub state: RuntimeState,
    pub effects: Vec<Effect>,
    pub stop_reason: Option<StopReason>,
}

pub fn step(state: RuntimeState, input: StepInput) -> StepResult {
    match input {
        StepInput::Owner {
            content,
            tokens,
            graph,
            turn_budget,
        } => owner_step(
            state,
            content,
            tokens,
            graph.map(|boxed| *boxed),
            turn_budget,
        ),
        StepInput::Completion { content, tokens } => completion_step(state, content, tokens),
        StepInput::EndpointOversize { preview } => turn::endpoint_oversize_step(state, &preview),
        StepInput::ToolOutput(output) => tool_output_step(state, output),
        StepInput::Compact {
            prefix,
            summary,
            memory_ids,
            policy,
        } => compact_step(state, prefix, summary, memory_ids, policy),
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
    graph_output::update_graph_after_output(&mut state, &pending, &output, &mut effects);
    let mut stop = stop_for_output(&output);
    if stop == StopReason::RepeatAction {
        state.repeat_faults = state.repeat_faults.saturating_add(1);
        state.tool_faults = 0;
        let recovery = repeat_recovery_notice(state.repeat_faults);
        state = append_recovery_notice(state, &recovery, &mut effects);
        let count = state.repeat_faults;
        record_recoverable_fault(
            &mut state,
            RecoveryFault::Repeat,
            count,
            Some(pending.action_text.clone()),
            &recovery,
            &mut effects,
        );
        if should_escalate(state.repeat_faults) {
            state = enter_recovery_route(
                state,
                RecoveryFault::Repeat,
                count,
                Some(pending.action_text.clone()),
                &mut effects,
            );
        }
    } else {
        state.repeat_faults = 0;
        if stop == StopReason::ToolError {
            state.tool_faults = state.tool_faults.saturating_add(1);
            let recovery = tool_recovery_notice(&output.content);
            state = append_recovery_notice(state, &recovery, &mut effects);
            let count = state.tool_faults;
            record_recoverable_fault(
                &mut state,
                RecoveryFault::Tool,
                count,
                Some(pending.action_text.clone()),
                &recovery,
                &mut effects,
            );
            if should_escalate(state.tool_faults) {
                state = enter_recovery_route(
                    state,
                    RecoveryFault::Tool,
                    count,
                    Some(pending.action_text.clone()),
                    &mut effects,
                );
            }
        } else {
            state.tool_faults = 0;
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
