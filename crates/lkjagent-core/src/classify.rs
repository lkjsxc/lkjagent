use crate::model::{Step, StepKind, StepState, Task, TaskSnapshot, TaskState, TemplateId};

pub fn classify(objective: &str) -> TemplateId {
    let lower = objective.to_ascii_lowercase();
    if lower.contains("manuscript") || lower.contains("chapter") {
        TemplateId::Manuscript
    } else if lower.contains("docs") || lower.contains("documentation") {
        TemplateId::DocsTree
    } else if lower.contains("journal") || lower.contains("schedule") || lower.contains("todo") {
        TemplateId::Journal
    } else if lower.contains("file") || lower.contains("write") || lower.contains("edit") {
        TemplateId::FileWork
    } else if lower.ends_with('?') || lower.starts_with("what ") || lower.starts_with("why ") {
        TemplateId::Question
    } else {
        TemplateId::Generic
    }
}

pub fn instantiate(id: u64, objective: &str) -> TaskSnapshot {
    let template = classify(objective);
    let task = Task {
        id,
        objective: objective.to_string(),
        template,
        state: TaskState::Open,
        brief: objective.to_string(),
        budget_used: 0,
        budget: 200,
        summary: String::new(),
        checks: Vec::new(),
    };
    let steps = match template {
        TemplateId::Question => vec![step(id, 1, 1, StepKind::Respond, "answer", objective)],
        _ => vec![
            explore_step(id, 1, objective, 20),
            step(id, 2, 2, StepKind::Respond, "report", "report the result"),
        ],
    };
    TaskSnapshot {
        task,
        steps,
        attempts: Vec::new(),
        check_results: Vec::new(),
        events: Vec::new(),
    }
}

fn explore_step(task_id: u64, id: u64, instruction: &str, budget: u32) -> Step {
    let mut step = step(task_id, id, 1, StepKind::Explore, "explore", instruction);
    step.action_budget = budget;
    step
}

fn step(
    task_id: u64,
    id: u64,
    ordinal: u32,
    kind: StepKind,
    title: &str,
    instruction: &str,
) -> Step {
    Step {
        id,
        task_id,
        ordinal,
        kind,
        title: title.to_string(),
        instruction: instruction.to_string(),
        inputs: String::new(),
        output_path: None,
        checks: Vec::new(),
        state: StepState::Pending,
        attempts_used: 0,
        actions_used: 0,
        action_budget: 0,
        split_used: false,
    }
}
