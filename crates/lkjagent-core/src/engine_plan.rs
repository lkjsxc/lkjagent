use crate::docs_tree::validate_plan;
use crate::engine::Command;
use crate::engine_extend::insert_after;
use crate::model::{Event, EventKind, StepState, TaskSnapshot};
use crate::parse::PlanLine;
use crate::plan::plan_steps;

pub(crate) fn finish_plan(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    lines: Vec<PlanLine>,
) {
    let additions = plan_steps(&snapshot.steps[index], lines);
    if snapshot.steps[index].title == "docs tree plan" {
        if let Err(diagnosis) = validate_plan(&snapshot.steps[index], &additions) {
            record_plan_fault(snapshot, commands, index, diagnosis);
            return;
        }
    }
    snapshot.steps[index].state = StepState::Done;
    insert_after(snapshot, index, &additions);
    commands.push(Command::AddSteps(additions));
}

fn record_plan_fault(
    snapshot: &mut TaskSnapshot,
    commands: &mut Vec<Command>,
    index: usize,
    diagnosis: String,
) {
    let step = &mut snapshot.steps[index];
    step.state = StepState::Active;
    step.attempts_used += 1;
    let kind = if step.attempts_used >= 3 {
        step.state = StepState::Blocked;
        EventKind::StepBlocked
    } else {
        EventKind::Notice
    };
    commands.push(Command::RecordEvent(Event {
        kind,
        content: diagnosis,
    }));
}
