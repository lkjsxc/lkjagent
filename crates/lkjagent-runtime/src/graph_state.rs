use lkjagent_context::budget::PREFIX_GRAPH_STATE;
use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::policy::ContextPressureLevel;
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
    graph_guard::append_case_guard(&mut state, guard);
    let id = lkjagent_store::graph::open_case(conn, open_case(&state), now)?;
    state.case_id = Some(id);
    lkjagent_store::graph::record_event(
        conn,
        id,
        "owner",
        state.active_node.0,
        state.phase.as_str(),
        "owner message opened graph case",
        now,
    )?;
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
        frame_ref: None,
        event_ref: None,
        confidence: 80,
        satisfies_completion: true,
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

fn open_case(state: &TaskGraphState) -> OpenCase {
    OpenCase {
        objective: state.objective_text().to_string(),
        raw_owner_text: state.objective.raw_owner_message.clone(),
        objective_version: state.objective.version,
        family: state.family.as_str().to_string(),
        subroute: state.subroute.clone(),
        route_reason: state.route_reason.clone(),
        phase: state.phase.as_str().to_string(),
        active_node: state.active_node.0.to_string(),
        plan: state.plan.summary_text(),
        evidence_requirements: state.evidence.requirement_ids(),
        selected_packages: state.context.selected_packages.clone(),
        pending_checks: state.evidence.pending_checks.clone(),
        next_action_class: state.next_action_class.clone(),
        context_pressure: pressure_str(state.context.pressure).to_string(),
    }
}

fn state_from_row(row: GraphCaseRow, evidence: Vec<GraphEvidenceRow>) -> TaskGraphState {
    let raw = if row.raw_owner_text.trim().is_empty() {
        row.objective.clone()
    } else {
        row.raw_owner_text.clone()
    };
    let mut state = initial_state(&raw, Some(row.id));
    state.objective.normalized = row.objective;
    state.objective.version = row.objective_version;
    state.family = family(&row.family);
    state.subroute = row.subroute;
    state.route_reason = row.route_reason;
    state.phase = phase(&row.phase);
    state.status = status(&row.status);
    state.active_node = node_id(&row.active_node);
    state.plan.reason = row.plan;
    state.context.selected_packages = row.selected_packages;
    state.context.pressure = pressure(&row.context_pressure);
    state.next_action_class = row.next_action_class;
    state.evidence = lkjagent_graph::case_evidence::EvidenceState::new(
        row.evidence_requirements,
        row.pending_checks,
    );
    state.evidence.records = evidence.into_iter().map(evidence_from_row).collect();
    state.workspace.touched_paths = state
        .evidence
        .records
        .iter()
        .filter_map(|row| row.path.clone())
        .collect();
    lkjagent_graph::completion::refresh_completion_state(&mut state);
    state
}

fn pressure(value: &str) -> ContextPressureLevel {
    match value {
        "yellow" => ContextPressureLevel::Yellow,
        "orange" => ContextPressureLevel::Orange,
        "red" => ContextPressureLevel::Red,
        "black-invalid" => ContextPressureLevel::BlackInvalid,
        _ => ContextPressureLevel::Green,
    }
}

fn pressure_str(value: ContextPressureLevel) -> &'static str {
    match value {
        ContextPressureLevel::Green => "green",
        ContextPressureLevel::Yellow => "yellow",
        ContextPressureLevel::Orange => "orange",
        ContextPressureLevel::Red => "red",
        ContextPressureLevel::BlackInvalid => "black-invalid",
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

fn idle_state() -> TaskGraphState {
    let mut state = initial_state("no active task", None);
    state.phase = TaskPhase::Waiting;
    state.status = CaseStatus::Waiting;
    state.active_node = GraphNodeId("intake");
    state.plan.reason = "no active graph case".to_string();
    state.evidence.records.clear();
    state.evidence.pending_checks.clear();
    state.completion.pending_checks.clear();
    state
}
