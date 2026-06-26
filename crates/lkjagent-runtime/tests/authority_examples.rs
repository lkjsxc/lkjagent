use lkjagent_protocol::parse_completion;
use lkjagent_runtime::mode::{decide_turn_authority, EndpointDecision, TurnAuthorityInput};
use lkjagent_tools::dispatch::registry_valid_example;

#[test]
fn rendered_model_examples_parse_for_call_model_modes() {
    for input in call_model_inputs() {
        let authority = decide_turn_authority(input);
        assert_eq!(authority.endpoint_decision, EndpointDecision::CallModel);
        assert!(parse_completion(&authority.valid_example).is_ok());
    }
}

#[test]
fn owner_missing_plan_prefers_graph_plan_example() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        artifact_root: Some("stories/chronos-fracture".to_string()),
        missing_evidence: vec!["plan".to_string(), "document-structure".to_string()],
        ..TurnAuthorityInput::default()
    });

    assert!(authority.valid_example.contains("<tool>graph.plan</tool>"));
    assert!(authority.valid_example.contains("stories/chronos-fracture"));
    assert!(parse_completion(&authority.valid_example).is_ok());
}

#[test]
fn owner_missing_document_structure_prefers_artifact_apply_example() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        artifact_root: Some("stories/chronos-fracture".to_string()),
        missing_evidence: vec!["document-structure".to_string()],
        ..TurnAuthorityInput::default()
    });

    assert!(authority
        .valid_example
        .contains("<tool>artifact.apply</tool>"));
    assert!(authority.valid_example.contains("stories/chronos-fracture"));
    assert!(parse_completion(&authority.valid_example).is_ok());
}

#[test]
fn owner_missing_audit_evidence_prefers_artifact_audit_example() {
    let authority = decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        artifact_root: Some("stories/chronos-fracture".to_string()),
        missing_evidence: vec!["artifact-readiness".to_string()],
        ..TurnAuthorityInput::default()
    });

    assert!(authority
        .valid_example
        .contains("<tool>artifact.audit</tool>"));
    assert!(authority.valid_example.contains("stories/chronos-fracture"));
    assert!(parse_completion(&authority.valid_example).is_ok());
}

#[test]
fn rendered_model_examples_match_dispatch_registry() {
    for (input, tool) in [
        (
            TurnAuthorityInput {
                active_owner_case: true,
                ..TurnAuthorityInput::default()
            },
            "graph.state",
        ),
        (
            TurnAuthorityInput {
                recoverable_owner_case: true,
                ..TurnAuthorityInput::default()
            },
            "graph.recover",
        ),
        (
            TurnAuthorityInput {
                maintenance_due: true,
                ..TurnAuthorityInput::default()
            },
            "memory.find",
        ),
    ] {
        let authority = decide_turn_authority(input);
        assert_eq!(
            registry_valid_example(tool).as_deref(),
            Some(authority.valid_example.as_str())
        );
    }
}

fn call_model_inputs() -> [TurnAuthorityInput; 3] {
    [
        TurnAuthorityInput {
            active_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            recoverable_owner_case: true,
            ..TurnAuthorityInput::default()
        },
        TurnAuthorityInput {
            maintenance_due: true,
            ..TurnAuthorityInput::default()
        },
    ]
}
