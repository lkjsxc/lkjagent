use lkjagent_context::budget::PREFIX_GRAPH_STATE;
use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::{
    initial_state, render_graph_slice, source_graph, CaseStatus, EvidenceKind, EvidenceRecord,
    GraphNodeId, TaskGraphState, TaskPhase,
};
use lkjagent_protocol::render_graph;
use lkjagent_store::graph::{GraphCaseRow, GraphEvidenceRow, OpenCase};
use lkjagent_tools::control::CompletionGuard;
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::graph_guard;
use crate::graph_parse::{evidence_kind, family, node_id, phase, status};
use crate::prompt::token_estimate;

pub fn open_owner_case(
    conn: &Connection,
    content: &str,
    now: &str,
) -> RuntimeResult<TaskGraphState> {
    open_owner_case_with_guard(conn, content, now, CompletionGuard::None)
}

pub fn open_owner_case_with_guard(
    conn: &Connection,
    content: &str,
    now: &str,
    guard: CompletionGuard,
) -> RuntimeResult<TaskGraphState> {
    let mut state = initial_state(content, None);
    graph_guard::append_plan_guard(&mut state.plan, guard);
    let id = lkjagent_store::graph::open_case(conn, open_case(&state), now)?;
    state.case_id = Some(id);
    let evidence = plan_evidence();
    lkjagent_store::graph::record_evidence(conn, id, &row_from_evidence(&evidence), now)?;
    lkjagent_store::graph::record_event(
        conn,
        id,
        "owner",
        state.active_node.0,
        state.phase.as_str(),
        "owner message opened graph case",
        now,
    )?;
    state.evidence.push(evidence);
    Ok(state)
}

pub fn prefix_graph_state(conn: &Connection) -> RuntimeResult<String> {
    let graph = match lkjagent_store::graph::active_case(conn)? {
        Some(row) => {
            let evidence = lkjagent_store::graph::evidence_for_case(conn, row.id)?;
            render_state(&state_from_row(row, evidence))
        }
        None => render_state(&idle_state()),
    };
    graph_guard::append_store_guard(conn, graph)
}

pub fn graph_notice_frame(state: &TaskGraphState) -> Frame {
    let rendered = render_graph(&render_state(state));
    Frame::new(
        FrameKind::GraphNotice,
        rendered.clone(),
        token_estimate(&rendered),
    )
}

pub fn render_state(state: &TaskGraphState) -> String {
    render_graph_slice(source_graph(), state, PREFIX_GRAPH_STATE)
}

pub fn evidence_record(
    requirement: &str,
    kind: EvidenceKind,
    summary: String,
    path: Option<String>,
) -> EvidenceRecord {
    EvidenceRecord {
        requirement: requirement.to_string(),
        kind,
        summary,
        path,
    }
}

pub fn row_from_evidence(evidence: &EvidenceRecord) -> GraphEvidenceRow {
    GraphEvidenceRow {
        requirement: evidence.requirement.clone(),
        kind: evidence.kind.as_str().to_string(),
        summary: evidence.summary.clone(),
        path: evidence.path.clone(),
    }
}

pub fn status_str(status: CaseStatus) -> &'static str {
    match status {
        CaseStatus::Active => "active",
        CaseStatus::Waiting => "waiting",
        CaseStatus::Closed => "closed",
        CaseStatus::Paused => "paused",
    }
}

fn open_case(state: &TaskGraphState) -> OpenCase<'_> {
    OpenCase {
        objective: &state.objective,
        family: state.family.as_str(),
        phase: state.phase.as_str(),
        active_node: state.active_node.0,
        plan: &state.plan,
        evidence_requirements: &state.evidence_requirements,
        selected_packages: &state.selected_packages,
        pending_checks: &state.pending_checks,
    }
}

fn state_from_row(row: GraphCaseRow, evidence: Vec<GraphEvidenceRow>) -> TaskGraphState {
    TaskGraphState {
        case_id: Some(row.id),
        objective: row.objective,
        family: family(&row.family),
        phase: phase(&row.phase),
        status: status(&row.status),
        active_node: node_id(&row.active_node),
        confidence: 60,
        plan: row.plan,
        risks: Vec::new(),
        candidate_paths: Vec::new(),
        touched_paths: evidence.iter().filter_map(|row| row.path.clone()).collect(),
        selected_packages: row.selected_packages,
        evidence_requirements: row.evidence_requirements,
        evidence: evidence.into_iter().map(evidence_from_row).collect(),
        pending_checks: row.pending_checks,
        recovery: None,
    }
}

fn evidence_from_row(row: GraphEvidenceRow) -> EvidenceRecord {
    evidence_record(
        &row.requirement,
        evidence_kind(&row.kind),
        row.summary,
        row.path,
    )
}

fn plan_evidence() -> EvidenceRecord {
    evidence_record(
        "plan",
        EvidenceKind::Note,
        "harness created initial graph plan".to_string(),
        None,
    )
}

fn idle_state() -> TaskGraphState {
    let mut state = initial_state("no active task", None);
    state.phase = TaskPhase::Waiting;
    state.status = CaseStatus::Waiting;
    state.active_node = GraphNodeId("classify");
    state.plan = "no active graph case".to_string();
    state.evidence.clear();
    state.pending_checks.clear();
    state
}
