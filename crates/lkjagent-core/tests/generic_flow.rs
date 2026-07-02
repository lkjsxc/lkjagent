use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::{apply_turn, next_work, Command, TurnOutcome, Work};
use lkjagent_core::model::TaskState;
use lkjagent_core::parse::{Action, ParseFault, ParsedOutput};

#[test]
fn generic_task_closes_after_eight_plus_turns_with_faults() {
    let mut snapshot = instantiate(1, "Survey the workspace and report.");
    let mut all_commands = Vec::new();

    let outcomes = vec![
        TurnOutcome::ParseFault(ParseFault::WrongBlock),
        TurnOutcome::ParseFault(ParseFault::Empty),
        action("fs.read", "path", "README.md"),
        action("fs.search", "query", "Aurora"),
        action("memory.find", "query", "workspace"),
        action("plan.note", "note", "ready to summarize"),
        TurnOutcome::Model(ParsedOutput::Finish("found enough evidence".to_string())),
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

fn action(tool: &str, name: &str, value: &str) -> TurnOutcome {
    TurnOutcome::Model(ParsedOutput::Action(Action {
        tool: tool.to_string(),
        params: vec![(name.to_string(), value.to_string())],
    }))
}

#[test]
fn waiting_and_closed_tasks_have_no_model_work() {
    let mut snapshot = instantiate(2, "What is in the workspace?");
    snapshot.task.state = TaskState::Waiting;
    assert!(matches!(next_work(&snapshot), Work::Wait));
    snapshot.task.state = TaskState::Closed;
    assert!(matches!(next_work(&snapshot), Work::Wait));
}
