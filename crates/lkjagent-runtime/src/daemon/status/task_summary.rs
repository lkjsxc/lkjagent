use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::maintenance::defer_all_directives;
use crate::prompt::token_estimate;

impl ResidentDaemon {
    pub(super) fn save_task_summary(
        &mut self,
        conn: &mut Connection,
        now: &str,
        summary: &str,
    ) -> RuntimeResult<()> {
        let memory_id = lkjagent_store::memory::save(
            conn,
            lkjagent_store::memory::MemoryKind::TaskSummary,
            &summary_title(summary),
            "task",
            summary,
            token_estimate(summary) as i64,
            now,
        )?;
        self.link_task_summary_memory(conn, memory_id, now)?;
        store_state::set(conn, "last task summary id", &memory_id.to_string())?;
        store_state::set(conn, "open task", "none")?;
        store_state::delete(conn, "completion guard")?;
        defer_all_directives(conn, now)?;
        let content = format!("task-summary memory_id={memory_id}\nsummary={summary}");
        append_event(
            conn,
            self.event_turn(),
            EventKind::Notice,
            &content,
            token_estimate(&content) as i64,
            now,
        )?;
        Ok(())
    }
}

impl ResidentDaemon {
    fn link_task_summary_memory(
        &self,
        conn: &Connection,
        memory_id: i64,
        now: &str,
    ) -> RuntimeResult<()> {
        let Some(graph) = self.state.graph.as_ref() else {
            return Ok(());
        };
        let Some(case_id) = graph.case_id else {
            return Ok(());
        };
        lkjagent_store::graph::link_memory(
            conn,
            case_id,
            memory_id,
            graph.active_node.0,
            "task-summary",
            now,
        )?;
        Ok(())
    }
}

fn summary_title(summary: &str) -> String {
    let first = summary
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .unwrap_or("closed task");
    first.chars().take(80).collect()
}
