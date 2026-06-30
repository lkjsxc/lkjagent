use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;

use crate::prompt::token_estimate;
use crate::step::budget::{budget_exhausted_step, spend_active_budget};
use crate::step::fault_wait::{enter_recovery_route, record_recoverable_fault, RecoveryFault};
use crate::step::frames::{append_notice, result};
use crate::step::oversize::{oversize_error, oversize_recovery, write_payload};
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason, TaskState};

pub(super) fn endpoint_oversize_step(mut state: RuntimeState, preview: &str) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let exhausted = spend_active_budget(&mut state);
    if let Some(exhausted) = exhausted {
        return budget_exhausted_step(state, exhausted);
    }
    let error = oversize_error(preview);
    let recovery = oversize_recovery(preview);
    state = append_notice(state, NoticeKind::Error, &error);
    state = append_notice(state, NoticeKind::Error, &recovery);
    let mut effects = events(&error, &recovery);
    if payload_risk(preview) {
        let count = state.parse_faults.saturating_add(1);
        state.parse_faults = count;
        record_recoverable_fault(
            &mut state,
            RecoveryFault::Payload,
            count,
            None,
            &recovery,
            &mut effects,
        );
        state = enter_recovery_route(state, RecoveryFault::Payload, count, None, &mut effects);
        if count >= 2 {
            if let Some(path) = manuscript_path(preview) {
                let handoff = format!(
                    "Blocked manuscript write after endpoint max tokens; remaining_path={path}"
                );
                state = append_notice(state, NoticeKind::Error, &handoff);
                state.task = TaskState::Waiting {
                    question: handoff.clone(),
                };
                effects.push(Effect::RecordEvent {
                    kind: EventKind::Notice,
                    content: handoff,
                    tokens: 12,
                });
                return result(state, effects, Some(StopReason::Ask));
            }
        }
    }
    result(state, effects, Some(StopReason::InvalidAction))
}

fn events(error: &str, recovery: &str) -> Vec<Effect> {
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
    ]
}

fn payload_risk(preview: &str) -> bool {
    write_payload(preview)
}

fn manuscript_path(preview: &str) -> Option<String> {
    preview
        .split_whitespace()
        .map(clean_path_token)
        .find(|token| {
            token.starts_with("stories/") && token.contains("/manuscript/") && token.contains(".md")
        })
        .map(|token| token.trim_end_matches('.').to_string())
}

fn clean_path_token(token: &str) -> &str {
    token
        .trim_matches(|ch: char| ch == '<' || ch == '>' || ch == ',')
        .split("\\n")
        .next()
        .unwrap_or(token)
        .split("content:")
        .next()
        .unwrap_or(token)
}
