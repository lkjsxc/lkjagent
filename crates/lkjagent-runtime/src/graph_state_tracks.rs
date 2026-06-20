use lkjagent_graph::{
    ranked_state_tracks, score_decimal, StatePosture, StateTrack, StateTrackId, TaskGraphState,
};
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::graph_parse::{node_id, phase};
use crate::step::GraphStateTrackEffect;

type StoreTrackRow = lkjagent_store::graph::state_tracks::GraphStateTrackRow;

pub fn replace_state_tracks(
    conn: &Connection,
    case_id: i64,
    state: &TaskGraphState,
    now: &str,
) -> RuntimeResult<()> {
    let rows = store_track_rows(graph_track_effects(state));
    lkjagent_store::graph::state_tracks::replace_state_tracks(conn, case_id, &rows, now)?;
    Ok(())
}

pub fn attach_track_rows(state: &mut TaskGraphState, tracks: Vec<StoreTrackRow>) {
    if tracks.is_empty() {
        return;
    }
    state.state_tracks = tracks.into_iter().map(track_from_row).collect();
    state.objective.attach_tracks(&state.state_tracks);
}

pub fn graph_track_effects(state: &TaskGraphState) -> Vec<GraphStateTrackEffect> {
    ranked_state_tracks(&state.state_tracks, 0, 80)
        .into_iter()
        .map(|item| GraphStateTrackEffect {
            track_id: item.track.id.0,
            label: item.track.label,
            posture: item.track.posture.as_str().to_string(),
            intensity: item.track.intensity,
            confidence: item.track.confidence,
            phase: item.track.phase.as_str().to_string(),
            active_node: item.track.active_node.0.to_string(),
            evidence_gap: item.track.evidence_gap,
            next_affordances: item.track.next_affordances,
            risk: item.track.risk,
            last_update_turn: item.track.last_update_turn,
            rank_score: item.rank_score,
        })
        .collect()
}

pub fn store_track_rows(tracks: Vec<GraphStateTrackEffect>) -> Vec<StoreTrackRow> {
    tracks
        .into_iter()
        .map(|track| StoreTrackRow {
            track_id: track.track_id,
            label: track.label,
            posture: track.posture,
            intensity: track.intensity,
            confidence: track.confidence,
            phase: track.phase,
            active_node: track.active_node,
            evidence_gap: track.evidence_gap,
            next_affordances: track.next_affordances,
            risk: track.risk,
            last_update_turn: track.last_update_turn,
            rank_score: track.rank_score,
        })
        .collect()
}

pub fn format_state_track_row(rank: usize, row: &StoreTrackRow) -> String {
    let gap = row.evidence_gap.first().map_or("none", String::as_str);
    format!(
        "{rank}. {} {} {} phase={} gap={}",
        row.posture,
        score_decimal(row.rank_score),
        row.label,
        row.phase,
        gap
    )
}

fn track_from_row(row: StoreTrackRow) -> StateTrack {
    StateTrack {
        id: StateTrackId(row.track_id),
        label: row.label,
        posture: posture(&row.posture),
        intensity: row.intensity,
        confidence: row.confidence,
        phase: phase(&row.phase),
        active_node: node_id(&row.active_node),
        evidence_gap: row.evidence_gap,
        next_affordances: row.next_affordances,
        risk: row.risk,
        last_update_turn: row.last_update_turn,
    }
}

fn posture(value: &str) -> StatePosture {
    match value {
        "Structuring" => StatePosture::Structuring,
        "Implementing" => StatePosture::Implementing,
        "Verifying" => StatePosture::Verifying,
        "Recovering" => StatePosture::Recovering,
        "Waiting" => StatePosture::Waiting,
        "Maintaining" => StatePosture::Maintaining,
        "Closing" => StatePosture::Closing,
        _ => StatePosture::Exploring,
    }
}
