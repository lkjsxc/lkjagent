use lkjagent_core::classify::instantiate;
use lkjagent_core::model::StepKind;
use lkjagent_core::render::{max_tokens, render_prompt, render_prompt_for_decision};
use lkjagent_core::runtime_artifact::DEFAULT_UNIT_TARGET_TOKENS;
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};

#[test]
fn write_steps_use_artifact_unit_budget() {
    assert_eq!(max_tokens(StepKind::Write), DEFAULT_UNIT_TARGET_TOKENS);
    assert_eq!(max_tokens(StepKind::Revise), DEFAULT_UNIT_TARGET_TOKENS);
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
    assert!(!prompt.system.contains("Return exactly <action>"));
}

#[test]
fn prompt_includes_task_brief() {
    let mut snapshot = instantiate(2, "What is known?");
    snapshot.task.brief = "memory_facts:\nrow memory fact".to_string();
    let prompt = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
    assert!(prompt.system.contains("Task brief:"));
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
