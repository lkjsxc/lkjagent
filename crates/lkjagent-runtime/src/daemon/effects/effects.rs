use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;
use crate::step::{Effect, StepResult};
use crate::task::StopReason;

impl ResidentDaemon {
    pub(super) fn apply_step_result(
        &mut self,
        conn: &mut Connection,
        now: &str,
        result: StepResult,
        skip_owner_record: bool,
    ) -> RuntimeResult<DaemonTick> {
        let mut tick = tick_for_stop(result.stop_reason);
        self.state = result.state;
        self.annotate_pending_authority(conn)?;
        for effect in result.effects {
            if let Some(next) = self.apply_effect(conn, now, effect, skip_owner_record)? {
                tick = next;
            }
        }
        self.write_observable(conn)?;
        Ok(tick)
    }

    fn annotate_pending_authority(&mut self, conn: &Connection) -> RuntimeResult<()> {
        let Some(pending) = self.state.pending_action.as_mut() else {
            return Ok(());
        };
        if pending.authority_decision_id.is_none() {
            pending.authority_decision_id = store_state::get(conn, "authority decision id")?;
        }
        if pending.prompt_frame_id.is_none() {
            pending.prompt_frame_id = store_state::get(conn, "authority prompt frame id")?;
        }
        if pending.staleness_fingerprint.is_none() {
            pending.staleness_fingerprint = store_state::get(conn, "kernel staleness fingerprint")?;
        }
        Ok(())
    }

    pub(super) fn record_endpoint_error(
        &self,
        conn: &Connection,
        now: &str,
        message: &str,
    ) -> RuntimeResult<()> {
        append_event(
            conn,
            self.event_turn(),
            EventKind::Error,
            message,
            token_estimate(message) as i64,
            now,
        )?;
        store_state::set(conn, "daemon state", "error")?;
        store_state::set(conn, "daemon error", message)?;
        Ok(())
    }

    fn apply_effect(
        &mut self,
        conn: &mut Connection,
        now: &str,
        effect: Effect,
        skip_owner_record: bool,
    ) -> RuntimeResult<Option<DaemonTick>> {
        match effect {
            Effect::RecordEvent {
                kind: EventKind::Owner,
                ..
            } if skip_owner_record => Ok(None),
            Effect::RecordEvent {
                kind,
                content,
                tokens,
            } => {
                append_event(conn, self.event_turn(), kind, &content, tokens, now)?;
                Ok(None)
            }
            Effect::ExecuteTool { action_text } => self.execute_pending(conn, now, &action_text),
            Effect::DistillTask { summary, .. } => {
                self.save_task_summary(conn, now, &summary)?;
                Ok(None)
            }
            effect @ (Effect::RecordGraphEvidence { .. }
            | Effect::RecordGraphPlan { .. }
            | Effect::RecordGraphContext { .. }
            | Effect::RecordGraphNote { .. }
            | Effect::RecordGraphTransition { .. }
            | Effect::RecordGraphFault { .. }
            | Effect::UpdateGraphRecovery { .. }
            | Effect::ReplaceGraphStateTracks { .. }
            | Effect::UpdateGraphCase { .. }) => {
                self.apply_graph_effect(conn, now, effect)?;
                Ok(None)
            }
            Effect::Pause { reason } => {
                store_state::set(conn, "daemon error", &reason)?;
                Ok(Some(DaemonTick::Paused))
            }
            Effect::CompactionRecorded {
                before_tokens,
                after_tokens,
                memory_ids,
                policy,
            } => self.record_compaction(conn, now, before_tokens, after_tokens, memory_ids, policy),
            Effect::DeferMaintenance => {
                crate::maintenance::defer_all_directives(conn, now)?;
                Ok(None)
            }
        }
    }
}

fn tick_for_stop(stop: Option<StopReason>) -> DaemonTick {
    match stop {
        Some(StopReason::Done) => DaemonTick::Done,
        Some(StopReason::Ask) => DaemonTick::Waiting,
        Some(StopReason::EndpointError) => DaemonTick::EndpointError,
        Some(StopReason::Acted | StopReason::Compaction | StopReason::Maintenance) => {
            DaemonTick::Working
        }
        Some(StopReason::InvalidAction | StopReason::UnknownTool) => DaemonTick::Working,
        Some(StopReason::BadParams | StopReason::BudgetNotice) => DaemonTick::Working,
        Some(StopReason::ToolError | StopReason::RepeatAction) => DaemonTick::Working,
        None => DaemonTick::Working,
    }
}
