pub use crate::manuscript_extract::{extract, ManuscriptFields};
use crate::model::{
    CheckSpec, Step, StepKind, StepState, Task, TaskSnapshot, TaskState, TemplateId,
};

pub fn instantiate(id: u64, objective: &str) -> TaskSnapshot {
    let fields = extract(objective);
    let checks = checks(&fields);
    let task = Task {
        id,
        objective: objective.to_string(),
        template: TemplateId::Manuscript,
        state: TaskState::Open,
        brief: format!("{} at {}", fields.title, fields.root),
        budget_used: 0,
        budget: fields.chapter_count as u32 + 40,
        summary: fields.note.clone().unwrap_or_default(),
        checks: checks.clone(),
    };
    let mut steps = vec![plan(id, objective, &fields), settings(id, &fields)];
    steps.extend(chapter_steps(id, &fields));
    let next = steps.len() as u32 + 1;
    steps.push(verify(id, next, checks));
    steps.push(respond(id, next + 1));
    TaskSnapshot {
        task,
        steps,
        attempts: Vec::new(),
        check_results: Vec::new(),
        events: Vec::new(),
    }
}

pub fn chapter_plan(fields: &ManuscriptFields) -> Vec<String> {
    let per = fields.total_words.div_ceil(fields.chapter_count);
    (1..=fields.chapter_count)
        .map(|index| chapter_line(fields, index, per))
        .collect()
}

fn chapter_line(fields: &ManuscriptFields, index: usize, words: usize) -> String {
    format!(
        "write {}/manuscript/chapter-{index:02}.md | Chapter {index} | words={words}",
        fields.root
    )
}

fn checks(fields: &ManuscriptFields) -> Vec<CheckSpec> {
    let mut checks = vec![
        CheckSpec::FileCount {
            glob: fields.glob.clone(),
            min: fields.chapter_count,
            max: Some(fields.chapter_count),
        },
        CheckSpec::MinWordsTotal {
            glob: fields.glob.clone(),
            n: fields.total_words,
        },
    ];
    for path in chapter_paths(fields) {
        for needle in ["to be written", "TODO", "placeholder"] {
            checks.push(CheckSpec::Absent {
                path: path.clone(),
                needle: needle.to_string(),
            });
        }
    }
    checks
}

fn chapter_paths(fields: &ManuscriptFields) -> Vec<String> {
    (1..=fields.chapter_count)
        .map(|index| format!("{}/manuscript/chapter-{index:02}.md", fields.root))
        .collect()
}

fn plan(task_id: u64, objective: &str, fields: &ManuscriptFields) -> Step {
    let mut step = base(task_id, 1, 1, StepKind::Plan, "outline", objective);
    step.inputs = chapter_plan(fields).join("\n");
    step.state = StepState::Done;
    step
}

fn chapter_steps(task_id: u64, fields: &ManuscriptFields) -> Vec<Step> {
    let per = fields.total_words.div_ceil(fields.chapter_count);
    chapter_paths(fields)
        .into_iter()
        .enumerate()
        .map(|(index, path)| chapter_step(task_id, index as u32 + 3, path, per))
        .collect()
}

fn chapter_step(task_id: u64, ordinal: u32, path: String, words: usize) -> Step {
    let mut step = base(
        task_id,
        ordinal as u64,
        ordinal,
        StepKind::Write,
        "chapter",
        "",
    );
    step.title = format!("chapter {:02}", ordinal.saturating_sub(2));
    step.instruction =
        format!("write one bounded 350-word unit toward a {words}-word chapter, then stop");
    step.output_path = Some(path);
    step
}

fn settings(task_id: u64, fields: &ManuscriptFields) -> Step {
    let mut step = base(
        task_id,
        2,
        2,
        StepKind::Write,
        "settings",
        "write premise and facts",
    );
    step.output_path = Some(format!("{}/settings.md", fields.root));
    step
}

fn verify(task_id: u64, ordinal: u32, checks: Vec<CheckSpec>) -> Step {
    let mut step = base(
        task_id,
        ordinal as u64,
        ordinal,
        StepKind::Verify,
        "verify manuscript",
        "measure manuscript",
    );
    step.checks = checks;
    step
}

fn respond(task_id: u64, ordinal: u32) -> Step {
    base(
        task_id,
        ordinal as u64,
        ordinal,
        StepKind::Respond,
        "respond",
        "report measured paths and words",
    )
}

fn base(
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
