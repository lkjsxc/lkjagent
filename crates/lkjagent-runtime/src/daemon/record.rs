use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_store::events::{append_event, EventKind};
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;

impl ResidentDaemon {
    pub(super) fn record_compaction(
        &self,
        conn: &Connection,
        now: &str,
        before_tokens: usize,
        after_tokens: usize,
        memory_ids: Vec<i64>,
        policy: ContextBudgetPolicy,
    ) -> RuntimeResult<Option<DaemonTick>> {
        let content = format!(
            "before_tokens={before_tokens}\nafter_tokens={after_tokens}\n\
             memory_ids={memory_ids:?}\ncontext_window={}\ncontext_reserve={}\n\
             context_soft_trigger={}\ncontext_hard_trigger={}\n\
             context_post_compaction_target={}",
            policy.window,
            policy.reserve,
            policy.soft_trigger,
            policy.hard_trigger,
            policy.post_compaction_target
        );
        append_event(
            conn,
            self.event_turn(),
            EventKind::Compaction,
            &content,
            16,
            now,
        )?;
        Ok(None)
    }
}
