use crate::checks::FileFact;
use crate::engine::Command;
use crate::model::{CheckResult, CheckSpec, Event, EventKind, Step, StepKind, StepState};

pub(crate) fn shortfall_steps(
    step: &Step,
    files: &[FileFact],
    results: &[CheckResult],
) -> Vec<Step> {
    if results.iter().all(|result| result.passed) {
        return Vec::new();
    }
    if let Some(steps) = manuscript_shortfall(step, files, results) {
        return steps;
    }
    if results
        .iter()
        .any(|result| result.name == "links_resolve" && !result.passed)
    {
        let path = files
            .iter()
            .find(|fact| fact.content.contains("missing.md"))
            .or_else(|| files.iter().find(|fact| fact.content.contains("](")))
            .or_else(|| files.first())
            .map(|fact| fact.path.clone())
            .unwrap_or_else(|| "README.md".to_string());
        return vec![revise_step(step, &path), verify_step(step)];
    }
    Vec::new()
}

fn manuscript_shortfall(
    step: &Step,
    files: &[FileFact],
    results: &[CheckResult],
) -> Option<Vec<Step>> {
    let CheckSpec::MinWordsTotal { glob, n } = step
        .checks
        .iter()
        .find(|spec| matches!(spec, CheckSpec::MinWordsTotal { .. }))?
    else {
        return None;
    };
    let measured = results
        .iter()
        .find(|result| result.name == "min_words_total")
        .and_then(|result| result.measured.parse::<usize>().ok())
        .unwrap_or(0);
    let path = last_matching(files, glob).unwrap_or_else(|| fallback_path(glob));
    let missing = n.saturating_sub(measured).max(1);
    Some(vec![write_step(step, &path, missing), verify_step(step)])
}

pub(crate) fn split_after_fault(step: &Step) -> Vec<Step> {
    if step.kind != StepKind::Write || step.attempts_used < 3 {
        return Vec::new();
    }
    let Some(path) = &step.output_path else {
        return Vec::new();
    };
    if !path.contains("/manuscript/chapter-") {
        return Vec::new();
    }
    vec![write_step(step, path, 250)]
}

pub(crate) fn add_steps(commands: &mut Vec<Command>, steps: Vec<Step>, notice: &str) {
    if steps.is_empty() {
        return;
    }
    commands.push(Command::RecordEvent(Event {
        kind: EventKind::Notice,
        content: notice.to_string(),
    }));
    commands.push(Command::AddSteps(steps));
}

pub(crate) fn insert_after(
    snapshot: &mut crate::model::TaskSnapshot,
    index: usize,
    additions: &[Step],
) {
    for (offset, step) in additions.iter().cloned().enumerate() {
        snapshot.steps.insert(index + 1 + offset, step);
    }
}

fn write_step(parent: &Step, path: &str, words: usize) -> Step {
    let mut step = clone_step(
        parent,
        parent.id.saturating_mul(10).saturating_add(1),
        StepKind::Write,
    );
    step.title = "manuscript extension".to_string();
    step.instruction = format!("append at least {words} continuation words");
    step.output_path = Some(path.to_string());
    step.checks.clear();
    step
}

fn revise_step(parent: &Step, path: &str) -> Step {
    let mut step = clone_step(
        parent,
        parent.id.saturating_mul(10).saturating_add(3),
        StepKind::Revise,
    );
    step.title = "docs link repair".to_string();
    step.instruction = "repair links reported by verification".to_string();
    step.output_path = Some(path.to_string());
    step.checks.clear();
    step
}

fn verify_step(parent: &Step) -> Step {
    let mut step = clone_step(
        parent,
        parent.id.saturating_mul(10).saturating_add(2),
        StepKind::Verify,
    );
    step.title = "verify manuscript extension".to_string();
    step.checks = parent.checks.clone();
    step
}

fn clone_step(parent: &Step, id: u64, kind: StepKind) -> Step {
    let mut step = parent.clone();
    step.id = id;
    step.kind = kind;
    step.state = StepState::Pending;
    step.attempts_used = 0;
    step.actions_used = 0;
    step
}

fn last_matching(files: &[FileFact], glob: &str) -> Option<String> {
    let mut paths = files
        .iter()
        .filter(|fact| glob_match(glob, &fact.path))
        .map(|fact| fact.path.clone())
        .collect::<Vec<_>>();
    paths.sort();
    paths.pop()
}

fn fallback_path(glob: &str) -> String {
    glob.replace("*.md", "chapter-01.md")
}

fn glob_match(glob: &str, path: &str) -> bool {
    if let Some((prefix, suffix)) = glob.split_once('*') {
        path.starts_with(prefix) && path.ends_with(suffix)
    } else {
        path == glob
    }
}
