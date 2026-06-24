use lkjagent_runtime::mode::{decide_turn_authority, TurnAuthorityInput};

#[test]
fn turn_snapshot_carries_case_graph_and_artifact_facts() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        case_id: Some(7),
        graph_node: Some("audit".to_string()),
        graph_phase: Some("verification".to_string()),
        artifact_root: Some("stories/sf".to_string()),
        required_evidence: vec!["artifact-readiness".to_string()],
        missing_evidence: vec!["artifact-readiness".to_string()],
        ..TurnAuthorityInput::default()
    });

    assert_eq!(authority.snapshot.case_id.as_deref(), Some("7"));
    assert_eq!(authority.snapshot.graph_node.as_deref(), Some("audit"));
    assert_eq!(
        authority.snapshot.graph_phase.as_deref(),
        Some("verification")
    );
    assert_eq!(
        authority.snapshot.active_artifact.as_deref(),
        Some("stories/sf")
    );
    assert_eq!(
        authority.snapshot.missing_evidence,
        vec!["artifact-readiness"]
    );
}
