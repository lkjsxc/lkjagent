#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::{RuntimeEvent, RuntimeMission};
use obligation_network_support::*;

#[test]
fn content_atom_audit_failure_routes_to_artifact_next() -> Result<(), String> {
    let mut input = report_input();
    input.latest_observation = Some(atom_failure());
    input.missing_evidence = vec!["artifact-readiness".to_string()];

    let decision = decision(input, RuntimeEvent::ArtifactAuditFailed)?;

    assert_eq!(next_tool(&decision)?, "artifact.next");
    assert!(admitted(&decision, "artifact.next"));
    Ok(())
}

#[test]
fn completion_refuses_missing_content_atoms() -> Result<(), String> {
    let mut input = report_input();
    input.latest_successful_observation = Some(atom_failure());
    input.existing_evidence = vec!["plan".to_string(), "observation".to_string()];

    let decision = decision(input, RuntimeEvent::CompletionRequested)?;

    assert_eq!(decision.mission, RuntimeMission::OwnerCompletion);
    assert!(!decision.completion_allowed);
    assert!(decision
        .completion_refusal
        .as_deref()
        .is_some_and(|text| text.contains("content-atoms:2:analysis.md")));
    assert!(decision
        .completion_gate_inputs
        .iter()
        .any(|line| line == "content_atom_missing_count=2"));
    Ok(())
}

#[test]
fn generic_root_conflict_blocks_instead_of_repairing_wrong_root() -> Result<(), String> {
    let mut input = owner_input();
    input.artifact_root = Some("structured-output".to_string());
    input.artifact_kind = Some("artifact".to_string());
    input.owner_objective = Some(
        "Create the final report at reports/market-map/analysis.md, not a generic root."
            .to_string(),
    );

    let decision = decision(input, RuntimeEvent::OwnerMessageReceived)?;

    assert!(decision
        .blocked_handoff_plan
        .as_deref()
        .is_some_and(|text| {
            text.contains("structured-output") && text.contains("reports/market-map/analysis.md")
        }));
    Ok(())
}

fn report_input() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.artifact_root = Some("reports/market-map".to_string());
    input.artifact_kind = Some("report".to_string());
    input.owner_objective = Some("Create a market map report.".to_string());
    input
}

fn atom_failure() -> String {
    "artifact audit failed\nroot=reports/market-map\nreadiness=missing-content-atoms\nartifact_atom_profile=report\natom_status=missing\natom_missing_count=2\nnext_atom=analysis.md\nrequired_atoms=executive-summary.md,evidence.md,analysis.md\nfailed=1\nfailures:\n- content_atoms_missing: analysis.md,recommendations.md\nnext_decision_required=true\ncandidate_action=artifact.next".to_string()
}
