use lkjagent_context::assemble::append_frame;
use lkjagent_context::budget::{ContextPressure, LOG_OBSERVATION};
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_protocol::{render_notice, Action};
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::{memory, state as store_state};
use lkjagent_tools::dispatch::DispatchOutput;
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::maintenance::task_summary_required;
use crate::prompt::token_estimate;
use crate::step::{step, StepInput};
use crate::task::CompactionCycle;

use super::compaction_support::{compaction_prompt, compaction_summary, memory_id, param};

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
        let before = self.state.context.used_tokens();
        let prompt = compaction_prompt(task_summary_required(&self.state.task));
        let rendered = render_notice("compaction", &prompt);
        let tokens = token_estimate(&rendered);
        let pressure = self.runtime.budget.pressure(before, tokens);
        if matches!(
            pressure,
            ContextPressure::Red | ContextPressure::BlackInvalid
        ) {
            return self.finish_compaction(conn, now, reason).map(Some);
        }
        self.state.context = append_frame(
            &self.state.context,
            Frame::new(FrameKind::Notice(NoticeKind::Compaction), rendered, tokens),
        );
        self.state.compaction = Some(CompactionCycle {
            before_tokens: before,
            turns_remaining: 4,
            task_summary_required: task_summary_required(&self.state.task),
            task_summary_saved: false,
            memory_ids: Vec::new(),
        });
        append_event(
            conn,
            self.event_turn(),
            EventKind::Notice,
            &prompt,
            tokens as i64,
            now,
        )?;
        self.write_observable(conn)?;
        Ok(Some(DaemonTick::Working))
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

    pub(super) fn advance_compaction_after_output(
        &mut self,
        conn: &mut Connection,
        now: &str,
        action: &Action,
        output: &DispatchOutput,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let Some(mut cycle) = self.state.compaction.take() else {
            return Ok(None);
        };
        if action.tool == "memory.save" {
            if let Some(id) = memory_id(&output.content) {
                cycle.memory_ids.push(id);
                if param(action, "kind").as_deref() == Some("task-summary") {
                    cycle.task_summary_saved = true;
                    store_state::set(conn, "last task summary id", &id.to_string())?;
                }
            }
        }
        cycle.turns_remaining = cycle.turns_remaining.saturating_sub(1);
        let done = cycle.turns_remaining == 0
            || (cycle.task_summary_required && cycle.task_summary_saved)
            || (!cycle.task_summary_required && !cycle.memory_ids.is_empty());
        self.state.compaction = Some(cycle);
        if done {
            self.finish_compaction(conn, now, "distillation complete")
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
        let summary = compaction_summary(conn, reason, before)?;
        let Some(cycle) = self.state.compaction.as_ref() else {
            return self.save_harness_summary(conn, now, summary);
        };
        let mut ids = cycle.memory_ids.clone();
        if cycle.task_summary_required && !cycle.task_summary_saved {
            let (summary, mut harness_ids) = self.save_harness_summary(conn, now, summary)?;
            ids.append(&mut harness_ids);
            return Ok((summary, ids));
        }
        Ok((summary, ids))
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
