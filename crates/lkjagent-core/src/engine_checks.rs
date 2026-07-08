use crate::checks::{evaluate, CommandFact, FileFact};
use crate::engine::Command;
use crate::engine_completion::record_event;
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
        .map(|spec| {
            let mut result = evaluate(spec, files, command_facts);
            result.params = Some(spec.clone());
            result
        })
        .collect::<Vec<_>>();
    let passed = results.iter().all(|result| result.passed);
    snapshot.check_results.extend(results.clone());
    commands.push(Command::RecordChecks {
        step_id,
        results: results.clone(),
    });
    if passed {
        snapshot.steps[index].state = StepState::Done;
        feed_next_step(snapshot, index, &results);
        record_event(
            commands,
            EventKind::StepDone,
            snapshot.steps[index].title.clone(),
        );
    } else {
        handle_failed(snapshot, commands, index, files, &results);
    }
}

fn feed_next_step(
    snapshot: &mut TaskSnapshot,
    index: usize,
    results: &[crate::model::CheckResult],
) {
    let summary = results
        .iter()
        .map(|result| {
            format!(
                "{}={} measured={}",
                result.name, result.passed, result.measured
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    if let Some(step) = snapshot
        .steps
        .iter_mut()
        .skip(index + 1)
        .find(|step| matches!(step.state, StepState::Pending | StepState::Active))
    {
        step.inputs.push_str("\ncheck_results:\n");
        step.inputs.push_str(&summary);
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
        snapshot.steps[index].state = StepState::Blocked;
        record_event(
            commands,
            EventKind::StepBlocked,
            "deterministic checks failed".to_string(),
        );
        return;
    }
    let keep = snapshot.check_results.len().saturating_sub(results.len());
    snapshot.check_results.truncate(keep);
    snapshot.steps[index].state = StepState::Skipped;
    let additions = insert_after(snapshot, index, &additions);
    add_steps(
        commands,
        additions,
        "extend artifact after word-count shortfall",
    );
}
