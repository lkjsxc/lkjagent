use lkjagent_context::assemble::append_frame;
use lkjagent_graph::case_plan::{PlanStep, StepId, StepStatus};
use lkjagent_graph::{
    completion::refresh_completion_state, completion_decision, EvidenceKind, GraphNodeId,
    TaskGraphState, TaskPhase, TransitionDecision,
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
        ensure_scaffold_plan(conn, now, graph, summary, path)?;
        for (requirement, kind) in [
            ("plan", EvidenceKind::Plan),
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
                graph.evidence.records.push(evidence);
            }
        }
        graph.evidence.pending_checks.clear();
        graph.completion.pending_checks.clear();
        refresh_completion_state(graph);
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

fn ensure_scaffold_plan(
    conn: &Connection,
    now: &str,
    graph: &mut TaskGraphState,
    summary: &str,
    path: Option<&str>,
) -> RuntimeResult<()> {
    if graph.plan.ready {
        return Ok(());
    }
    graph.plan.reason = "deterministic counted scaffold".to_string();
    graph.plan.ready = true;
    graph.plan.steps = vec![PlanStep {
        id: StepId("scaffold-1".to_string()),
        title: "create and audit counted document scaffold".to_string(),
        rationale: summary.to_string(),
        status: StepStatus::Done,
        node: GraphNodeId("document"),
        target_paths: path.map(str::to_string).into_iter().collect(),
        required_evidence: vec!["document-structure".to_string()],
        verification: Vec::new(),
    }];
    let Some(case_id) = graph.case_id else {
        return Ok(());
    };
    let rows = vec![lkjagent_store::graph::plan::GraphPlanStepRow {
        case_id,
        step_id: "scaffold-1".to_string(),
        title: "create and audit counted document scaffold".to_string(),
        rationale: summary.to_string(),
        status: "done".to_string(),
        node: "document".to_string(),
        target_paths: path.map(str::to_string).into_iter().collect(),
        checks: vec!["document audit".to_string()],
        sort_order: 0,
    }];
    lkjagent_store::graph::plan::replace_plan_steps(conn, case_id, &rows, now)?;
    Ok(())
}

fn missing_scaffold_evidence(graph: &TaskGraphState, requirement: &str) -> bool {
    !graph
        .evidence
        .records
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
