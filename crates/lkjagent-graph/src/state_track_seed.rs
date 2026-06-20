use crate::model::{GraphNodeId, TaskFamily, TaskPhase};
use crate::state_track::{StatePosture, StateTrack, StateTrackInput};

struct TrackSeed<'a> {
    id: &'a str,
    label: &'a str,
    posture: StatePosture,
    intensity: u8,
    confidence: u8,
    phase: TaskPhase,
    active_node: GraphNodeId,
    gaps: &'a [&'a str],
}

impl TrackSeed<'_> {
    fn into_track(self) -> StateTrack {
        StateTrack::new(StateTrackInput {
            id: self.id,
            label: self.label,
            posture: self.posture,
            intensity: self.intensity,
            confidence: self.confidence,
            phase: self.phase,
            active_node: self.active_node,
            gaps: self.gaps,
        })
    }
}

macro_rules! seed {
    ($id:expr, $label:expr, $posture:expr, $intensity:expr, $confidence:expr, $phase:expr, $node:expr, $gaps:expr $(,)?) => {
        TrackSeed {
            id: $id,
            label: $label,
            posture: $posture,
            intensity: $intensity,
            confidence: $confidence,
            phase: $phase,
            active_node: $node,
            gaps: $gaps,
        }
        .into_track()
    };
}
pub fn initial_state_tracks(
    family: TaskFamily,
    active_node: GraphNodeId,
    confidence: u8,
) -> Vec<StateTrack> {
    match family {
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => docs(active_node, confidence),
        TaskFamily::Architecture => architecture(active_node, confidence),
        TaskFamily::Recovery => vec![
            seed!(
                "recovery",
                "fault-recovery",
                StatePosture::Recovering,
                86,
                confidence,
                TaskPhase::Recovery,
                GraphNodeId("recover"),
                &["recovery evidence"],
            ),
            seed!(
                "inspection",
                "state-inspection",
                StatePosture::Exploring,
                55,
                60,
                TaskPhase::Context,
                GraphNodeId("survey"),
                &["current state"],
            ),
        ],
        TaskFamily::IdleMaintenance | TaskFamily::Maintenance => vec![
            seed!(
                "maintenance",
                "maintenance",
                StatePosture::Maintaining,
                64,
                confidence,
                TaskPhase::Maintenance,
                active_node,
                &["maintenance evidence"],
            ),
            seed!(
                "verification",
                "verification-gates",
                StatePosture::Verifying,
                52,
                55,
                TaskPhase::Verification,
                GraphNodeId("verify"),
                &["check evidence"],
            ),
        ],
        _ => vec![
            seed!(
                "implementation",
                "implementation",
                StatePosture::Implementing,
                76,
                confidence,
                TaskPhase::Planning,
                active_node,
                &["plan evidence"],
            ),
            seed!(
                "verification",
                "verification-gates",
                StatePosture::Verifying,
                62,
                60,
                TaskPhase::Verification,
                GraphNodeId("verify"),
                &["test evidence"],
            ),
            seed!(
                "recovery",
                "action-recovery",
                StatePosture::Recovering,
                46,
                50,
                TaskPhase::Recovery,
                GraphNodeId("recover"),
                &["fault evidence"],
            ),
        ],
    }
}

fn docs(active_node: GraphNodeId, confidence: u8) -> Vec<StateTrack> {
    vec![
        seed!(
            "document-structure",
            "document-structure",
            StatePosture::Structuring,
            88,
            confidence,
            TaskPhase::Planning,
            active_node,
            &["document audit"],
        ),
        seed!(
            "action-recovery",
            "action-param-reliability",
            StatePosture::Recovering,
            61,
            60,
            TaskPhase::Recovery,
            GraphNodeId("recover"),
            &["normalizer tests"],
        ),
        seed!(
            "observability",
            "observability-ledger",
            StatePosture::Exploring,
            48,
            55,
            TaskPhase::Planning,
            active_node,
            &["status evidence"],
        ),
    ]
}

fn architecture(active_node: GraphNodeId, confidence: u8) -> Vec<StateTrack> {
    vec![
        seed!(
            "architecture",
            "target-architecture",
            StatePosture::Structuring,
            82,
            confidence,
            TaskPhase::Planning,
            active_node,
            &["design evidence"],
        ),
        seed!(
            "implementation",
            "implementation-alignment",
            StatePosture::Implementing,
            66,
            60,
            TaskPhase::Execution,
            GraphNodeId("execute"),
            &["code evidence"],
        ),
        seed!(
            "verification",
            "verification-gates",
            StatePosture::Verifying,
            58,
            60,
            TaskPhase::Verification,
            GraphNodeId("verify"),
            &["test evidence"],
        ),
    ]
}
