#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::{RuntimeEvent, RuntimeMission};
use obligation_network_support::*;

#[test]
fn direct_chapter_request_forces_exact_manuscript_write() -> Result<(), String> {
    let mut input = owner_input();
    input.owner_objective = Some(direct_objective());
    input.artifact_root = Some("stories/the-bell-rings-twice".to_string());
    input.missing_evidence = vec!["artifact-readiness".to_string()];

    let decision = decision(input, RuntimeEvent::OwnerMessageReceived)?;
    let contract = decision
        .content_write_contract
        .as_ref()
        .ok_or("missing content contract")?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert_eq!(
        contract.paths,
        vec!["stories/the-bell-rings-twice/manuscript/chapter-01.md"]
    );
    assert!(contract
        .required_sections
        .contains(&"finished chapter prose".to_string()));
    assert!(contract.max_file_bytes >= 12_000);
    Ok(())
}

#[test]
fn completion_refuses_story_bible_only_manuscript() -> Result<(), String> {
    let mut input = owner_input();
    input.owner_objective = Some(long_objective());
    input.artifact_root = Some("stories/bell-rings-twice".to_string());
    input.existing_evidence = vec!["plan".to_string(), "observation".to_string()];
    input.latest_successful_observation = Some(manuscript_failure());

    let decision = decision(input, RuntimeEvent::CompletionRequested)?;

    assert_eq!(decision.mission, RuntimeMission::OwnerCompletion);
    assert!(!decision.completion_allowed);
    assert!(decision.completion_refusal.as_deref().is_some_and(|text| {
        text.contains("manuscript-word-count") && text.contains("chapter-01.md")
    }));
    assert!(decision
        .completion_gate_inputs
        .iter()
        .any(|line| line == "manuscript_words_written=0"));
    Ok(())
}

#[test]
fn provider_anomaly_preserves_and_shrinks_manuscript_path() -> Result<(), String> {
    let mut input = owner_input();
    input.owner_objective = Some(direct_objective());
    input.artifact_root = Some("stories/the-bell-rings-twice".to_string());
    input.provider_anomaly_class = Some("reasoning_only_response".to_string());
    input.latest_successful_observation = Some(write_candidate());

    let decision = decision(
        input,
        RuntimeEvent::ProviderAnomaly {
            class: "reasoning_only_response".to_string(),
        },
    )?;
    let contract = decision.content_write_contract.as_ref().ok_or("contract")?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    assert_eq!(
        contract.paths[0],
        "stories/the-bell-rings-twice/manuscript/chapter-01.md"
    );
    assert_eq!(contract.max_file_bytes, 6_000);
    Ok(())
}

#[test]
fn repeated_provider_anomaly_blocks_with_exact_path() -> Result<(), String> {
    let mut input = owner_input();
    input.owner_objective = Some(direct_objective());
    input.artifact_root = Some("stories/the-bell-rings-twice".to_string());
    input.provider_anomaly_class = Some("reasoning_only_response".to_string());
    input.provider_retry_count = 2;

    let decision = decision(
        input,
        RuntimeEvent::ProviderAnomaly {
            class: "reasoning_only_response".to_string(),
        },
    )?;

    assert!(decision
        .blocked_handoff_plan
        .as_deref()
        .is_some_and(|text| { text.contains("manuscript") && text.contains("chapter-01.md") }));
    Ok(())
}

fn direct_objective() -> String {
    "Write one 700 to 900 word chapter at stories/the-bell-rings-twice/manuscript/chapter-01.md. Do not create structured-output.".to_string()
}

fn long_objective() -> String {
    "Create a 10,000 word high-school romance novel named The Bell Rings Twice in ten chapters."
        .to_string()
}

fn manuscript_failure() -> String {
    "artifact audit failed\nroot=stories/bell-rings-twice\nreadiness=missing-manuscript-content\nfailures:\n- manuscript_missing_paths: stories/bell-rings-twice/manuscript/chapter-01.md\n- manuscript_word_count: 0\n- manuscript_target_words: 10000\n- next_manuscript_path: stories/bell-rings-twice/manuscript/chapter-01.md\nnext_decision_required=true\ncandidate_action=artifact.next".to_string()
}

fn write_candidate() -> String {
    "next_decision_required=true\ncandidate_action=fs.batch_write\nnext_paths:\n- stories/the-bell-rings-twice/manuscript/chapter-01.md".to_string()
}
