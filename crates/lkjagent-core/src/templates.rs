use crate::classify::classify;
use crate::model::{
    CheckSpec, Step, StepKind, StepState, Task, TaskSnapshot, TaskState, TemplateId,
};

#[rustfmt::skip]
const WORKSPACE_WORDS: &[&str] = &["workspace", "repo", "repository", "file", "docs", "/", ".md"];

pub fn instantiate(id: u64, objective: &str) -> TaskSnapshot {
    let template = classify(objective);
    if template == TemplateId::DocsTree {
        return crate::docs_tree::instantiate(id, objective);
    }
    let mut task = task(id, objective, template);
    task.checks = task_checks(objective, template);
    let steps = match template {
        TemplateId::Generic => generic_steps(id, objective),
        TemplateId::Question => question_steps(id, objective),
        TemplateId::FileWork => file_work_steps(id, objective, &task.checks),
        TemplateId::Journal => journal_steps(id, objective, &task.checks),
        TemplateId::Manuscript | TemplateId::DocsTree => generic_steps(id, objective),
    };
    TaskSnapshot {
        task,
        steps,
        attempts: Vec::new(),
        check_results: Vec::new(),
        events: Vec::new(),
    }
}

pub fn concrete_paths(objective: &str) -> Vec<String> {
    objective
        .split_whitespace()
        .map(|word| word.trim_matches(|ch: char| ch == ',' || ch == '.' || ch == ':' || ch == ';'))
        .filter(|word| {
            word.contains('/')
                || word.contains(".md")
                || word.contains(".txt")
                || word.contains(".rs")
        })
        .filter(|word| !word.contains("://") && !word.contains(".."))
        .map(ToString::to_string)
        .collect()
}

fn task(id: u64, objective: &str, template: TemplateId) -> Task {
    Task {
        id,
        objective: objective.to_string(),
        template,
        state: TaskState::Open,
        brief: objective.to_string(),
        budget_used: 0,
        budget: 200,
        summary: String::new(),
        checks: Vec::new(),
    }
}

fn generic_steps(task_id: u64, objective: &str) -> Vec<Step> {
    vec![
        explore_step(task_id, 1, objective, 20),
        step(
            task_id,
            2,
            2,
            StepKind::Respond,
            "respond",
            "report the result",
        ),
    ]
}

fn question_steps(task_id: u64, objective: &str) -> Vec<Step> {
    if workspace_question(objective) {
        vec![
            explore_step(task_id, 1, objective, 6),
            step(
                task_id,
                2,
                2,
                StepKind::Respond,
                "answer",
                "answer from gathered facts only",
            ),
        ]
    } else {
        vec![step(task_id, 1, 1, StepKind::Respond, "answer", objective)]
    }
}

fn file_work_steps(task_id: u64, objective: &str, checks: &[CheckSpec]) -> Vec<Step> {
    vec![
        step(task_id, 1, 1, StepKind::Plan, "plan file work", objective),
        verify_step(task_id, 2, checks),
        step(
            task_id,
            3,
            3,
            StepKind::Respond,
            "respond",
            "summarize verified file work",
        ),
    ]
}

fn journal_steps(task_id: u64, objective: &str, checks: &[CheckSpec]) -> Vec<Step> {
    let mut write = step(task_id, 1, 1, StepKind::Write, "write journal", objective);
    write.output_path = Some("journal/today.md".to_string());
    write.inputs = "date=today existing_tail=".to_string();
    vec![
        write,
        verify_step(task_id, 2, checks),
        step(
            task_id,
            3,
            3,
            StepKind::Respond,
            "respond",
            "confirm verified journal update",
        ),
    ]
}

fn task_checks(objective: &str, template: TemplateId) -> Vec<CheckSpec> {
    let mut checks = concrete_paths(objective)
        .into_iter()
        .map(|path| CheckSpec::FileExists { path })
        .collect::<Vec<_>>();
    if template == TemplateId::Journal && checks.is_empty() {
        checks.push(CheckSpec::FileExists {
            path: "journal/today.md".to_string(),
        });
    }
    checks
}

fn explore_step(task_id: u64, id: u64, instruction: &str, budget: u32) -> Step {
    let mut step = step(
        task_id,
        id,
        id as u32,
        StepKind::Explore,
        "explore",
        instruction,
    );
    step.action_budget = budget;
    step
}

fn verify_step(task_id: u64, id: u64, checks: &[CheckSpec]) -> Step {
    let mut step = step(
        task_id,
        id,
        id as u32,
        StepKind::Verify,
        "verify outputs",
        "run deterministic checks",
    );
    step.checks = checks.to_vec();
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

fn workspace_question(objective: &str) -> bool {
    let lower = objective.to_ascii_lowercase();
    WORKSPACE_WORDS.iter().any(|needle| lower.contains(needle))
}
