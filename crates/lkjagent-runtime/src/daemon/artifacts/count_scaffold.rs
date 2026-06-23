use lkjagent_graph::case_recovery::{FaultKind, RecoveryRecord};
use lkjagent_graph::{CaseStatus, GraphNodeId, TaskPhase};
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::count_guard::{CountGuard, CountMode};
use lkjagent_tools::count_seed::scaffold_counted_documents;
use lkjagent_tools::observe;
use rusqlite::Connection;

use super::count_scaffold_gate::{counted_scaffold_closure, CountedScaffoldClosure};
use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::graph_state::status_str;
use crate::task::TaskState;

impl ResidentDaemon {
    pub(super) fn auto_scaffold_counted_documents(
        &mut self,
        conn: &mut Connection,
        now: &str,
        guard: CountGuard,
        objective: &str,
    ) -> RuntimeResult<()> {
        let (output, evidence_summary) =
            match scaffold_counted_documents(&self.runtime.tools.workspace, guard, objective) {
                Ok(content) => {
                    let summary = normalize_scaffold_summary(&content);
                    let output = observe::ok(
                        content,
                        self.runtime.tools.observation_tokens,
                        "finish with agent.done",
                    );
                    (output, Some(summary))
                }
                Err(error) => (
                    observe::error(error.to_string(), self.runtime.tools.observation_tokens),
                    None,
                ),
            };
        self.append_output_frame(conn, now, &output.kind, output.rendered)?;
        if let Some(summary) = evidence_summary {
            self.record_scaffold_graph_evidence(conn, now, &summary, Some("structured-output"))?;
            self.close_counted_scaffold(conn, now, guard, &summary)?;
        }
        Ok(())
    }

    fn close_counted_scaffold(
        &mut self,
        conn: &mut Connection,
        now: &str,
        guard: CountGuard,
        evidence_summary: &str,
    ) -> RuntimeResult<()> {
        let summary = close_summary(guard, evidence_summary);
        let close_target = match counted_scaffold_closure(self.state.graph.as_ref()) {
            CountedScaffoldClosure::Admit { target } => target,
            CountedScaffoldClosure::Wait { question } => {
                return self.wait_counted_scaffold(conn, now, question);
            }
        };
        self.state.task = TaskState::Closed {
            summary: summary.clone(),
        };
        self.dispatch_state.control.work_open = false;
        self.dispatch_state.control.question_outstanding = false;
        self.dispatch_state.control.guard = CompletionGuard::None;
        if let Some(graph) = self.state.graph.as_mut() {
            graph.status = CaseStatus::Closed;
            graph.phase = TaskPhase::Closed;
            graph.active_node = close_target;
            if let Some(case_id) = graph.case_id {
                lkjagent_store::graph::update_case(
                    conn,
                    case_id,
                    graph.phase.as_str(),
                    graph.active_node.0,
                    status_str(graph.status),
                    now,
                )?;
            }
        }
        self.save_task_summary(conn, now, &summary)?;
        self.write_observable(conn)
    }

    fn wait_counted_scaffold(
        &mut self,
        conn: &mut Connection,
        now: &str,
        question: String,
    ) -> RuntimeResult<()> {
        self.state.task = TaskState::Waiting {
            question: question.clone(),
        };
        self.dispatch_state.control.work_open = true;
        self.dispatch_state.control.question_outstanding = true;
        if let Some(graph) = self.state.graph.as_mut() {
            graph.phase = TaskPhase::Waiting;
            graph.active_node = GraphNodeId("recover");
            graph.recovery.strategy = Some(question.clone());
            graph.recovery.history.push(RecoveryRecord {
                kind: FaultKind::Verification,
                summary: question.clone(),
                action_fingerprint: None,
            });
            if let Some(case_id) = graph.case_id {
                lkjagent_store::graph::update_case(
                    conn,
                    case_id,
                    graph.phase.as_str(),
                    graph.active_node.0,
                    status_str(graph.status),
                    now,
                )?;
                lkjagent_store::graph::record_event(
                    conn,
                    case_id,
                    "recovery",
                    graph.active_node.0,
                    graph.phase.as_str(),
                    &question,
                    now,
                )?;
            }
        }
        self.write_observable(conn)
    }
}

fn close_summary(guard: CountGuard, evidence_summary: &str) -> String {
    match guard.mode {
        CountMode::Exact => format!(
            "created counted structured-output scaffold with {} files\n{evidence_summary}",
            guard.target
        ),
        CountMode::Approximate => format!(
            "created structured-output scaffold at about {}-file scale\n{evidence_summary}",
            guard.target
        ),
    }
}

fn normalize_scaffold_summary(content: &str) -> String {
    content.split_whitespace().collect::<Vec<_>>().join(" ")
}
