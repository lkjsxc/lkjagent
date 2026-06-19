use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_graph::{
    completion_decision, EvidenceKind, GraphNodeId, TaskPhase, TransitionDecision,
};
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_tools::benchmark_seed::scaffold_markdown_corpus;
use lkjagent_tools::observe::{self, OutputKind};
use lkjagent_tools::structure_seed::{scaffold_profile, ScaffoldProfile};
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::graph_state::{evidence_record, graph_notice_frame, row_from_evidence, status_str};
use crate::prompt::token_estimate;

impl ResidentDaemon {
    pub(super) fn auto_scaffold_recursive_docs(
        &mut self,
        conn: &Connection,
        now: &str,
        profile: ScaffoldProfile,
    ) -> RuntimeResult<()> {
        let output = match scaffold_profile(&self.runtime.tools.workspace, profile) {
            Ok(content) => observe::ok(
                content,
                self.runtime.tools.observation_tokens,
                "inspect docs with shell.run",
            ),
            Err(error) => observe::error(error.to_string(), self.runtime.tools.observation_tokens),
        };
        self.append_output_frame(conn, now, &output.kind, output.rendered)?;
        if matches!(output.kind, OutputKind::Observation { .. }) {
            self.record_scaffold_graph_evidence(conn, now, "recursive docs scaffold")?;
        }
        Ok(())
    }

    pub(super) fn auto_scaffold_markdown_corpus(
        &mut self,
        conn: &Connection,
        now: &str,
        target: usize,
    ) -> RuntimeResult<()> {
        let output = match scaffold_markdown_corpus(&self.runtime.tools.workspace, target) {
            Ok(content) => observe::ok(
                content,
                self.runtime.tools.observation_tokens,
                "finish with agent.done",
            ),
            Err(error) => observe::error(error.to_string(), self.runtime.tools.observation_tokens),
        };
        self.append_output_frame(conn, now, &output.kind, output.rendered)?;
        if matches!(output.kind, OutputKind::Observation { .. }) {
            self.record_scaffold_graph_evidence(conn, now, "markdown corpus scaffold")?;
        }
        Ok(())
    }

    pub(super) fn recursive_docs_requested(content: &str) -> bool {
        let lower = content.to_ascii_lowercase();
        lower.contains("docs")
            || lower.contains("documentation")
            || lower.contains("encyclopedia")
            || lower.contains("knowledge base")
            || lower.contains("wiki")
            || content.contains("ドキュメント")
            || content.contains("百科事典")
    }

    pub(super) fn benchmark_docs_requested(content: &str) -> bool {
        let lower = content.to_ascii_lowercase();
        lower.contains("docs/benchmark-corpus")
            || (lower.contains("benchmark")
                && (lower.contains("documentation") || lower.contains("corpus")))
    }

    pub(super) fn scaffold_profile(&self) -> ScaffoldProfile {
        if self.dispatch_state.control.guard.is_knowledge() {
            ScaffoldProfile::Knowledge
        } else {
            ScaffoldProfile::Generic
        }
    }

    pub(super) fn append_output_frame(
        &mut self,
        conn: &Connection,
        now: &str,
        kind: &OutputKind,
        rendered: String,
    ) -> RuntimeResult<()> {
        let tokens = token_estimate(&rendered);
        self.state.context = append_frame(
            &self.state.context,
            Frame::new(frame_kind(kind), rendered.clone(), tokens),
        );
        append_event(
            conn,
            self.event_turn(),
            event_kind(kind),
            &rendered,
            tokens as i64,
            now,
        )?;
        Ok(())
    }

    pub(super) fn record_scaffold_graph_evidence(
        &mut self,
        conn: &Connection,
        now: &str,
        summary: &str,
    ) -> RuntimeResult<()> {
        let Some(graph) = self.state.graph.as_mut() else {
            return Ok(());
        };
        for (requirement, kind) in [
            ("observation", EvidenceKind::Observation),
            ("document-structure", EvidenceKind::File),
            ("verification", EvidenceKind::Verification),
        ] {
            if missing_required_evidence(graph, requirement) {
                let evidence = evidence_record(requirement, kind, summary.to_string(), None);
                if let Some(case_id) = graph.case_id {
                    lkjagent_store::graph::record_evidence(
                        conn,
                        case_id,
                        &row_from_evidence(&evidence),
                        now,
                    )?;
                }
                graph.evidence.push(evidence);
            }
        }
        graph.pending_checks.clear();
        refresh_graph_case(graph);
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
        self.state.context = append_frame(&self.state.context, graph_notice_frame(graph));
        Ok(())
    }
}

fn missing_required_evidence(graph: &lkjagent_graph::TaskGraphState, requirement: &str) -> bool {
    graph
        .evidence_requirements
        .iter()
        .any(|item| item == requirement)
        && !graph
            .evidence
            .iter()
            .any(|item| item.requirement == requirement)
}

fn frame_kind(kind: &OutputKind) -> FrameKind {
    match kind {
        OutputKind::Observation { .. } => FrameKind::Observation,
        OutputKind::Notice { .. } => FrameKind::Notice(NoticeKind::Error),
    }
}

fn event_kind(kind: &OutputKind) -> EventKind {
    match kind {
        OutputKind::Notice { .. } => EventKind::Notice,
        OutputKind::Observation { .. } => EventKind::Observation,
    }
}

fn refresh_graph_case(graph: &mut lkjagent_graph::TaskGraphState) {
    match completion_decision(graph) {
        TransitionDecision::Admit { .. } => {
            graph.phase = TaskPhase::Completion;
            graph.active_node = GraphNodeId("complete");
        }
        TransitionDecision::Defer { .. } => {
            graph.phase = TaskPhase::Execution;
            graph.active_node = GraphNodeId("execute");
        }
        TransitionDecision::Recover { .. } | TransitionDecision::Refuse { .. } => {
            graph.phase = TaskPhase::Recovery;
            graph.active_node = GraphNodeId("recover");
        }
    }
}
