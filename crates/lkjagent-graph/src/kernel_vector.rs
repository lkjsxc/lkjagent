use crate::kernel_events::CaseEvent;
use crate::kernel_track_ops::{bump, floor, lower, threshold, tracks_at, weight};
use crate::kernel_types::*;

const GUARD: f32 = 0.80;

pub fn reduce_case_event(state: &CaseState, event: &CaseEvent) -> CaseState {
    let mut next = state.clone();
    next.state_vector = update_state_vector(&state.state_vector, event);
    if let CaseEvent::RepeatedInvalidAction { signature } = event {
        if !next.repeated_signatures.contains(signature) {
            next.repeated_signatures.push(signature.clone());
        }
    }
    next.hard_state.node = select_next_state(&next);
    next
}

pub fn update_state_vector(vector: &StateVector, event: &CaseEvent) -> StateVector {
    let mut next = vector.clone();
    match event {
        CaseEvent::ParseFault { consecutive } => parse_fault(&mut next, *consecutive),
        CaseEvent::ParsedAction => lower(&mut next, TrackLabel::ParseRecovery, 0.25),
        CaseEvent::ToolParameterFault { .. } => {
            bump(&mut next, TrackLabel::ActionParamReliability, 0.30);
        }
        CaseEvent::RepeatedInvalidAction { .. } => {
            floor(&mut next, TrackLabel::RepeatedActionRisk, 0.85);
        }
        CaseEvent::DocAudit { passed } => doc_audit(&mut next, *passed),
        CaseEvent::RelationAudit { passed } => audit_track(
            &mut next,
            TrackLabel::StructureConnectivity,
            *passed,
            0.80,
            0.70,
        ),
        CaseEvent::MockContentAudit { passed } => {
            audit_track(&mut next, TrackLabel::MockContentRisk, *passed, 0.85, 0.80)
        }
        CaseEvent::ModelNameAudit { passed } => audit_track(
            &mut next,
            TrackLabel::ModelSpecificNaming,
            *passed,
            0.80,
            0.80,
        ),
        CaseEvent::ArtifactObjectiveMismatch { .. } => artifact_mismatch(&mut next),
        CaseEvent::ArtifactAudit { passed } => artifact_audit(&mut next, *passed),
        CaseEvent::ContextUsage { hard } => {
            floor(
                &mut next,
                TrackLabel::ContextPressure,
                if *hard { 0.90 } else { 0.55 },
            );
        }
        CaseEvent::PostCompaction { matched } => compaction(&mut next, *matched),
        CaseEvent::OwnerTaskArrived => floor(&mut next, TrackLabel::QueueInterruption, 0.75),
        CaseEvent::QueueClassified => lower(&mut next, TrackLabel::QueueInterruption, 0.50),
        CaseEvent::CompletionEvidenceReady => {
            bump(&mut next, TrackLabel::CompletionReadiness, 0.35)
        }
    }
    next.updated_by = Some(format!("{:?}", event));
    next
}

pub fn dominant_tracks(vector: &StateVector) -> Vec<StateTrack> {
    tracks_at(vector, GUARD)
}

pub fn guard_tracks(vector: &StateVector) -> Vec<StateTrack> {
    vector
        .tracks
        .iter()
        .filter(|track| track.guard.is_some() && track.weight.0 >= threshold(track))
        .cloned()
        .collect()
}

pub fn promotion_decision(vector: &StateVector) -> Option<StateNode> {
    if weight(vector, TrackLabel::ContextPressure) >= 0.85
        || weight(vector, TrackLabel::ContextSnapshotMismatch) >= 0.80
    {
        Some(StateNode::Compacting)
    } else if weight(vector, TrackLabel::ArtifactDrift) >= 0.75
        || weight(vector, TrackLabel::ParseRecovery) >= 0.80
    {
        Some(StateNode::Recovering)
    } else if weight(vector, TrackLabel::QueueInterruption) >= 0.70 {
        Some(StateNode::Intake)
    } else {
        None
    }
}

pub fn select_next_state(state: &CaseState) -> StateNode {
    promotion_decision(&state.state_vector).unwrap_or(state.hard_state.node)
}

fn parse_fault(vector: &mut StateVector, consecutive: u8) {
    bump(vector, TrackLabel::ParseRecovery, 0.25);
    bump(vector, TrackLabel::ActionParamReliability, 0.10);
    lower(vector, TrackLabel::CompletionReadiness, 0.20);
    if consecutive >= 3 {
        floor(vector, TrackLabel::ParseRecovery, 0.90);
    }
}

fn doc_audit(vector: &mut StateVector, passed: bool) {
    if passed {
        lower(vector, TrackLabel::DocumentStructure, 0.50);
    } else {
        floor(vector, TrackLabel::DocumentStructure, 0.85);
        lower(vector, TrackLabel::ArtifactReadiness, 0.40);
    }
}

fn audit_track(
    vector: &mut StateVector,
    label: TrackLabel,
    passed: bool,
    fail_floor: f32,
    pass_lower: f32,
) {
    if passed {
        lower(vector, label, pass_lower);
    } else {
        floor(vector, label, fail_floor);
    }
}

fn artifact_mismatch(vector: &mut StateVector) {
    floor(vector, TrackLabel::ArtifactDrift, 0.90);
    lower(vector, TrackLabel::ArtifactReadiness, 0.60);
}

fn artifact_audit(vector: &mut StateVector, passed: bool) {
    if passed {
        lower(vector, TrackLabel::ArtifactDrift, 0.60);
        bump(vector, TrackLabel::ArtifactReadiness, 0.35);
    } else {
        floor(vector, TrackLabel::ArtifactReadiness, 0.20);
    }
}

fn compaction(vector: &mut StateVector, matched: bool) {
    if matched {
        lower(vector, TrackLabel::ContextPressure, 0.70);
    } else {
        floor(vector, TrackLabel::ContextSnapshotMismatch, 0.90);
    }
}
