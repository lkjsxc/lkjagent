use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{apply_turn, next_work, Command, TurnOutcome, Work};
use lkjagent_core::model::{CheckResult, CheckSpec, EventKind, StepState, TaskSnapshot, TaskState};

#[test]
fn blocked_bridge_step_never_closes() {
    let snapshot = unsafe_file_work(StepState::Blocked);
    let (next, commands) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Blocked);
    assert!(task_blocked_event(&commands));
}

#[test]
fn active_bridge_step_never_closes() {
    let snapshot = unsafe_file_work(StepState::Active);
    let (next, _) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Blocked);
}

#[test]
fn skipped_bridge_step_without_evidence_never_closes() {
    let snapshot = unsafe_file_work(StepState::Skipped);
    let (next, _) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Blocked);
}

#[test]
fn file_work_without_artifact_evidence_never_closes() {
    let mut snapshot = instantiate(4, "Create something to read");
    for step in &mut snapshot.steps {
        step.state = StepState::Done;
    }
    let (next, _) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Blocked);
}

#[test]
fn all_done_question_can_close_without_artifact_checks() {
    let mut snapshot = instantiate(5, "are you ok?");
    for step in &mut snapshot.steps {
        step.state = StepState::Done;
    }
    let (next, commands) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Closed);
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::RecordEvent(event) if event.kind == EventKind::TaskClosed
    )));
}

#[test]
fn blocked_bridge_projection_selects_block_instead_of_close() {
    let snapshot = unsafe_file_work(StepState::Blocked);

    assert!(matches!(next_work(&snapshot), Work::BlockTask(_)));
}

#[test]
fn mismatched_check_result_params_never_close_file_work() {
    let mut snapshot = checked_file_work();
    snapshot.check_results = vec![check_result(file_exists("other.md"))];

    let (next, _) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Blocked);
}

#[test]
fn matching_check_result_params_allow_file_work_close() {
    let mut snapshot = checked_file_work();
    snapshot.check_results = vec![check_result(file_exists("notes/out.md"))];

    let (next, _) = apply_turn(&snapshot, &Work::CloseTask, TurnOutcome::Noop);

    assert_eq!(next.task.state, TaskState::Closed);
}

fn checked_file_work() -> TaskSnapshot {
    let mut snapshot = instantiate(10, "Write notes/out.md with setup notes.");
    for step in &mut snapshot.steps {
        step.state = StepState::Done;
    }
    snapshot.task.checks = vec![file_exists("notes/out.md")];
    snapshot
}

fn file_exists(path: &str) -> CheckSpec {
    CheckSpec::FileExists {
        path: path.to_string(),
    }
}

fn check_result(spec: CheckSpec) -> CheckResult {
    CheckResult {
        name: "file_exists".to_string(),
        params: Some(spec),
        passed: true,
        measured: "true".to_string(),
    }
}

fn unsafe_file_work(first_state: StepState) -> TaskSnapshot {
    let mut snapshot = instantiate(9, "Create something to read with structured details");
    snapshot.steps[0].state = first_state;
    for step in snapshot.steps.iter_mut().skip(1) {
        step.state = StepState::Done;
    }
    snapshot
}

fn task_blocked_event(commands: &[Command]) -> bool {
    commands.iter().any(|command| {
        matches!(
            command,
            Command::RecordEvent(event) if event.kind == EventKind::TaskBlocked
        )
    })
}
