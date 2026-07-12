use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::model::{Attempt, Step, Task};
pub use crate::prompt_policy::max_tokens;
use crate::prompt_policy::{envelope_tag, expected_block, protocol, protocol_for_envelope};
use crate::runtime_decision::{OutputEnvelope, RuntimeDecision};
use crate::runtime_strategy::{instruction, prompt_cap};
use crate::runtime_tool_cards::{plan_example, protocol_card, render_tool_view};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    pub system: String,
    pub user: String,
    pub fingerprint: String,
    pub max_tokens: u32,
    pub stop: String,
}

const HARD_CAP: usize = 8_000;

#[rustfmt::skip]
pub fn render_prompt_for_decision(task: &Task, steps: &[Step], step: &Step,
    decision: &RuntimeDecision) -> Prompt {
    render_prompt_for_decision_with_attempts(task, steps, &[], step, decision)
}

pub fn render_prompt_for_decision_with_attempts(
    task: &Task,
    steps: &[Step],
    attempts: &[Attempt],
    step: &Step,
    decision: &RuntimeDecision,
) -> Prompt {
    let mut prompt = render_prompt(task, steps, step);
    let state = decision.harness_state.prompt_fragment();
    prompt.system = truncate(&format!("{}\n{state}", prompt.system), HARD_CAP);
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
    }
    if let Some(change) = instruction(&decision.recovery_policy) {
        prompt.user = format!("{}\n\nStrategy change: {change}", prompt.user);
    }
    prompt.user = truncate(
        &format!("{}\n\n{}", prompt.user, protocol_card(decision)),
        HARD_CAP,
    );
    if let Some(frame) = recovery_frame(decision, attempts, step.id) {
        prompt.user = truncate(&format!("{}\n\n{frame}", prompt.user), HARD_CAP);
    }
    prompt.user = truncate(&prompt.user, prompt_cap(&decision.recovery_policy));
    prompt.fingerprint = fingerprint(&prompt.system, &prompt.user);
    prompt
}

pub fn render_prompt(task: &Task, steps: &[Step], step: &Step) -> Prompt {
    let tag = expected_block(step.kind);
    let brief = truncate(&task.brief, 450);
    let system = format!(
        "lkjagent writes honestly. Objective: {}\nMatter brief:\n{}\nExpected: {tag}\n{}",
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
            250,
        )
    } else {
        String::new()
    };
    let frame = truncate(&step_frame(step), 4_000);
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

fn recovery_frame(
    decision: &RuntimeDecision,
    attempts: &[Attempt],
    step_id: u64,
) -> Option<String> {
    let attempt = attempts
        .iter()
        .rev()
        .find(|attempt| attempt.step_id == step_id && !attempt.diagnosis.trim().is_empty())?;
    let tag = envelope_tag(decision.expected_envelope).unwrap_or("no_output");
    let excerpt_hash = fingerprint(&decision.id, &attempt.diagnosis);
    let repair = repair_shape(decision, tag);
    Some(truncate(
        &format!(
            "Recovery frame:\ndecision={} strategy={} attempt={} fault={} invalid_excerpt_hash={}\nOutput only a corrected envelope or ask plainly for missing information.\n{}",
            decision.id, decision.recovery_policy, attempt.ordinal, attempt.diagnosis, excerpt_hash, repair
        ),
        250,
    ))
}

fn repair_shape(decision: &RuntimeDecision, tag: &str) -> String {
    if decision.expected_envelope == OutputEnvelope::Plan {
        return format!("Corrected parser-valid plan example:\n{}", plan_example());
    }
    if decision.expected_envelope != OutputEnvelope::Action {
        return format!("Next expected envelope: <{tag}>...</{tag}>");
    }
    let tool = decision
        .tool_view
        .entries
        .first()
        .map_or("TOOL", |entry| entry.name.as_str());
    format!(
        "Corrected minimal action shape:\n<tool_call><tool>{tool}</tool><input></input></tool_call>"
    )
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
