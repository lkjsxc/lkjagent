use crate::kernel_events::{AuditResult, CaseEvent, Fault, ToolObservation};
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

pub fn apply_audit_result(vector: &StateVector, audit: &AuditResult) -> StateVector {
    match audit.kind.as_str() {
        "doc.audit" => update_state_vector(
            vector,
            &CaseEvent::DocAudit {
                passed: audit.passed,
            },
        ),
        "artifact.audit" => update_state_vector(
            vector,
            &CaseEvent::ArtifactAudit {
                passed: audit.passed,
            },
        ),
        _ if audit.passed => vector.clone(),
        _ => update_state_vector(
            vector,
            &CaseEvent::ArtifactObjectiveMismatch {
                reason: audit.kind.clone(),
            },
        ),
    }
}

pub fn apply_tool_observation(vector: &StateVector, observation: &ToolObservation) -> StateVector {
    if observation.succeeded {
        update_state_vector(vector, &CaseEvent::ParsedAction)
    } else {
        update_state_vector(
            vector,
            &CaseEvent::ToolParameterFault {
                expected: observation.tool.clone(),
                received: "failed observation".to_string(),
            },
        )
    }
}

pub fn classify_fault(text: &str) -> Fault {
    let lower_text = text.to_ascii_lowercase();
    if lower_text.contains("parse") {
        Fault::ParserFault
    } else if lower_text.contains("parameter") || lower_text.contains("schema") {
        Fault::ToolParameterFault
    } else if lower_text.contains("drift") {
        Fault::ArtifactDrift
    } else if lower_text.contains("repeat") {
        Fault::RepeatedActionRefusal
    } else if lower_text.contains("queue") {
        Fault::QueueInterruption
    } else {
        Fault::ToolExecutionFault
    }
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
