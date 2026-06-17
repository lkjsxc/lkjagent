use lkjagent_context::model::NoticeKind;
use lkjagent_store::events::EventKind;

use crate::maintenance::{maintenance_notice, MaintenanceCycle, MaintenanceDirective};
use crate::prompt::token_estimate;
use crate::step::frames::{append_notice, result};
use crate::step::{Effect, StepResult};
use crate::task::{RuntimeState, StopReason};

pub(super) fn maintenance_start_step(
    mut state: RuntimeState,
    directive: MaintenanceDirective,
    budget: u16,
) -> StepResult {
    let notice = maintenance_notice(directive, budget);
    state.maintenance = Some(MaintenanceCycle {
        directive,
        turns_remaining: budget,
    });
    state = append_notice(state, NoticeKind::Maintenance, &notice);
    result(
        state,
        vec![Effect::RecordEvent {
            kind: EventKind::Notice,
            content: notice.clone(),
            tokens: token_estimate(&notice) as i64,
        }],
        Some(StopReason::Maintenance),
    )
}
