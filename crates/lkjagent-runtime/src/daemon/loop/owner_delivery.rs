use lkjagent_graph::TaskFamily;
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::persisted::{next_owner_tokens, owner_preview};
use super::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::graph_state::open_owner_case_with_guard;
use crate::intake::{self, DeliveredOwner};
use crate::step::{step, StepInput};
use crate::task::TaskState;

impl ResidentDaemon {
    pub(super) fn deliver_owner(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<()> {
        let tokens = next_owner_tokens(conn)?;
        let turn = self.state.turn.saturating_add(1);
        let Some(owner) = intake::deliver_next(conn, turn, tokens as i64, now)? else {
            return Ok(());
        };
        let owner_content = owner.content.clone();
        self.endpoint_retry_at = None;
        self.dispatch_state.reset_repeat_tracking();
        let starting_task = !matches!(
            self.state.task,
            TaskState::Open { .. } | TaskState::Waiting { .. }
        );
        let visible_task = store_state::get(conn, "open task")?;
        let visible_maintenance = visible_task
            .as_deref()
            .is_some_and(|task| task.starts_with("maintenance:"));
        if starting_task || visible_maintenance {
            store_state::set(conn, "open task", &owner_preview(&owner.content))?;
        }
        let previous_guard = self.dispatch_state.control.guard;
        update_guard(self, starting_task, &owner.content);
        if starting_task || previous_guard != self.dispatch_state.control.guard {
            store_state::set(
                conn,
                "completion guard",
                &self.dispatch_state.control.guard.as_state_value(),
            )?;
        }
        self.open_owner_step(
            conn,
            now,
            owner,
            starting_task,
            visible_maintenance,
            owner_content,
        )
    }

    fn open_owner_step(
        &mut self,
        conn: &mut Connection,
        now: &str,
        owner: DeliveredOwner,
        starting_task: bool,
        visible_maintenance: bool,
        owner_content: String,
    ) -> RuntimeResult<()> {
        let guard = self.dispatch_state.control.guard;
        let scaffold_docs =
            starting_task && guard.is_recursive() && Self::recursive_docs_requested(&owner.content);
        let benchmark_target = guard
            .markdown_target()
            .filter(|_| Self::benchmark_docs_requested(&owner.content));
        let graph = if starting_task || visible_maintenance {
            Some(open_owner_case_with_guard(
                conn,
                &owner.content,
                now,
                guard,
            )?)
        } else {
            None
        };
        let counted_guard = graph.as_ref().and_then(|graph| {
            (starting_task
                && benchmark_target.is_none()
                && graph.family == TaskFamily::Documentation
                && !manuscript_scaffold_veto(&owner.content, graph.document.as_ref())
                && !guard.is_recursive())
            .then(|| guard.count_guard())
            .flatten()
        });
        let scaffold_profile = self.scaffold_profile();
        self.apply_step_result(
            conn,
            now,
            step(
                self.state.clone(),
                StepInput::Owner {
                    content: owner.content,
                    tokens: owner.tokens,
                    graph: graph.map(Box::new),
                    turn_budget: self.runtime.task_turn_budget,
                },
            ),
            true,
        )?;
        self.maybe_auto_scaffold(conn, now, scaffold_profile, scaffold_docs, benchmark_target)?;
        if let Some(guard) = counted_guard {
            self.auto_scaffold_counted_documents(conn, now, guard, &owner_content)?;
        }
        Ok(())
    }

    fn maybe_auto_scaffold(
        &mut self,
        conn: &mut Connection,
        now: &str,
        profile: lkjagent_tools::structure_seed::ScaffoldProfile,
        scaffold_docs: bool,
        benchmark_target: Option<usize>,
    ) -> RuntimeResult<()> {
        if self.dispatch_state.control.guard.is_recursive() && scaffold_docs {
            self.auto_scaffold_recursive_docs(conn, now, profile)?;
        }
        if let Some(target) = benchmark_target {
            self.auto_scaffold_markdown_corpus(conn, now, target)?;
        }
        Ok(())
    }
}

fn update_guard(daemon: &mut ResidentDaemon, starting_task: bool, content: &str) {
    if starting_task {
        daemon.dispatch_state.control.start_task(content);
    } else {
        daemon.dispatch_state.control.resume_task_with(content);
    }
}

fn manuscript_scaffold_veto(
    content: &str,
    document: Option<&lkjagent_graph::case_document::DocumentState>,
) -> bool {
    let lower = content.to_ascii_lowercase();
    let story_root = document
        .map(|doc| doc.root.trim_start_matches("./").starts_with("stories/"))
        .unwrap_or(false);
    lower.contains("do not create structured-output")
        || lower.contains("no structured-output")
        || lower.contains("/manuscript/")
        || (story_root && lower.contains("manuscript"))
        || (story_root && lower.contains("chapter") && lower.contains("word"))
}
