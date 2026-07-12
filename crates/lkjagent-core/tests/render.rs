use lkjagent_core::classify::instantiate;
use lkjagent_core::model::{Attempt, AttemptOutcome, StepKind};
use lkjagent_core::parse::{parse_expected_for_decision, ParseFault};
use lkjagent_core::render::{
    max_tokens, render_prompt, render_prompt_for_decision, render_prompt_for_decision_with_attempts,
};
use lkjagent_core::runtime_artifact::DEFAULT_UNIT_TARGET_TOKENS;
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};

#[test]
fn write_steps_use_artifact_unit_budget_with_close_headroom() {
    let expected = DEFAULT_UNIT_TARGET_TOKENS + 256;
    assert_eq!(max_tokens(StepKind::Write), expected);
    assert_eq!(max_tokens(StepKind::Revise), expected);
}

#[test]
fn decision_envelope_replaces_step_prompt_policy() {
    let snapshot = instantiate(3, "Survey workspace and report.");
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    let prompt = render_prompt_for_decision(
        &snapshot.task,
        &snapshot.steps,
        &snapshot.steps[0],
        &decision,
    );
    assert!(prompt.system.contains("Expected: message"));
    assert!(prompt.system.contains("Return exactly <message>"));
    assert!(!prompt.system.contains("Return exactly <lkjagent_action>"));
}

#[test]
fn explore_decision_renders_tool_call_contract() {
    let snapshot = instantiate(3, "Survey workspace and report.");
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    let prompt = render_prompt_for_decision(
        &snapshot.task,
        &snapshot.steps,
        &snapshot.steps[0],
        &decision,
    );
    assert!(prompt.system.contains("Expected: lkjagent_action"));
    assert!(prompt.system.contains("Harness state: act"));
    assert!(prompt
        .system
        .contains("Return exactly one <lkjagent_action>"));
    assert!(prompt.user.contains("<tool_name>fs.read</tool_name>"));
    assert!(prompt.user.contains("Schema-only shape, not copyable:"));
    assert!(prompt.user.contains("<input>"));
    assert!(prompt.user.contains("<path>FIELD_VALUE</path>"));
    assert_eq!(prompt.stop, "</lkjagent_action>");
}

#[test]
fn legacy_rendered_placeholder_shape_is_rejected() -> Result<(), String> {
    let snapshot = instantiate(3, "Survey workspace and report.");
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("fs.read", "read file").with_params(vec!["path"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    let prompt = render_prompt_for_decision(
        &snapshot.task,
        &snapshot.steps,
        &snapshot.steps[0],
        &decision,
    );
    let shape = prompt
        .user
        .split("Schema-only shape, not copyable:\n")
        .last()
        .unwrap_or("");
    assert_eq!(
        parse_expected_for_decision(&decision, shape),
        Err(ParseFault::Action(
            lkjagent_core::runtime_tool_call::ToolCallError::UnknownRoot
        ))
    );
    Ok(())
}

#[test]
fn generic_decision_envelopes_render_protocol_cards() {
    for (envelope, tag) in [
        (OutputEnvelope::Content, "content"),
        (OutputEnvelope::Message, "message"),
        (OutputEnvelope::Verdict, "verdict"),
    ] {
        let snapshot = instantiate(3, "Render contract.");
        let decision = RuntimeDecision::new(
            "decision-1",
            "case-1",
            OperationKey("model.call/1".to_string()),
            ToolSetView::empty(),
            envelope,
        );
        let prompt = render_prompt_for_decision(
            &snapshot.task,
            &snapshot.steps,
            &snapshot.steps[0],
            &decision,
        );
        assert!(prompt.user.contains(&format!("Copy this shape:\n<{tag}>")));
        assert_eq!(prompt.stop, format!("</{tag}>"));
    }
}

#[test]
fn fault_linked_recovery_frame_names_next_envelope() {
    let snapshot = instantiate(3, "Survey workspace and report.");
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    let attempts = vec![Attempt {
        step_id: snapshot.steps[0].id,
        ordinal: 2,
        prompt_fingerprint: "old".to_string(),
        outcome: AttemptOutcome::ParseFault,
        diagnosis: "WrongBlock".to_string(),
        tokens_in: 0,
        tokens_out: 0,
        cached_tokens: 0,
        cache_status: "unknown".to_string(),
    }];
    let prompt = render_prompt_for_decision_with_attempts(
        &snapshot.task,
        &snapshot.steps,
        &attempts,
        &snapshot.steps[0],
        &decision,
    );

    assert!(prompt.user.contains("Recovery frame:"));
    assert!(prompt.user.contains("decision=decision-1"));
    assert!(prompt.user.contains("fault=WrongBlock"));
    assert!(prompt.user.contains("invalid_excerpt_hash="));
    assert!(prompt
        .user
        .contains("Next expected envelope: <message>...</message>"));
}

#[test]
fn prompt_includes_matter_brief() {
    let mut snapshot = instantiate(2, "What is known?");
    snapshot.task.brief = "memory_facts:\nrow memory fact".to_string();
    let prompt = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
    assert!(prompt.system.contains("Matter brief:"));
    assert!(prompt.system.contains("row memory fact"));
}

#[test]
fn retry_prompt_fingerprint_changes() {
    let mut snapshot = instantiate(1, "answer a workspace question");
    let step = match snapshot.steps.first().cloned() {
        Some(step) => step,
        None => return assert_eq!(snapshot.steps.len(), 1),
    };
    let before = render_prompt(&snapshot.task, &snapshot.steps, &step);
    snapshot.steps[0].attempts_used = 1;
    let after = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
    assert_ne!(before.fingerprint, after.fingerprint);
}
