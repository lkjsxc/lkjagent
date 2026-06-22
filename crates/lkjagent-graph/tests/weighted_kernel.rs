use lkjagent_graph::kernel_events::{CaseEvent, EvidenceLedger, Fault, FaultLedger};
use lkjagent_graph::kernel_types::{
    CaseId, CaseLifecycle, CaseState, CompletionGate, Confidence, HardState, Objective, Phase,
    Posture, StateNode, StateTrack, StateVector, ToolIntent, TrackLabel, TrackSource, Weight,
};
use lkjagent_graph::{
    authorize_tool_intent, check_completion_gates, compile_context_frame, compile_prompt_frame,
    compile_prompt_frame_from_context, reduce_case_event, required_context_slices_from_tracks,
    select_recovery, update_state_vector, PromptMode,
};

#[test]
fn parse_faults_raise_guard_and_block_large_payloads() {
    let state = reduce_case_event(&case(), &CaseEvent::ParseFault { consecutive: 3 });
    let intent = intent("fs.batch_write", "batch:large", 3);
    let auth = authorize_tool_intent(&state, &intent);

    assert!(!auth.allowed);
    assert!(auth.blocked_by.contains(&TrackLabel::ParseRecovery));
    assert!(auth.preferred_tools.contains(&"fs.write".to_string()));
}

#[test]
fn artifact_drift_blocks_artifact_continuation() {
    let state = reduce_case_event(
        &case(),
        &CaseEvent::ArtifactObjectiveMismatch {
            reason: "bread path under Japanese cookbook".to_string(),
        },
    );
    let next = authorize_tool_intent(&state, &intent("artifact.next", "next", 0));
    let apply = authorize_tool_intent(&state, &intent("artifact.apply", "apply", 1));

    assert!(!next.allowed);
    assert!(!apply.allowed);
    assert!(next.blocked_by.contains(&TrackLabel::ArtifactDrift));
}

#[test]
fn queue_interruption_preserves_mutation_block_until_classified() {
    let interrupted = reduce_case_event(&case(), &CaseEvent::OwnerTaskArrived);
    let blocked = authorize_tool_intent(&interrupted, &intent("fs.write", "write:a", 1));
    let classified = reduce_case_event(&interrupted, &CaseEvent::QueueClassified);
    let allowed = authorize_tool_intent(&classified, &intent("fs.write", "write:b", 1));

    assert!(!blocked.allowed);
    assert!(blocked.blocked_by.contains(&TrackLabel::QueueInterruption));
    assert!(allowed.allowed);
}

#[test]
fn documentation_quality_guards_block_completion_and_memory() {
    let mock = reduce_case_event(&case(), &CaseEvent::MockContentAudit { passed: false });
    let done = authorize_tool_intent(&mock, &intent("agent.done", "done:mock", 0));
    assert!(!done.allowed);
    assert!(done.blocked_by.contains(&TrackLabel::MockContentRisk));

    let named = reduce_case_event(&case(), &CaseEvent::ModelNameAudit { passed: false });
    let memory = authorize_tool_intent(&named, &intent("memory.save", "memory:named", 0));
    assert!(!memory.allowed);
    assert!(memory.blocked_by.contains(&TrackLabel::ModelSpecificNaming));

    let relation = reduce_case_event(&case(), &CaseEvent::RelationAudit { passed: false });
    let done = authorize_tool_intent(&relation, &intent("agent.done", "done:relation", 0));
    assert!(!done.allowed);
    assert!(done.blocked_by.contains(&TrackLabel::StructureConnectivity));
}

#[test]
fn completion_is_hard_gated_even_when_ready_track_is_high() {
    let mut state = case();
    state.hard_state.completion_gates = vec![CompletionGate {
        name: "artifact-readiness".to_string(),
        satisfied: false,
    }];
    state.state_vector =
        update_state_vector(&state.state_vector, &CaseEvent::CompletionEvidenceReady);
    state.state_vector =
        update_state_vector(&state.state_vector, &CaseEvent::CompletionEvidenceReady);
    state.state_vector =
        update_state_vector(&state.state_vector, &CaseEvent::CompletionEvidenceReady);

    let done = authorize_tool_intent(&state, &intent("agent.done", "done", 0));
    let gates = check_completion_gates(&state);

    assert!(!done.allowed);
    assert!(!gates.allowed);
}

#[test]
fn prompt_frame_compiles_state_guards_and_next_action() {
    let state = reduce_case_event(&case(), &CaseEvent::ModelNameAudit { passed: false });
    let frame = compile_prompt_frame(&state);

    assert_eq!(frame.case_id, "case-1");
    assert_eq!(frame.mode, PromptMode::Expansion);
    assert!(frame
        .dominant_guards
        .contains(&TrackLabel::ModelSpecificNaming));
    assert!(frame
        .context_slices
        .contains(&"model-name sanitizer".to_string()));
    assert_eq!(
        frame.next_action_recommendation,
        "run sanitizer or model-name audit repair"
    );
}

#[test]
fn context_frame_compiles_before_prompt_frame() {
    let state = reduce_case_event(&case(), &CaseEvent::ParseFault { consecutive: 3 });
    let context = compile_context_frame(&state);
    let prompt = compile_prompt_frame_from_context(&context);

    assert_eq!(context.case_id, "case-1");
    assert!(context.guard_tracks.contains(&TrackLabel::ParseRecovery));
    assert!(context
        .selected_context_slices
        .contains(&"action grammar".to_string()));
    assert_eq!(prompt.mode, PromptMode::Recovery);
    assert_eq!(prompt.context_slices, context.selected_context_slices);
}

#[test]
fn dominant_tracks_select_context_and_recovery_steps() {
    let state = reduce_case_event(&case(), &CaseEvent::ParseFault { consecutive: 3 });
    let slices = required_context_slices_from_tracks(&state.state_vector);
    let recovery = select_recovery(&Fault::ParserFault);

    assert!(slices.contains(&"action grammar".to_string()));
    assert!(recovery.contains(&"one small action".to_string()));
}

fn case() -> CaseState {
    CaseState {
        case_id: CaseId("case-1".to_string()),
        lifecycle: CaseLifecycle::Active,
        hard_state: HardState {
            node: StateNode::Executing,
            phase: Phase::Execute,
            allowed_tools: vec![
                "fs.write".to_string(),
                "fs.batch_write".to_string(),
                "artifact.next".to_string(),
                "artifact.apply".to_string(),
                "memory.save".to_string(),
                "agent.done".to_string(),
            ],
            blocked_tools: Vec::new(),
            completion_gates: vec![CompletionGate {
                name: "objective-match".to_string(),
                satisfied: true,
            }],
        },
        state_vector: StateVector {
            tracks: vec![completion_track()],
            updated_by: None,
        },
        objective: Objective {
            raw: "Create a Japanese cookbook".to_string(),
            normalized: "Japanese-food cookbook".to_string(),
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
        weight: Weight(0.0),
        confidence: Confidence(0.8),
        source: TrackSource::Completion,
        evidence_gap: Some("artifact-readiness".to_string()),
        guard: None,
        decay: lkjagent_graph::kernel_types::DecayPolicy::Slow,
        last_updated: None,
    }
}

fn intent(name: &str, signature: &str, payload_size: usize) -> ToolIntent {
    ToolIntent {
        name: name.to_string(),
        signature: signature.to_string(),
        payload_size,
    }
}
