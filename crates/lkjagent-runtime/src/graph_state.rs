use lkjagent_context::budget::PREFIX_GRAPH_STATE;
use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_graph::{
    initial_state, render_graph_slice, source_graph, CaseStatus, EvidenceKind, EvidenceRecord,
    TaskGraphState,
};
use lkjagent_protocol::render_graph;
use lkjagent_store::graph::{GraphEvidenceRow, OpenCase};
use lkjagent_tools::control::CompletionGuard;
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::graph_guard;
use crate::graph_state_row::{idle_state, state_from_row};
use crate::graph_state_tracks::replace_state_tracks;
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
    replace_state_tracks(conn, id, &state, now)?;
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
    let budget = prefix_body_budget();
    let graph = match lkjagent_store::graph::active_case(conn)? {
        Some(row) => {
            let evidence = lkjagent_store::graph::evidence_for_case(conn, row.id)?;
            let tracks = lkjagent_store::graph::state_tracks::state_tracks_for_case(conn, row.id)?;
            render_state_budgeted(&state_from_row(row, evidence, tracks), budget)
        }
        None => render_state_budgeted(&idle_state(), budget),
    };
    let guarded = graph_guard::append_store_guard(conn, graph)?;
    Ok(fit_prefix_body(&guarded, budget))
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
    render_state_budgeted(state, PREFIX_GRAPH_STATE)
}

fn render_state_budgeted(state: &TaskGraphState, budget: usize) -> String {
    render_graph_slice(source_graph(), state, budget)
}

fn prefix_body_budget() -> usize {
    PREFIX_GRAPH_STATE.saturating_sub(token_estimate("## graph state\n"))
}

fn fit_prefix_body(text: &str, budget: usize) -> String {
    if token_estimate(text) <= budget {
        return text.to_string();
    }
    let marker = "\n[graph prefix narrowed]";
    let mut out = String::new();
    for ch in text.chars() {
        let candidate = format!("{out}{ch}{marker}");
        if token_estimate(&candidate) > budget {
            break;
        }
        out.push(ch);
    }
    format!("{out}{marker}")
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

fn pressure_str(value: lkjagent_graph::policy::ContextPressureLevel) -> &'static str {
    match value {
        lkjagent_graph::policy::ContextPressureLevel::Green => "green",
        lkjagent_graph::policy::ContextPressureLevel::Yellow => "yellow",
        lkjagent_graph::policy::ContextPressureLevel::Orange => "orange",
        lkjagent_graph::policy::ContextPressureLevel::Red => "red",
        lkjagent_graph::policy::ContextPressureLevel::BlackInvalid => "black-invalid",
    }
}
