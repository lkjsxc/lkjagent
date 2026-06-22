use lkjagent_context::budget::{ContextPressure, LOG_OBSERVATION};
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::render_notice;
use lkjagent_store::{memory, state as store_state};
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::maintenance::task_summary_required;
use crate::prompt::token_estimate;
use crate::step::{step, StepInput};

use super::compaction_support::compaction_summary;

impl ResidentDaemon {
    pub(super) fn start_compaction_distillation(
        &mut self,
        conn: &mut Connection,
        now: &str,
        reason: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        if self.state.pending_action.is_some() || self.state.compaction.is_some() {
            return Ok(None);
        }
        self.finish_compaction(conn, now, reason).map(Some)
    }

    pub(super) fn continue_compaction_before_endpoint(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let pressure = self
            .runtime
            .budget
            .pressure(self.state.context.used_tokens(), LOG_OBSERVATION);
        if matches!(
            pressure,
            ContextPressure::Red | ContextPressure::BlackInvalid
        ) {
            self.finish_compaction(conn, now, "compaction pressure")
                .map(Some)
        } else {
            Ok(None)
        }
    }

    pub(super) fn finish_compaction(
        &mut self,
        conn: &mut Connection,
        now: &str,
        reason: &str,
    ) -> RuntimeResult<DaemonTick> {
        if self.state.pending_action.is_some() {
            return Ok(DaemonTick::Working);
        }
        let before = self.state.compaction.as_ref().map_or_else(
            || self.state.context.used_tokens(),
            |cycle| cycle.before_tokens,
        );
        let (summary, memory_ids) = self.compaction_summary_rows(conn, now, reason, before)?;
        let prefix = super::startup::build_prefix_from_store(conn, &self.runtime.tools.workspace)?;
        let rendered = render_notice("compaction", &summary);
        let frame = Frame::new(
            FrameKind::Notice(NoticeKind::Compaction),
            rendered.clone(),
            token_estimate(&rendered),
        );
        let result = step(
            self.state.clone(),
            StepInput::Compact {
                prefix,
                summary: frame,
                memory_ids,
                policy: self.runtime.budget,
            },
        );
        self.apply_step_result(conn, now, result, false)
    }

    fn compaction_summary_rows(
        &mut self,
        conn: &mut Connection,
        now: &str,
        reason: &str,
        before: usize,
    ) -> RuntimeResult<(String, Vec<i64>)> {
        let summary = compaction_summary(
            conn,
            reason,
            before,
            self.state.graph.as_ref(),
            &self.state.context.log,
        )?;
        self.save_harness_summary(conn, now, summary)
    }

    fn save_harness_summary(
        &mut self,
        conn: &mut Connection,
        now: &str,
        summary: String,
    ) -> RuntimeResult<(String, Vec<i64>)> {
        if !task_summary_required(&self.state.task) {
            return Ok((summary, Vec::new()));
        }
        let id = memory::save(
            conn,
            memory::MemoryKind::TaskSummary,
            "compaction resume",
            "compaction task",
            &summary,
            token_estimate(&summary) as i64,
            now,
        )?;
        store_state::set(conn, "last task summary id", &id.to_string())?;
        Ok((summary, vec![id]))
    }
}
