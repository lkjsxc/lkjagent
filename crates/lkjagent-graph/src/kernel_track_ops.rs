use crate::kernel_types::*;

const DEFAULT_GUARD_THRESHOLD: f32 = 0.80;

pub(crate) fn tracks_at(vector: &StateVector, minimum: f32) -> Vec<StateTrack> {
    vector
        .tracks
        .iter()
        .filter(|track| track.weight.0 >= minimum)
        .cloned()
        .collect()
}

pub(crate) fn weight(vector: &StateVector, label: TrackLabel) -> f32 {
    vector
        .tracks
        .iter()
        .find(|track| track.label == label)
        .map_or(0.0, |track| track.weight.0)
}

pub(crate) fn bump(vector: &mut StateVector, label: TrackLabel, by: f32) {
    mutate(vector, label, |old| (old + by).min(1.0));
}

pub(crate) fn lower(vector: &mut StateVector, label: TrackLabel, by: f32) {
    mutate(vector, label, |old| (old - by).max(0.0));
}

pub(crate) fn floor(vector: &mut StateVector, label: TrackLabel, value: f32) {
    mutate(vector, label, |old| old.max(value));
}

pub(crate) fn threshold(track: &StateTrack) -> f32 {
    match track.label {
        TrackLabel::ArtifactDrift => 0.75,
        TrackLabel::QueueInterruption | TrackLabel::MockContentRisk => 0.70,
        TrackLabel::ModelSpecificNaming
        | TrackLabel::StructureConnectivity
        | TrackLabel::RepeatedActionRisk => 0.60,
        _ => DEFAULT_GUARD_THRESHOLD,
    }
}

fn mutate(vector: &mut StateVector, label: TrackLabel, f: impl FnOnce(f32) -> f32) {
    let track = ensure_track(vector, label);
    track.weight = Weight(f(track.weight.0));
}

fn ensure_track(vector: &mut StateVector, label: TrackLabel) -> &mut StateTrack {
    let index = match vector.tracks.iter().position(|track| track.label == label) {
        Some(index) => index,
        None => {
            vector.tracks.push(default_track(label));
            vector.tracks.len().saturating_sub(1)
        }
    };
    &mut vector.tracks[index]
}

fn default_track(label: TrackLabel) -> StateTrack {
    StateTrack {
        label,
        posture: Posture::Observing,
        weight: Weight(0.0),
        confidence: Confidence(0.70),
        source: TrackSource::Runtime,
        evidence_gap: None,
        guard: default_guard(label),
        decay: DecayPolicy::Slow,
        last_updated: None,
    }
}

fn default_guard(label: TrackLabel) -> Option<GuardPolicy> {
    match label {
        TrackLabel::MockContentRisk
        | TrackLabel::ModelSpecificNaming
        | TrackLabel::StructureConnectivity => Some(GuardPolicy::BlockCompletion),
        TrackLabel::ParseRecovery => Some(GuardPolicy::RestrictLargePayload),
        TrackLabel::ArtifactDrift => Some(GuardPolicy::BlockArtifactMutation),
        TrackLabel::ContextPressure | TrackLabel::ContextSnapshotMismatch => {
            Some(GuardPolicy::BlockMutation)
        }
        TrackLabel::QueueInterruption => Some(GuardPolicy::RequireQueueClassification),
        TrackLabel::RepeatedActionRisk => Some(GuardPolicy::BlockRepeatedSignature),
        _ => None,
    }
}
