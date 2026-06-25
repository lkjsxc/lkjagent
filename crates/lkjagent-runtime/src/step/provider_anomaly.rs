use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;

use crate::prompt::token_estimate;
use crate::step::budget::{budget_exhausted_step, spend_active_budget};
use crate::step::frames::{append_notice, result};
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason};

pub(super) fn provider_anomaly_step(
    mut state: RuntimeState,
    class: &str,
    detail: &str,
) -> StepResult {
    state.turn = state.turn.saturating_add(1);
    let exhausted = spend_active_budget(&mut state);
    if let Some(exhausted) = exhausted {
        return budget_exhausted_step(state, exhausted);
    }
    let error = format!("provider anomaly: {class}");
    let notice = format!(
        "provider response had no usable assistant action; class={class}; detail={detail}; route=endpoint-recovery; parse_fault_counter=unchanged"
    );
    state = append_notice(state, NoticeKind::Error, &error);
    state = append_notice(state, NoticeKind::Error, &notice);
    result(
        state,
        vec![
            Effect::RecordEvent {
                kind: EventKind::Error,
                content: error.clone(),
                tokens: token_estimate(&error) as i64,
            },
            Effect::RecordEvent {
                kind: EventKind::Notice,
                content: notice.clone(),
                tokens: token_estimate(&notice) as i64,
            },
        ],
        Some(StopReason::EndpointError),
    )
}
