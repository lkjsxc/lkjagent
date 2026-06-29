mod obligation_network_support;

use lkjagent_runtime::kernel::{
    build_snapshot, progress_key_for_snapshot, reduce, RuntimeEvent, RuntimeFault, RuntimeMission,
};
use obligation_network_support::*;

#[test]
fn missing_root_audit_forces_root_identity_batch_write() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input.latest_observation = Some(missing_root_observation("stories/novel"));
    let decision = decision(input, RuntimeEvent::ArtifactRootMissing)?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert_eq!(
        identity_paths(&decision)?,
        story_identity_paths("stories/novel")
    );
    assert!(blocked(&decision, "doc.audit"));
    assert!(!admitted(&decision, "doc.audit"));
    assert_eq!(admitted_tools(&decision), vec!["fs.batch_write"]);
    Ok(())
}

#[test]
fn missing_root_survives_provider_anomaly_observation() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input.latest_successful_observation = Some(missing_root_observation("stories/novel"));
    input.latest_observation = Some("provider anomaly: reasoning_only_response".to_string());
    let decision = decision(
        input,
        RuntimeEvent::ProviderAnomaly {
            class: "reasoning_only_response".to_string(),
        },
    )?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert!(blocked(&decision, "doc.audit"));
    Ok(())
}

#[test]
fn artifact_next_ready_for_audit_is_not_converted_to_batch_write() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input.latest_observation = Some(
        "artifact_next_result=ready_for_audit\nroot=stories/novel\nkind=story\nmissing=0\nnext_decision_required=true\ncandidate_action=artifact.audit"
            .to_string(),
    );
    let decision = decision(input, RuntimeEvent::ArtifactWeakPathFound)?;

    assert_eq!(next_tool(&decision)?, "artifact.audit");
    Ok(())
}

#[test]
fn missing_root_recovery_never_forces_same_root_doc_audit() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input.latest_observation = Some(missing_root_observation("stories/novel"));
    input.latest_fault = Some(RuntimeFault::Repeat);
    input.retry_count = 1;
    let decision = decision(input, repeat_event())?;

    assert_eq!(decision.mission, RuntimeMission::OwnerRecovery);
    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert!(blocked(&decision, "doc.audit"));
    Ok(())
}

#[test]
fn structure_failed_audit_forces_repair_batch_write() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input.latest_observation = Some(
        "document audit failed\nroot=stories/novel\ntopology=failed\ncontent_readiness=not-requested\nfailures:\n- h1_count: README.md"
            .to_string(),
    );
    let decision = decision(input, RuntimeEvent::ArtifactAuditFailed)?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert_eq!(identity_paths(&decision)?, vec!["stories/novel/README.md"]);
    Ok(())
}

#[test]
fn artifact_next_root_missing_produces_same_identity_contract() -> Result<(), String> {
    let mut input = owner_input();
    input.latest_observation = Some(artifact_next_missing_root("stories/novel"));
    let decision = decision(input, RuntimeEvent::ArtifactRootMissing)?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert_eq!(
        identity_paths(&decision)?,
        story_identity_paths("stories/novel")
    );
    Ok(())
}

#[test]
fn repeated_action_guard_uses_progress_key() -> Result<(), String> {
    let mut input = owner_input();
    input.latest_observation = Some(missing_root_observation("stories/novel"));
    input.latest_fault = Some(RuntimeFault::Repeat);
    input.prior_action_fingerprint = Some("raw-doc-audit-action".to_string());
    input.retry_count = 2;
    let snapshot = build_snapshot(input).map_err(format_error)?;
    let key = progress_key_for_snapshot(&snapshot).fingerprint();
    let decision = reduce(&snapshot, repeat_event()).map_err(format_error)?;

    assert!(key.contains("target=stories/novel"));
    assert!(key.contains("action_class=identity-write"));
    assert!(decision
        .admission_view
        .refused_action_fingerprints
        .contains(&key));
    assert!(decision
        .admission_view
        .refused_action_fingerprints
        .contains(&"raw-doc-audit-action".to_string()));
    Ok(())
}

#[test]
fn current_model_run_missing_root_replays_to_batch_write() -> Result<(), String> {
    let fixture = include_str!("../../../data/logs/current-model-run.md");
    let mut checked = 0;
    for row in fixture.lines().filter(missing_root_row).take(8) {
        let mut input = owner_input();
        input.missing_evidence = vec!["document-structure".to_string()];
        input.latest_observation = Some(row.replace("<br>", "\n"));
        let decision = decision(input, RuntimeEvent::ArtifactRootMissing)?;
        assert_eq!(next_tool(&decision)?, "fs.batch_write");
        checked += 1;
    }
    assert!(checked > 0);
    Ok(())
}

#[test]
fn audit_owned_direct_graph_evidence_remains_blocked() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec![
        "document-structure".to_string(),
        "artifact-readiness".to_string(),
    ];
    let decision = decision(input, RuntimeEvent::OwnerMessageReceived)?;

    assert!(blocked(&decision, "graph.evidence"));
    assert!(!admitted(&decision, "graph.evidence"));
    Ok(())
}

#[test]
fn completion_waits_for_audit_and_verification_facts() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec![
        "document-structure".to_string(),
        "artifact-readiness".to_string(),
        "verification".to_string(),
    ];
    let decision = decision(input, RuntimeEvent::CompletionRequested)?;

    assert_eq!(decision.mission, RuntimeMission::OwnerCompletion);
    assert!(!decision.completion_allowed);
    assert!(decision.completion_refusal.is_some());
    assert_ne!(next_tool(&decision)?, "agent.done");
    Ok(())
}

fn admitted_tools(decision: &lkjagent_runtime::kernel::RuntimeDecision) -> Vec<&str> {
    decision
        .admission_view
        .admitted_tools
        .iter()
        .map(|tool| tool.as_str())
        .collect()
}

fn missing_root_row(line: &&str) -> bool {
    line.contains("observation") && line.contains("missing_root")
}
