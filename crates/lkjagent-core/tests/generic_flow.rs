use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{
    apply_turn, next_work, next_work_with_decision, Command, TurnOutcome, Work,
};
use lkjagent_core::model::{CheckSpec, StepState, TaskState};
use lkjagent_core::parse::{Action, ParseFault, ParsedOutput};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView};

#[test]
fn generic_task_closes_after_eight_plus_turns_with_faults() {
    let mut snapshot = instantiate(1, "Survey the workspace and report.");
    let mut all_commands = Vec::new();

    let outcomes = vec![
        TurnOutcome::ParseFault(ParseFault::WrongBlock),
        TurnOutcome::ParseFault(ParseFault::Empty),
        action("fs.read", "path", "README.md"),
        action("fs.search", "query", "release"),
        action("memory.find", "query", "workspace"),
        action("plan.note", "note", "ready to summarize"),
        action("finish", "summary", "found enough evidence"),
        TurnOutcome::Model(ParsedOutput::Message("Survey complete.".to_string())),
        TurnOutcome::Noop,
    ];

    for outcome in outcomes {
        let work = next_work(&snapshot);
        let (next, commands) = apply_turn(&snapshot, &work, outcome);
        all_commands.extend(commands);
        snapshot = next;
    }

    assert_eq!(snapshot.task.state, TaskState::Closed);
    assert!(snapshot.attempts.len() >= 8);
    assert!(
        all_commands
            .iter()
            .filter(|cmd| matches!(cmd, Command::RunExplore(_)))
            .count()
            == 4
    );
    assert!(all_commands
        .iter()
        .any(|cmd| matches!(cmd, Command::RecordEvent(_))));
}

#[test]
fn repeated_explore_action_is_not_executed() {
    let snapshot = instantiate(3, "Survey the workspace and report.");
    let work = next_work(&snapshot);
    let (snapshot, _) = apply_turn(&snapshot, &work, action("fs.read", "path", "README.md"));
    let work = next_work(&snapshot);
    let (snapshot, commands) = apply_turn(&snapshot, &work, action("fs.read", "path", "README.md"));
    assert_eq!(snapshot.steps[0].actions_used, 1);
    assert_eq!(snapshot.steps[0].attempts_used, 1);
    assert!(!commands
        .iter()
        .any(|cmd| matches!(cmd, Command::RunExplore(_))));
    assert!(commands.iter().any(|cmd| matches!(
        cmd,
        Command::RecordAttempt(attempt)
            if attempt.diagnosis.contains("repeated action")
    )));
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

fn action(tool: &str, name: &str, value: &str) -> TurnOutcome {
    TurnOutcome::Model(ParsedOutput::Action(Action {
        tool: tool.to_string(),
        params: vec![(name.to_string(), value.to_string())],
    }))
}

#[test]
fn decision_operation_selects_work_even_when_step_order_differs() {
    let snapshot = instantiate(7, "Write notes/out.md with setup notes.");
    let close = decision("completion.close", OutputEnvelope::None);
    assert!(matches!(
        next_work_with_decision(&snapshot, &close),
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
fn waiting_and_closed_tasks_have_no_model_work() {
    let mut snapshot = instantiate(2, "What is in the workspace?");
    snapshot.task.state = TaskState::Waiting;
    assert!(matches!(next_work(&snapshot), Work::Wait));
    snapshot.task.state = TaskState::Closed;
    assert!(matches!(next_work(&snapshot), Work::Wait));
}
