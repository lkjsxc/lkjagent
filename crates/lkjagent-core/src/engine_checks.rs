use crate::checks::{evaluate, CommandFact, FileFact};
use crate::engine::Command;
use crate::engine_extend::{add_steps, insert_after, shortfall_steps};
use crate::model::{EventKind, StepState, TaskSnapshot};

pub(crate) fn handle_checks(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    step_id: u64,
    files: &[FileFact],
    command_facts: &[CommandFact],
) {
    let Some(index) = snapshot.steps.iter().position(|step| step.id == step_id) else {
        return;
    };
    let results = snapshot.steps[index]
        .checks
        .iter()
        .map(|spec| evaluate(spec, files, command_facts))
        .collect::<Vec<_>>();
    let passed = results.iter().all(|result| result.passed);
    snapshot.check_results.extend(results.clone());
    commands.push(Command::RecordChecks(results.clone()));
    if passed {
        snapshot.steps[index].state = StepState::Done;
        crate::engine_steps::record_event(
            commands,
            EventKind::StepDone,
            snapshot.steps[index].title.clone(),
        );
    } else {
        handle_failed(snapshot, commands, index, files, &results);
    }
}

fn handle_failed(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    files: &[FileFact],
    results: &[crate::model::CheckResult],
) {
    snapshot.steps[index].attempts_used += 1;
    let additions = shortfall_steps(&snapshot.steps[index], files, results);
    if additions.is_empty() {
        return;
    }
    let keep = snapshot.check_results.len().saturating_sub(results.len());
    snapshot.check_results.truncate(keep);
    snapshot.steps[index].state = StepState::Skipped;
    insert_after(snapshot, index, &additions);
    add_steps(commands, additions, "extend manuscript after shortfall");
}
