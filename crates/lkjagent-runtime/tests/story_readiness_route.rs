#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::{RuntimeEvent, RuntimeFault};
use obligation_network_support::*;

#[test]
fn story_readiness_failure_routes_to_artifact_next() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input.latest_observation = Some(story_readiness_failure());
    let decision = decision(input, RuntimeEvent::ArtifactAuditFailed)?;

    assert_eq!(next_tool(&decision)?, "artifact.next");
    assert_eq!(admitted_tools(&decision), vec!["artifact.next"]);
    Ok(())
}

#[test]
fn story_readiness_candidate_survives_repeat_fault() -> Result<(), String> {
    let mut input = owner_input();
    input.missing_evidence = vec!["artifact-readiness".to_string()];
    input.latest_successful_observation = Some(story_readiness_failure());
    input.latest_observation = Some("repeat action refused".to_string());
    input.latest_fault = Some(RuntimeFault::Repeat);
    input.retry_count = 1;
    let decision = decision(input, repeat_event())?;

    assert_eq!(next_tool(&decision)?, "artifact.next");
    assert_eq!(admitted_tools(&decision), vec!["artifact.next"]);
    Ok(())
}

fn story_readiness_failure() -> String {
    "artifact audit failed\nroot=stories/novel\nreadiness=missing-semantic-content\nfailures:\n- story_semantic_missing: premise,timeline\n- story_scale_missing: profile-scale-content-groups,profile-scale-word-count\nnext_decision_required=true\ncandidate_action=artifact.next".to_string()
}

fn admitted_tools(decision: &lkjagent_runtime::kernel::RuntimeDecision) -> Vec<&str> {
    decision
        .admission_view
        .admitted_tools
        .iter()
        .map(|tool| tool.as_str())
        .collect()
}
