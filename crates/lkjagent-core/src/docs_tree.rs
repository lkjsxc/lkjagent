use crate::docs_tree_extract::parse_inputs;
pub use crate::docs_tree_extract::{extract, DocsFields};
use crate::model::{
    CheckSpec, Step, StepKind, StepState, Task, TaskSnapshot, TaskState, TemplateId,
};

pub fn instantiate(id: u64, objective: &str) -> TaskSnapshot {
    let fields = extract(objective);
    let checks = checks(&fields);
    let task = Task {
        id,
        objective: objective.to_string(),
        template: TemplateId::DocsTree,
        state: TaskState::Open,
        brief: format!("{} at {}", fields.topic, fields.root),
        budget_used: 0,
        budget: fields.page_count.unwrap_or(4) as u32 + 20,
        summary: String::new(),
        checks: checks.clone(),
    };
    let steps = vec![
        plan(id, objective, &fields),
        verify(id, checks),
        respond(id),
    ];
    TaskSnapshot {
        task,
        steps,
        attempts: Vec::new(),
        check_results: Vec::new(),
        events: Vec::new(),
    }
}

pub fn validate_plan(parent: &Step, steps: &[Step]) -> Result<(), String> {
    let fields = parse_inputs(&parent.inputs);
    let paths = steps
        .iter()
        .filter_map(|step| step.output_path.as_deref())
        .collect::<Vec<_>>();
    if paths
        .iter()
        .any(|path| !path.starts_with(&format!("{}/", fields.root)))
    {
        return Err("plan path escapes docs root".to_string());
    }
    for dir in dirs(&fields.root, &paths) {
        let readme = format!("{dir}/README.md");
        if !paths.iter().any(|path| *path == readme) {
            return Err(format!("missing README for {dir}"));
        }
    }
    if fields.exact {
        let pages = paths
            .iter()
            .filter(|path| !path.ends_with("README.md"))
            .count();
        if fields.page_count != Some(pages) {
            return Err(format!(
                "page count mismatch expected {:?} got {pages}",
                fields.page_count
            ));
        }
    }
    Ok(())
}

fn checks(fields: &DocsFields) -> Vec<CheckSpec> {
    let mut checks = vec![
        CheckSpec::ReadmeCoverage {
            root: fields.root.clone(),
        },
        CheckSpec::LinksResolve {
            root: fields.root.clone(),
        },
    ];
    if let (Some(count), true) = (fields.page_count, fields.exact) {
        checks.push(CheckSpec::FileCount {
            glob: format!("{}/*.md", fields.root),
            min: count + 1,
            max: Some(count + 1),
        });
    }
    checks
}

fn plan(task_id: u64, objective: &str, fields: &DocsFields) -> Step {
    let mut step = base(task_id, 1, 1, StepKind::Plan, "docs tree plan", objective);
    step.inputs = format!(
        "root={} pages={:?} exact={}",
        fields.root, fields.page_count, fields.exact
    );
    step
}

fn verify(task_id: u64, checks: Vec<CheckSpec>) -> Step {
    let mut step = base(
        task_id,
        2,
        2,
        StepKind::Verify,
        "verify docs tree",
        "check README and links",
    );
    step.checks = checks;
    step
}

fn respond(task_id: u64) -> Step {
    base(
        task_id,
        3,
        3,
        StepKind::Respond,
        "respond",
        "report docs tree",
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

fn dirs(root: &str, paths: &[&str]) -> Vec<String> {
    let mut dirs = vec![root.to_string()];
    for path in paths {
        let mut current = String::new();
        for part in path.split('/').take_while(|part| !part.ends_with(".md")) {
            current = if current.is_empty() {
                part.to_string()
            } else {
                format!("{current}/{part}")
            };
            if current.starts_with(root) && !dirs.contains(&current) {
                dirs.push(current.clone());
            }
        }
    }
    dirs
}
