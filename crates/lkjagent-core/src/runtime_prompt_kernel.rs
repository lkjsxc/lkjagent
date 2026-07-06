use serde::{Deserialize, Serialize};

use crate::render::Prompt;
use crate::runtime_context_plan::ContextFramePlan;
use crate::runtime_decision::RuntimeDecision;
use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

pub const PROMPT_PROFILE: &str = "kernel-v1";
pub const CONTEXT_PROFILE: &str = "clean-context-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptCard {
    pub id: String,
    pub kind: String,
    pub reason: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptCardPlan {
    pub prompt_profile: String,
    pub context_profile: String,
    pub cards: Vec<PromptCard>,
    pub fingerprint: String,
}

pub fn build_prompt_card_plan(
    decision: &RuntimeDecision,
    prompt: &Prompt,
    context_plan: &ContextFramePlan,
) -> Result<PromptCardPlan, FingerprintError> {
    let tool_fingerprint = decision.tool_view_fingerprint()?;
    let cards = vec![
        card(
            "kernel",
            format!(
                "case={} decision={} prompt-profile={} context-profile={}",
                decision.case_id, decision.id, PROMPT_PROFILE, CONTEXT_PROFILE
            ),
        )?,
        card("objective", format!("operation={}", decision.operation.0))?,
        card(
            "state",
            format!(
                "snapshot={} state={}",
                decision.snapshot_fingerprint, decision.state_vector_fingerprint
            ),
        )?,
        card("facts", facts_reason(decision, context_plan))?,
        card("conflicts", conflict_reason(context_plan))?,
        card("recovery", format!("policy={}", decision.recovery_policy))?,
        card(
            "tools",
            format!(
                "tool-view={} count={}",
                tool_fingerprint,
                decision.tool_view.entries.len()
            ),
        )?,
        card(
            "output",
            format!(
                "envelope={:?} stop={} max={}",
                decision.expected_envelope, prompt.stop, prompt.max_tokens
            ),
        )?,
    ];
    let fingerprint = stable_fingerprint(&PlanSeed {
        prompt_profile: PROMPT_PROFILE,
        context_profile: CONTEXT_PROFILE,
        cards: &cards,
    })?;
    Ok(PromptCardPlan {
        prompt_profile: PROMPT_PROFILE.to_string(),
        context_profile: CONTEXT_PROFILE.to_string(),
        cards,
        fingerprint,
    })
}

fn facts_reason(decision: &RuntimeDecision, plan: &ContextFramePlan) -> String {
    format!(
        "context={} included={} excluded={}",
        decision.context_frame_fingerprint,
        entry_list(&plan.included),
        entry_list(&plan.excluded)
    )
}

fn conflict_reason(plan: &ContextFramePlan) -> String {
    let unresolved = plan
        .excluded
        .iter()
        .filter(|entry| entry.reason == "unresolved-conflict")
        .cloned()
        .collect::<Vec<_>>();
    format!("unresolved={}", entry_list(&unresolved))
}

fn entry_list(entries: &[crate::runtime_context::ContextPlanEntry]) -> String {
    if entries.is_empty() {
        return "none".to_string();
    }
    let mut values = entries
        .iter()
        .map(|entry| format!("{}:{}", entry.item_id, entry.reason))
        .collect::<Vec<_>>();
    values.sort();
    values.join(",")
}

fn card(kind: &str, reason: String) -> Result<PromptCard, FingerprintError> {
    let id = format!("card-{kind}");
    let fingerprint = stable_fingerprint(&CardSeed {
        kind,
        reason: &reason,
    })?;
    Ok(PromptCard {
        id,
        kind: kind.to_string(),
        reason,
        fingerprint,
    })
}

#[derive(Serialize)]
struct CardSeed<'a> {
    kind: &'a str,
    reason: &'a str,
}

#[derive(Serialize)]
struct PlanSeed<'a> {
    prompt_profile: &'a str,
    context_profile: &'a str,
    cards: &'a [PromptCard],
}
