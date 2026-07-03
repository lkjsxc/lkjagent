use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::model::{Step, StepKind, Task};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    pub system: String,
    pub user: String,
    pub fingerprint: String,
    pub max_tokens: u32,
    pub stop: String,
}

const HARD_CAP: usize = 8_000;
const STEP_FRAME: usize = 4_000;
const RETRY_FRAME: usize = 250;

pub fn render_prompt(task: &Task, steps: &[Step], step: &Step) -> Prompt {
    let tag = expected_block(step.kind);
    let system = format!(
        "lkjagent writes honestly. Objective: {}\nExpected: {tag}\n{}",
        task.objective,
        protocol(step.kind)
    );
    let digest = plan_digest(steps);
    let retry = if step.attempts_used > 0 {
        truncate(
            &format!(
                "\nRetry diagnosis: attempt {} must change shape or scope.",
                step.attempts_used
            ),
            RETRY_FRAME,
        )
    } else {
        String::new()
    };
    let frame = truncate(&step_frame(step), STEP_FRAME);
    let user = truncate(
        &format!("Plan:\n{digest}\n\nStep:\n{frame}{retry}"),
        HARD_CAP,
    );
    let fingerprint = fingerprint(&system, &user);
    Prompt {
        system,
        user,
        fingerprint,
        max_tokens: max_tokens(step.kind),
        stop: format!("</{tag}>"),
    }
}

fn plan_digest(steps: &[Step]) -> String {
    steps
        .iter()
        .map(|step| {
            format!(
                "{} {:?} {:?} {}",
                step.ordinal, step.kind, step.state, step.title
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn step_frame(step: &Step) -> String {
    format!(
        "title={}\ninstruction={}\ninputs={}\noutput={}",
        step.title,
        step.instruction,
        step.inputs,
        step.output_path.as_deref().unwrap_or("none")
    )
}

fn truncate(text: &str, cap_tokens: usize) -> String {
    let cap = cap_tokens.saturating_mul(4);
    if text.len() <= cap {
        return text.to_string();
    }
    let keep = cap.saturating_sub(5) / 2;
    let head = text.chars().take(keep).collect::<String>();
    let tail = text
        .chars()
        .rev()
        .take(keep)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("{head}[...]{tail}")
}

fn fingerprint(system: &str, user: &str) -> String {
    let mut hasher = DefaultHasher::new();
    system.hash(&mut hasher);
    user.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn max_tokens(kind: StepKind) -> u32 {
    match kind {
        StepKind::Write | StepKind::Revise => 3_000,
        StepKind::Plan => 900,
        StepKind::Explore => 500,
        StepKind::Respond | StepKind::Ask => 700,
        StepKind::Verify => 300,
    }
}

fn protocol(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Plan => "Return exactly <plan> lines </plan>. Lines: write PATH | TITLE | words=N, explore | GOAL | budget=N, or respond | SUMMARY. Use only relative paths.",
        StepKind::Write | StepKind::Revise => "Return exactly <content> prose </content>. Write the requested file body only. No analysis outside the block.",
        StepKind::Explore => "Return exactly <action>...</action> using one allowed tool, or <finish>summary</finish> when enough evidence is gathered.",
        StepKind::Respond | StepKind::Ask => "Return exactly <message>owner-facing answer</message>. Use gathered facts only.",
        StepKind::Verify => "Return exactly <verdict>pass or fail plus measured evidence</verdict>.",
    }
}

fn expected_block(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Write | StepKind::Revise => "content",
        StepKind::Plan => "plan",
        StepKind::Explore => "action",
        StepKind::Respond | StepKind::Ask => "message",
        StepKind::Verify => "verdict",
    }
}

#[cfg(test)]
mod tests {
    use crate::classify::instantiate;

    use super::*;

    #[test]
    fn retry_prompt_fingerprint_changes() {
        let mut snapshot = instantiate(1, "answer a workspace question");
        let step = match snapshot.steps.first().cloned() {
            Some(step) => step,
            None => return assert_eq!(snapshot.steps.len(), 1),
        };
        let before = render_prompt(&snapshot.task, &snapshot.steps, &step);
        snapshot.steps[0].attempts_used = 1;
        let after = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
        assert_ne!(before.fingerprint, after.fingerprint);
    }
}
