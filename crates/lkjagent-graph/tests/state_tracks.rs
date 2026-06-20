use lkjagent_graph::{
    initial_state, promote_recovery_track, ranked_state_tracks, render_graph_slice, source_graph,
    GraphNodeId, StatePosture, StateTrack, TaskPhase,
};

#[test]
fn owner_message_creates_multiple_candidate_tracks() {
    let state = initial_state(
        "lkjagent docs should recover from unknown params path and show progress.",
        Some(1),
    );

    assert!(state.state_tracks.len() >= 2);
    assert_eq!(
        state.objective.envelope.candidate_tracks,
        state.state_tracks
    );
    assert_ne!(
        state.objective.normalized,
        state.objective.raw_owner_message
    );
    assert!(state.objective.normalized.contains("Improve lkjagent"));
}

#[test]
fn state_tracks_are_ranked_by_intensity_recency_gap() {
    let stale = track("stale", 95, TaskPhase::Execution, 1, &[]);
    let urgent = track("urgent", 70, TaskPhase::Recovery, 10, &["fault", "test"]);
    let ranked = ranked_state_tracks(&[stale, urgent], 10, 80);

    assert_eq!(ranked[0].track.label, "urgent");
}

#[test]
fn closed_track_does_not_dominate_active_track() {
    let closed = track("closed", 100, TaskPhase::Closed, 10, &["done"]);
    let active = track("active", 75, TaskPhase::Execution, 10, &["evidence"]);
    let ranked = ranked_state_tracks(&[closed, active], 10, 80);

    assert_eq!(ranked[0].track.label, "active");
}

#[test]
fn recovery_track_rises_after_parse_fault() {
    let mut state = initial_state("fix parser bug", Some(2));

    promote_recovery_track(
        &mut state.state_tracks,
        "parse-recovery",
        GraphNodeId("recover-parse"),
        TaskPhase::Recovery,
    );
    let rendered = render_graph_slice(source_graph(), &state, 900);

    assert!(rendered.contains("Active states: 1. Recovering"));
    assert!(rendered.contains("parse-recovery"));
}

fn track(label: &str, intensity: u8, phase: TaskPhase, turn: u64, gaps: &[&str]) -> StateTrack {
    let mut track = StateTrack::new(
        label,
        label,
        StatePosture::Implementing,
        intensity,
        70,
        phase,
        GraphNodeId("execute"),
        gaps,
    );
    track.last_update_turn = Some(turn);
    track
}
