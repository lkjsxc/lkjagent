use lkjagent_graph::kernel_events::{CaseEvent, EvidenceLedger, FaultLedger};
use lkjagent_graph::kernel_types::{
    CaseId, CaseLifecycle, CaseState, CompletionGate, Confidence, HardState, Objective, Phase,
    Posture, StateNode, StateTrack, StateVector, ToolIntent, TrackLabel, TrackSource, Weight,
};
use lkjagent_graph::{authorize_tool_intent, compile_context_frame, reduce_case_event};

#[test]
fn maintenance_noop_blocks_completion_until_suppressed() {
    let noop = reduce_case_event(
        &case(),
        &CaseEvent::MaintenanceCycleNoop {
            suppression_created: false,
        },
    );
    let done = authorize_tool_intent(&noop, &intent("agent.done", "done:no-op"));
    assert!(!done.allowed);
    assert!(done.blocked_by.contains(&TrackLabel::MaintenanceNoopRisk));
    assert!(done.preferred_tools.contains(&"memory.save".to_string()));

    let suppressed = reduce_case_event(
        &noop,
        &CaseEvent::MaintenanceCycleNoop {
            suppression_created: true,
        },
    );
    let frame = compile_context_frame(&suppressed);
    assert!(!frame
        .guard_tracks
        .contains(&TrackLabel::MaintenanceNoopRisk));
}

#[test]
fn workspace_memory_claim_requires_workspace_evidence() {
    let state = reduce_case_event(&case(), &CaseEvent::WorkspaceClaimFromMemory);
    let done = authorize_tool_intent(&state, &intent("agent.done", "done:workspace"));
    let frame = compile_context_frame(&state);

    assert!(!done.allowed);
    assert!(done.blocked_by.contains(&TrackLabel::WorkspaceEvidenceRisk));
    assert!(done
        .preferred_tools
        .contains(&"workspace.summary".to_string()));
    assert!(frame
        .selected_context_slices
        .contains(&"workspace evidence rule".to_string()));
}

fn case() -> CaseState {
    CaseState {
        case_id: CaseId("case-guards".to_string()),
        lifecycle: CaseLifecycle::Active,
        hard_state: HardState {
            node: StateNode::Executing,
            phase: Phase::Execute,
            allowed_tools: vec![
                "agent.done".to_string(),
                "memory.save".to_string(),
                "workspace.summary".to_string(),
            ],
            blocked_tools: Vec::new(),
            completion_gates: vec![CompletionGate {
                name: "maintenance-output".to_string(),
                satisfied: true,
            }],
        },
        state_vector: StateVector {
            tracks: vec![completion_track()],
            updated_by: None,
        },
        objective: Objective {
            raw: "maintain workspace".to_string(),
            normalized: "maintenance".to_string(),
        },
        evidence: EvidenceLedger(Vec::new()),
        faults: FaultLedger(Vec::new()),
        repeated_signatures: Vec::new(),
    }
}

fn completion_track() -> StateTrack {
    StateTrack {
        label: TrackLabel::CompletionReadiness,
        posture: Posture::Verifying,
        weight: Weight(0.9),
        confidence: Confidence(0.8),
        source: TrackSource::Completion,
        evidence_gap: None,
        guard: None,
        decay: lkjagent_graph::kernel_types::DecayPolicy::Slow,
        last_updated: None,
    }
}

fn intent(name: &str, signature: &str) -> ToolIntent {
    ToolIntent {
        name: name.to_string(),
        signature: signature.to_string(),
        payload_size: 0,
    }
}
