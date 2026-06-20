use crate::model::{GraphNodeId, TaskFamily, TaskPhase};
use crate::state_track::{StatePosture, StateTrack};

pub fn initial_state_tracks(
    family: TaskFamily,
    active_node: GraphNodeId,
    confidence: u8,
) -> Vec<StateTrack> {
    match family {
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => docs(active_node, confidence),
        TaskFamily::Architecture => architecture(active_node, confidence),
        TaskFamily::Recovery => vec![
            track(
                "recovery",
                "fault-recovery",
                StatePosture::Recovering,
                86,
                confidence,
                TaskPhase::Recovery,
                GraphNodeId("recover"),
                &["recovery evidence"],
            ),
            track(
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
            track(
                "maintenance",
                "maintenance",
                StatePosture::Maintaining,
                64,
                confidence,
                TaskPhase::Maintenance,
                active_node,
                &["maintenance evidence"],
            ),
            track(
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
            track(
                "implementation",
                "implementation",
                StatePosture::Implementing,
                76,
                confidence,
                TaskPhase::Planning,
                active_node,
                &["plan evidence"],
            ),
            track(
                "verification",
                "verification-gates",
                StatePosture::Verifying,
                62,
                60,
                TaskPhase::Verification,
                GraphNodeId("verify"),
                &["test evidence"],
            ),
            track(
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
        track(
            "document-structure",
            "document-structure",
            StatePosture::Structuring,
            88,
            confidence,
            TaskPhase::Planning,
            active_node,
            &["document audit"],
        ),
        track(
            "action-recovery",
            "action-param-reliability",
            StatePosture::Recovering,
            61,
            60,
            TaskPhase::Recovery,
            GraphNodeId("recover"),
            &["normalizer tests"],
        ),
        track(
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
        track(
            "architecture",
            "target-architecture",
            StatePosture::Structuring,
            82,
            confidence,
            TaskPhase::Planning,
            active_node,
            &["design evidence"],
        ),
        track(
            "implementation",
            "implementation-alignment",
            StatePosture::Implementing,
            66,
            60,
            TaskPhase::Execution,
            GraphNodeId("execute"),
            &["code evidence"],
        ),
        track(
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

fn track(
    id: &str,
    label: &str,
    posture: StatePosture,
    intensity: u8,
    confidence: u8,
    phase: TaskPhase,
    active_node: GraphNodeId,
    gaps: &[&str],
) -> StateTrack {
    StateTrack::new(
        id,
        label,
        posture,
        intensity,
        confidence,
        phase,
        active_node,
        gaps,
    )
}
