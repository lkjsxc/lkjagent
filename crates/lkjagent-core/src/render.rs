use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::model::{Step, StepKind, Task};
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision, ToolSetView};

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

pub fn render_prompt_for_decision(
    task: &Task,
    steps: &[Step],
    step: &Step,
    decision: &RuntimeDecision,
) -> Prompt {
    let mut prompt = render_prompt(task, steps, step);
    if let Some(max_tokens) = decision.model_budget_tokens {
        prompt.max_tokens = max_tokens;
    }
    if let Some(tag) = envelope_tag(decision.expected_envelope) {
        prompt.stop = format!("</{tag}>");
    }
    let rendered_view = render_tool_view(&decision.tool_view);
    if !rendered_view.is_empty() {
        prompt.user = truncate(
            &format!(
                "{}\n\nTool view from decision:\n{rendered_view}",
                prompt.user
            ),
            HARD_CAP,
        );
        prompt.fingerprint = fingerprint(&prompt.system, &prompt.user);
    }
    prompt
}

pub fn render_prompt(task: &Task, steps: &[Step], step: &Step) -> Prompt {
    let tag = expected_block(step.kind);
    let brief = truncate(&task.brief, 450);
    let system = format!(
        "lkjagent writes honestly. Objective: {}\nTask brief:\n{}\nExpected: {tag}\n{}",
        task.objective,
        brief,
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
        StepKind::Write | StepKind::Revise => 2_400,
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
        StepKind::Explore => "Return exactly <action>...</action> using one allowed tool. To finish, use <tool>finish</tool> with <summary>...</summary>.",
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

fn envelope_tag(envelope: OutputEnvelope) -> Option<&'static str> {
    match envelope {
        OutputEnvelope::Content => Some("content"),
        OutputEnvelope::Plan => Some("plan"),
        OutputEnvelope::Action => Some("action"),
        OutputEnvelope::Message => Some("message"),
        OutputEnvelope::Verdict => Some("verdict"),
        OutputEnvelope::None => None,
    }
}

fn render_tool_view(view: &ToolSetView) -> String {
    view.entries
        .iter()
        .map(|entry| {
            format!(
                "- {}: {} required={} optional={}",
                entry.name,
                entry.purpose,
                entry.required_params.join(","),
                entry.optional_params.join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
