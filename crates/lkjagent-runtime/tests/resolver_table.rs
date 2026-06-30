#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::{RuntimeEvent, RuntimeMission};
use obligation_network_support::*;

#[test]
fn resolver_table_names_primary_rules() -> Result<(), String> {
    assert_rule(
        plan_case(),
        RuntimeEvent::OwnerMessageReceived,
        "graph.plan",
        "record-graph-plan",
    )?;
    assert_rule(
        identity_case(),
        RuntimeEvent::OwnerMessageReceived,
        "artifact.plan",
        "inspect-artifact-plan",
    )?;
    assert_rule(
        root_case(),
        RuntimeEvent::ArtifactRootMissing,
        "fs.batch_write",
        "semantic-write-contract",
    )?;
    assert_rule(
        content_case(),
        RuntimeEvent::ArtifactWeakPathFound,
        "fs.batch_write",
        "semantic-write-contract",
    )?;
    assert_rule(
        structure_case(),
        RuntimeEvent::OwnerMessageReceived,
        "doc.audit",
        "audit-doc-audit",
    )?;
    assert_rule(
        readiness_case(),
        RuntimeEvent::OwnerMessageReceived,
        "artifact.audit",
        "audit-artifact-audit",
    )?;
    Ok(())
}

#[test]
fn recovery_blocked_handoff_has_exact_rule_and_missing_path() -> Result<(), String> {
    let mut input = owner_input();
    input.retry_count = 2;
    input.latest_observation = Some(
        "manuscript_target_words=10000\nnext_manuscript_path=stories/novel/manuscript/chapter-01.md\nanomaly_shrink_level=2"
            .to_string(),
    );
    let decision = decision(
        input,
        RuntimeEvent::ProviderAnomaly {
            class: "reasoning_only_response".to_string(),
        },
    )?;

    assert_eq!(decision.mission, RuntimeMission::OwnerRecovery);
    assert_eq!(
        decision.blocked_handoff_plan.as_deref(),
        Some(
            "manuscript provider anomaly blocked next_path=stories/novel/manuscript/chapter-01.md"
        )
    );
    assert!(decision
        .resolver_plan
        .as_deref()
        .unwrap_or("")
        .contains("rule=blocked-handoff"));
    assert!(decision
        .progress_key
        .as_deref()
        .unwrap_or("")
        .contains("plan=rule=blocked-handoff"));
    Ok(())
}

fn assert_rule(
    input: lkjagent_runtime::kernel::SnapshotAdapterInput,
    event: RuntimeEvent,
    tool: &str,
    rule: &str,
) -> Result<(), String> {
    let decision = decision(input, event)?;
    assert_eq!(next_tool(&decision)?, tool);
    let label = decision.resolver_plan.as_deref().unwrap_or("");
    assert!(label.contains(&format!("rule={rule}")), "label={label}");
    assert!(decision.progress_key.is_some());
    Ok(())
}

fn plan_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.missing_evidence = vec!["plan".to_string()];
    input
}

fn identity_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.artifact_root = None;
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input
}

fn root_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input.latest_observation = Some(missing_root_observation("stories/novel"));
    input
}

fn content_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input.latest_observation = Some(
        "artifact_next_result=write_contract\nroot=stories/novel\nnext_decision_required=true\ncandidate_action=fs.batch_write\npaths:\n- stories/novel/chapter.md"
            .to_string(),
    );
    input
}

fn structure_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.missing_evidence = vec!["document-structure".to_string()];
    input
}

fn readiness_case() -> lkjagent_runtime::kernel::SnapshotAdapterInput {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input
}
