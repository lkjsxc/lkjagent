use lkjagent_graph::case_evidence::EvidenceState;
use lkjagent_graph::policy::ContextPressureLevel;
use lkjagent_graph::{
    completion, initial_state, CaseStatus, EvidenceRecord, GraphNodeId, TaskGraphState, TaskPhase,
};
use lkjagent_store::graph::{GraphCaseRow, GraphEvidenceRow};

use crate::graph_parse::{evidence_kind, family, node_id, phase, status};
use crate::graph_state_tracks::attach_track_rows;

type TrackRow = lkjagent_store::graph::state_tracks::GraphStateTrackRow;

pub fn state_from_row(
    row: GraphCaseRow,
    evidence: Vec<GraphEvidenceRow>,
    tracks: Vec<TrackRow>,
) -> TaskGraphState {
    let raw = if row.raw_owner_text.trim().is_empty() {
        row.objective.clone()
    } else {
        row.raw_owner_text.clone()
    };
    let mut state = initial_state(&raw, Some(row.id));
    state.objective.normalized = row.objective;
    state.objective.envelope.normalized_objective = state.objective.normalized.clone();
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
    state.evidence = EvidenceState::new(row.evidence_requirements, row.pending_checks);
    state.evidence.records = evidence.into_iter().map(evidence_from_row).collect();
    attach_track_rows(&mut state, tracks);
    state.workspace.touched_paths = state
        .evidence
        .records
        .iter()
        .filter_map(|row| row.path.clone())
        .collect();
    completion::refresh_completion_state(&mut state);
    state
}

pub fn idle_state() -> TaskGraphState {
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

fn pressure(value: &str) -> ContextPressureLevel {
    match value {
        "yellow" => ContextPressureLevel::Yellow,
        "orange" => ContextPressureLevel::Orange,
        "red" => ContextPressureLevel::Red,
        "black-invalid" => ContextPressureLevel::BlackInvalid,
        _ => ContextPressureLevel::Green,
    }
}

fn evidence_from_row(row: GraphEvidenceRow) -> EvidenceRecord {
    EvidenceRecord {
        requirement: row.requirement,
        kind: evidence_kind(&row.kind),
        summary: row.summary,
        path: row.path,
        frame_ref: None,
        event_ref: None,
        confidence: 80,
        satisfies_completion: true,
    }
}
