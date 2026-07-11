use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{
    apply_turn, next_work, next_work_with_decision, Command, TurnOutcome, Work,
};
use lkjagent_core::model::{CheckSpec, StepState, TaskState};
use lkjagent_core::parse::ParsedOutput;
use lkjagent_core::runtime_decision::{
    EffectCommand, OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView,
};

#[test]
fn generic_explore_requires_a_persisted_decision() {
    let snapshot = instantiate(1, "Survey the workspace and report.");
    assert!(matches!(
        next_work(&snapshot),
        Work::BlockTask(reason) if reason == "Explore requires a persisted runtime decision"
    ));
}

#[test]
fn generic_explore_blocked_turn_emits_no_effect_command() {
    let snapshot = instantiate(2, "Survey the workspace and report.");
    let work = next_work(&snapshot);
    let (_, commands) = apply_turn(&snapshot, &work, TurnOutcome::Noop);
    assert!(!commands
        .iter()
        .any(|command| matches!(command, Command::RunExplore(_))));
}

#[test]
fn planned_write_steps_keep_word_checks() {
    let snapshot = instantiate(5, "Write notes/out.md with setup notes.");
    let work = next_work(&snapshot);
    let plan = ParsedOutput::Plan(vec![lkjagent_core::parse::PlanLine::Write {
        path: "notes/out.md".to_string(),
        title: "draft".to_string(),
        words: 25,
    }]);
    let (snapshot, _) = apply_turn(&snapshot, &work, TurnOutcome::Model(plan));
    assert!(snapshot.steps.iter().any(|step| matches!(
        step.checks.as_slice(),
        [CheckSpec::MinWords { path, n }] if path == "notes/out.md" && *n == 25
    )));
}

#[test]
fn endpoint_errors_use_ten_failure_patience() {
    let mut snapshot = instantiate(4, "What is an agent?");
    for _ in 0..9 {
        let work = next_work(&snapshot);
        let (next, _) = apply_turn(&snapshot, &work, TurnOutcome::EndpointError("down".into()));
        snapshot = next;
        assert_ne!(snapshot.steps[0].state, StepState::Blocked);
    }
    let work = next_work(&snapshot);
    let (snapshot, _) = apply_turn(&snapshot, &work, TurnOutcome::EndpointError("down".into()));
    assert_eq!(snapshot.steps[0].state, StepState::Blocked);
}

fn decision(operation: &str, envelope: OutputEnvelope) -> RuntimeDecision {
    RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey(operation.to_string()),
        ToolSetView::empty(),
        envelope,
    )
}

#[test]
fn decision_operation_selects_work_even_when_step_order_differs() {
    let snapshot = instantiate(7, "Write notes/out.md with setup notes.");
    let close = decision("completion.close", OutputEnvelope::None);
    assert!(matches!(
        next_work_with_decision(&snapshot, &close),
        Work::BlockTask(_)
    ));
    let mut done = instantiate(70, "are you ok?");
    for step in &mut done.steps {
        step.state = StepState::Done;
    }
    assert!(matches!(
        next_work_with_decision(&done, &close),
        Work::CloseTask
    ));
    let model = decision("model.call/1", OutputEnvelope::Plan);
    assert!(matches!(
        next_work_with_decision(&snapshot, &model),
        Work::CallModel { step_id: 1, .. }
    ));
}

#[test]
fn unsupported_decision_blocks_instead_of_recomputing_work() {
    let snapshot = instantiate(8, "What is in the workspace?");
    let decision = decision("unknown.operation", OutputEnvelope::None);
    assert!(matches!(
        next_work_with_decision(&snapshot, &decision),
        Work::BlockTask(_)
    ));
}

#[test]
fn state_resolve_decision_is_native_model_free_work() {
    let snapshot = instantiate(9, "What is in the workspace?");
    let decision = decision("state.resolve", OutputEnvelope::None);
    assert_eq!(
        next_work_with_decision(&snapshot, &decision),
        Work::ResolveState
    );
}

#[test]
fn native_workspace_write_effect_emits_write_command() {
    let snapshot = instantiate(10, "What is in the workspace?");
    assert!(native_effect_commands(&snapshot, "workspace.write_text")
        .iter()
        .any(|command| matches!(
            command,
            Command::WriteFile { path, content } if path == "native.md" && content == "body"
        )));
    assert!(native_effect_commands(&snapshot, "workspace.append_text")
        .iter()
        .any(|command| matches!(
            command,
            Command::AppendFile { path, content } if path == "native.md" && content == "body"
        )));
}

fn native_effect_commands(
    snapshot: &lkjagent_core::model::TaskSnapshot,
    name: &str,
) -> Vec<Command> {
    let mut decision = decision("effect.workspace.write", OutputEnvelope::None);
    decision.effect_command = Some(EffectCommand {
        name: name.to_string(),
        path: Some("native.md".to_string()),
        content: Some("body".to_string()),
    });
    let work = next_work_with_decision(snapshot, &decision);
    apply_turn(snapshot, &work, TurnOutcome::Noop).1
}

#[test]
fn waiting_and_closed_tasks_have_no_model_work() {
    let mut snapshot = instantiate(2, "What is in the workspace?");
    snapshot.task.state = TaskState::Waiting;
    assert!(matches!(next_work(&snapshot), Work::Wait));
    snapshot.task.state = TaskState::Closed;
    assert!(matches!(next_work(&snapshot), Work::Wait));
}
