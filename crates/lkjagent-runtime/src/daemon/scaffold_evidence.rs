use lkjagent_context::assemble::append_frame;
use lkjagent_graph::{
    completion_decision, EvidenceKind, GraphNodeId, TaskGraphState, TaskPhase, TransitionDecision,
};
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::graph_state::{evidence_record, graph_notice_frame, row_from_evidence, status_str};

impl ResidentDaemon {
    pub(super) fn record_scaffold_graph_evidence(
        &mut self,
        conn: &Connection,
        now: &str,
        summary: &str,
        path: Option<&str>,
    ) -> RuntimeResult<()> {
        let Some(graph) = self.state.graph.as_mut() else {
            return Ok(());
        };
        for (requirement, kind) in [
            ("observation", EvidenceKind::Observation),
            ("document-structure", EvidenceKind::File),
            ("verification", EvidenceKind::Verification),
        ] {
            if missing_scaffold_evidence(graph, requirement) {
                let evidence = evidence_record(
                    requirement,
                    kind,
                    summary.to_string(),
                    path.map(str::to_string),
                );
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

fn missing_scaffold_evidence(graph: &TaskGraphState, requirement: &str) -> bool {
    !graph
        .evidence
        .iter()
        .any(|item| item.requirement == requirement)
}

fn refresh_graph_case(graph: &mut TaskGraphState) {
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
