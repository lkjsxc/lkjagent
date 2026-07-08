use crate::engine::Command;
use crate::model::{
    CheckResult, CheckSpec, Event, EventKind, StepKind, StepState, TaskSnapshot, TaskState,
    TemplateId,
};

pub(crate) fn close_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>) {
    if let Some(reason) = completion_blocker(snapshot) {
        block_task(snapshot, commands, &reason);
        return;
    }
    snapshot.task.state = TaskState::Closed;
    record_event(
        commands,
        EventKind::TaskClosed,
        snapshot.task.summary.clone(),
    );
}

pub(crate) fn completion_blocker(snapshot: &TaskSnapshot) -> Option<String> {
    for (index, step) in snapshot.steps.iter().enumerate() {
        if step.state == StepState::Done || skipped_is_superseded(snapshot, index) {
            continue;
        }
        return Some(format!(
            "step {} {} is {}",
            step.id,
            step.title,
            step_state(step.state)
        ));
    }
    check_blocker(snapshot)
}

fn skipped_is_superseded(snapshot: &TaskSnapshot, index: usize) -> bool {
    let step = &snapshot.steps[index];
    if step.state != StepState::Skipped || step.kind != StepKind::Verify {
        return false;
    }
    let later = snapshot.steps.iter().skip(index + 1).collect::<Vec<_>>();
    let repair_done = later.iter().any(|item| {
        matches!(item.kind, StepKind::Write | StepKind::Revise) && item.state == StepState::Done
    });
    let verify_done = later.iter().any(|item| {
        item.kind == StepKind::Verify && item.state == StepState::Done && item.checks == step.checks
    });
    repair_done && verify_done
}

fn check_blocker(snapshot: &TaskSnapshot) -> Option<String> {
    if snapshot.task.checks.is_empty() {
        return artifact_evidence_required(snapshot.task.template)
            .then(|| "artifact evidence missing".to_string());
    }
    for spec in &snapshot.task.checks {
        if !snapshot
            .check_results
            .iter()
            .any(|result| check_matches(result, spec))
        {
            return Some(format!("task check missing: {}", check_name(spec)));
        }
    }
    if artifact_response_path_missing(snapshot) {
        return Some("artifact response path missing".to_string());
    }
    snapshot
        .check_results
        .iter()
        .find(|result| !result.passed)
        .map(|result| format!("task check failed: {}", result.name))
}

fn artifact_response_path_missing(snapshot: &TaskSnapshot) -> bool {
    snapshot
        .steps
        .iter()
        .filter_map(|step| step.output_path.as_ref())
        .filter(|path| path.starts_with("artifacts/"))
        .any(|path| !snapshot.task.summary.contains(path))
}

fn check_matches(result: &CheckResult, spec: &CheckSpec) -> bool {
    let decision = result
        .decision_id
        .as_deref()
        .is_some_and(|id| !id.is_empty());
    let evidence =
        matches!(spec, CheckSpec::Judged { .. }) || result.evidence_fingerprint.is_some();
    result.passed
        && decision
        && evidence
        && refs_present(result, spec)
        && result.name == check_name(spec)
        && result.params.as_ref() == Some(spec)
}

fn refs_present(result: &CheckResult, spec: &CheckSpec) -> bool {
    matches!(spec, CheckSpec::Command { .. }) || !result.artifact_refs.is_empty()
}

fn check_name(spec: &CheckSpec) -> &'static str {
    match spec {
        CheckSpec::FileExists { .. } => "file_exists",
        CheckSpec::MinWords { .. } => "min_words",
        CheckSpec::MinWordsTotal { .. } => "min_words_total",
        CheckSpec::MaxLines { .. } => "max_lines",
        CheckSpec::FileCount { .. } => "file_count",
        CheckSpec::Contains { .. } => "contains",
        CheckSpec::Absent { .. } => "absent",
        CheckSpec::ReadmeCoverage { .. } => "readme_coverage",
        CheckSpec::LinksResolve { .. } => "links_resolve",
        CheckSpec::Command { .. } => "command",
        CheckSpec::Judged { .. } => "judged",
    }
}

fn artifact_evidence_required(template: TemplateId) -> bool {
    matches!(
        template,
        TemplateId::DocsTree
            | TemplateId::FileWork
            | TemplateId::Journal
            | TemplateId::LegacyArtifact
    )
}

fn step_state(state: StepState) -> &'static str {
    match state {
        StepState::Pending => "pending",
        StepState::Active => "active",
        StepState::Done => "done",
        StepState::Blocked => "blocked",
        StepState::Skipped => "skipped without supersession evidence",
    }
}

pub(crate) fn block_task(snapshot: &mut TaskSnapshot, commands: &mut Vec<Command>, reason: &str) {
    snapshot.task.state = TaskState::Blocked;
    snapshot.task.summary = reason.to_string();
    record_event(commands, EventKind::TaskBlocked, reason.to_string());
}

pub(crate) fn record_event(commands: &mut Vec<Command>, kind: EventKind, content: String) {
    commands.push(Command::RecordEvent(Event { kind, content }));
}
