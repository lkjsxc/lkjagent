use lkjagent_core::model::{AttemptOutcome, EventKind, StepKind, StepState, TaskState, TemplateId};

use crate::error::{StoreError, StoreResult};

pub fn template(value: &str) -> StoreResult<TemplateId> {
    match value {
        "generic" => Ok(TemplateId::Generic),
        "question" => Ok(TemplateId::Question),
        "manuscript" => Ok(TemplateId::Manuscript),
        "docstree" => Ok(TemplateId::DocsTree),
        "filework" => Ok(TemplateId::FileWork),
        "journal" => Ok(TemplateId::Journal),
        _ => Err(invalid("template", value)),
    }
}

pub fn task_state(value: &str) -> StoreResult<TaskState> {
    match value {
        "open" => Ok(TaskState::Open),
        "waiting" => Ok(TaskState::Waiting),
        "blocked" => Ok(TaskState::Blocked),
        "closed" => Ok(TaskState::Closed),
        _ => Err(invalid("task state", value)),
    }
}

pub fn step_kind(value: &str) -> StoreResult<StepKind> {
    match value {
        "plan" => Ok(StepKind::Plan),
        "write" => Ok(StepKind::Write),
        "revise" => Ok(StepKind::Revise),
        "explore" => Ok(StepKind::Explore),
        "verify" => Ok(StepKind::Verify),
        "respond" => Ok(StepKind::Respond),
        "ask" => Ok(StepKind::Ask),
        _ => Err(invalid("step kind", value)),
    }
}

pub fn step_state(value: &str) -> StoreResult<StepState> {
    match value {
        "pending" => Ok(StepState::Pending),
        "active" => Ok(StepState::Active),
        "done" => Ok(StepState::Done),
        "blocked" => Ok(StepState::Blocked),
        "skipped" => Ok(StepState::Skipped),
        _ => Err(invalid("step state", value)),
    }
}

pub fn attempt_outcome(value: &str) -> StoreResult<AttemptOutcome> {
    match value {
        "ok" => Ok(AttemptOutcome::Ok),
        "parsefault" | "parse_fault" => Ok(AttemptOutcome::ParseFault),
        "checkfail" | "check_fail" => Ok(AttemptOutcome::CheckFail),
        "effecterror" | "effect_error" => Ok(AttemptOutcome::EffectError),
        "endpointerror" | "endpoint_error" => Ok(AttemptOutcome::EndpointError),
        _ => Err(invalid("attempt outcome", value)),
    }
}

pub fn event_kind(value: &str) -> StoreResult<EventKind> {
    match value {
        "owner" => Ok(EventKind::Owner),
        "stepdone" | "step_done" => Ok(EventKind::StepDone),
        "stepblocked" | "step_blocked" => Ok(EventKind::StepBlocked),
        "taskclosed" | "task_closed" => Ok(EventKind::TaskClosed),
        "taskblocked" | "task_blocked" => Ok(EventKind::TaskBlocked),
        "question" => Ok(EventKind::Question),
        "answer" => Ok(EventKind::Answer),
        "notice" => Ok(EventKind::Notice),
        _ => Err(invalid("event kind", value)),
    }
}

fn invalid(name: &str, value: &str) -> StoreError {
    StoreError::InvalidState(format!("unknown {name}: {value}"))
}
