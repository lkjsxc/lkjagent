use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;
use lkjagent_tools::dispatch::DispatchOutput;

use crate::prompt::token_estimate;
use crate::recovery::{repeat_recovery_notice, should_escalate, tool_recovery_notice};
use crate::task::{RuntimeState, StopReason};

mod action_params;
mod budget;
mod budget_render;
mod compact;
mod cycle;
mod effects_model;
mod fault_key;
mod fault_meta;
mod fault_wait;
mod frames;
mod graph_output;
mod graph_output_evidence;
mod graph_output_plan;
mod graph_output_plan_helpers;
mod graph_phase;
mod input;
mod output;
mod oversize;
mod oversize_step;
mod owner_guidance;
mod provider_anomaly;
mod recovery_select;
mod turn;

use compact::compact_step;
use cycle::maintenance_start_step;
pub use effects_model::{Effect, GraphPlanStepEffect, GraphStateTrackEffect};
use fault_wait::{enter_recovery_route, record_recoverable_fault, RecoveryFault};
use frames::{append_notice, result};
pub use input::StepInput;
use output::{append_output_frame, event_kind, handle_control_success, stop_for_output};
use provider_anomaly::provider_anomaly_step;
use turn::{completion_step as complete, owner_step};

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
        StepInput::Completion { content, tokens } => complete(state, content, tokens, None),
        StepInput::AuthorizedCompletion(content, tokens, authority) => {
            complete(state, content, tokens, Some(authority))
        }
        StepInput::TurnBudgetCheckpoint => budget::task_checkpoint_step(state),
        StepInput::EndpointOversize { preview } => {
            oversize_step::endpoint_oversize_step(state, &preview)
        }
        StepInput::ProviderAnomaly(class, detail) => provider_anomaly_step(state, &class, &detail),
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
