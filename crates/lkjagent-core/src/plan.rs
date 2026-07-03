use crate::model::{CheckSpec, Step, StepKind, StepState};
use crate::parse::PlanLine;

pub(crate) fn plan_steps(parent: &Step, lines: Vec<PlanLine>) -> Vec<Step> {
    lines
        .into_iter()
        .enumerate()
        .map(|(offset, line)| plan_step(parent, offset as u32 + 1, line))
        .collect()
}

fn plan_step(parent: &Step, offset: u32, line: PlanLine) -> Step {
    let id = parent.id.saturating_mul(100).saturating_add(offset as u64);
    match line {
        PlanLine::Write { path, title, words } => Step {
            id,
            task_id: parent.task_id,
            ordinal: parent.ordinal + offset,
            kind: StepKind::Write,
            title,
            instruction: format!("write at least {words} words"),
            inputs: String::new(),
            output_path: Some(path.clone()),
            checks: vec![CheckSpec::MinWords { path, n: words }],
            state: StepState::Pending,
            attempts_used: 0,
            actions_used: 0,
            action_budget: 0,
            split_used: false,
        },
        PlanLine::Explore { goal, budget } => Step {
            id,
            task_id: parent.task_id,
            ordinal: parent.ordinal + offset,
            kind: StepKind::Explore,
            title: "explore".to_string(),
            instruction: goal,
            inputs: String::new(),
            output_path: None,
            checks: Vec::new(),
            state: StepState::Pending,
            attempts_used: 0,
            actions_used: 0,
            action_budget: budget,
            split_used: false,
        },
        PlanLine::Respond { summary } => Step {
            id,
            task_id: parent.task_id,
            ordinal: parent.ordinal + offset,
            kind: StepKind::Respond,
            title: "respond".to_string(),
            instruction: summary,
            inputs: String::new(),
            output_path: None,
            checks: Vec::new(),
            state: StepState::Pending,
            attempts_used: 0,
            actions_used: 0,
            action_budget: 0,
            split_used: false,
        },
    }
}
