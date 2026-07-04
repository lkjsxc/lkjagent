use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::model::{Step, Task};
pub use crate::prompt_policy::max_tokens;

use crate::prompt_policy::{envelope_tag, expected_block, protocol, protocol_for_envelope};
use crate::runtime_decision::{RuntimeDecision, ToolSetView};

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
        prompt.system = prompt
            .system
            .replace(
                &format!("Expected: {}", expected_block(step.kind)),
                &format!("Expected: {tag}"),
            )
            .replace(
                protocol(step.kind),
                protocol_for_envelope(decision.expected_envelope),
            );
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
