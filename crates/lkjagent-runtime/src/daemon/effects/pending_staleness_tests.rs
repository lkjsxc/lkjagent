use super::*;
use crate::mode::{decide_turn_authority, TurnAuthorityInput};
use lkjagent_protocol::parse_completion;

#[test]
fn persisted_action_survives_compaction_prompt_rotation() -> Result<(), String> {
    let pending = pending(
        "frame-before",
        stale("false", "head-before", "prompt-before"),
    )?;
    let current = authority(
        "frame-after",
        stale("true", "head-after", "prompt-after"),
        None,
    );

    assert_eq!(
        persisted_action_refusal(&pending, &current, "fs.batch_write"),
        None
    );
    Ok(())
}

#[test]
fn persisted_action_refuses_core_authority_change() -> Result<(), String> {
    let pending = pending(
        "frame-before",
        stale("false", "head-before", "prompt-before"),
    )?;
    let current = authority(
        "frame-after",
        stale_with_node("verify", "false", "head-before", "prompt-before"),
        Some("verify"),
    );

    let refusal = persisted_action_refusal(&pending, &current, "fs.batch_write")
        .ok_or_else(|| "core graph change should stale pending action".to_string())?;
    assert!(refusal.contains("staleness_fingerprint"));
    Ok(())
}

fn pending(frame: &str, stale: String) -> Result<PendingAction, String> {
    let action = parse_completion("<action>\n<tool>graph.state</tool>\n</action>")
        .map_err(|error| format!("{error:?}"))?;
    Ok(PendingAction {
        action,
        action_text: String::new(),
        authority_decision_id: Some("decision-1".to_string()),
        prompt_frame_id: Some(frame.to_string()),
        staleness_fingerprint: Some(stale),
    })
}

fn authority(frame: &str, stale: String, node: Option<&str>) -> TurnAuthority {
    decide_turn_authority(TurnAuthorityInput {
        active_owner_case: true,
        graph_node: Some(node.unwrap_or("document").to_string()),
        graph_phase: Some("execution".to_string()),
        prompt_frame_id: Some(frame.to_string()),
        staleness_fingerprint: Some(stale),
        ..TurnAuthorityInput::default()
    })
}

fn stale(compaction: &str, head: &str, prompt: &str) -> String {
    stale_with_node("document", compaction, head, prompt)
}

fn stale_with_node(node: &str, compaction: &str, head: &str, prompt: &str) -> String {
    format!(
        "stale:queue=None:0;case=Some(1);graph=Some(\"{node}\"):Some(\"execution\");artifact=Some(\"stories/chronos-fracture\"):Some(\"style/tone.md\");fault=None:0:None:None;missing=document-structure|artifact-readiness;compaction={compaction}:{head};maintenance=false:false:;prompt={prompt}"
    )
}
