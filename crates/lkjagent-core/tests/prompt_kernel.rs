use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::{ContextFramePlan, ContextPlanEntry};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_prompt_kernel::build_prompt_card_plan;

#[test]
fn card_plan_has_ordered_profiles_and_fingerprints() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("finish", "finish").with_params(vec!["summary"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    let prompt = Prompt {
        system: "system".to_string(),
        user: "user".to_string(),
        fingerprint: "prompt-fp".to_string(),
        max_tokens: 500,
        stop: "</lkjagent_action_v2>".to_string(),
    };
    let plan = build_prompt_card_plan(
        &decision,
        &prompt,
        &ContextFramePlan {
            included: Vec::new(),
            excluded: Vec::new(),
            lanes: Vec::new(),
        },
    )
    .map_err(|error| error.message)?;

    let kinds = plan
        .cards
        .iter()
        .map(|card| card.kind.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        vec![
            "kernel",
            "objective",
            "state",
            "facts",
            "conflicts",
            "recovery",
            "tools",
            "output"
        ]
    );
    assert_eq!(plan.prompt_profile, "kernel-v1");
    assert!(plan.fingerprint.starts_with("fnv1a64:"));
    Ok(())
}

#[test]
fn card_reasons_list_context_selection_audit() -> Result<(), String> {
    let decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::empty(),
        OutputEnvelope::Message,
    );
    let prompt = Prompt {
        system: "system".to_string(),
        user: "user".to_string(),
        fingerprint: "prompt-fp".to_string(),
        max_tokens: 500,
        stop: "</message>".to_string(),
    };
    let plan = build_prompt_card_plan(
        &decision,
        &prompt,
        &ContextFramePlan {
            included: vec![entry("ctx-good", "clean-current")],
            excluded: vec![
                entry("ctx-bad", "contamination:FailedModelOutput"),
                entry("ctx-conflict", "unresolved-conflict"),
            ],
            lanes: Vec::new(),
        },
    )
    .map_err(|error| error.message)?;

    let facts = reason(&plan, "facts")?;
    assert!(facts.contains("ctx-good:clean-current"));
    assert!(facts.contains("ctx-bad:contamination:FailedModelOutput"));
    assert!(facts.contains("ctx-conflict:unresolved-conflict"));
    assert!(reason(&plan, "conflicts")?.contains("ctx-conflict:unresolved-conflict"));
    Ok(())
}

fn entry(item_id: &str, reason: &str) -> ContextPlanEntry {
    ContextPlanEntry {
        item_id: item_id.to_string(),
        reason: reason.to_string(),
    }
}

fn reason<'a>(
    plan: &'a lkjagent_core::runtime_prompt_kernel::PromptCardPlan,
    kind: &str,
) -> Result<&'a str, String> {
    plan.cards
        .iter()
        .find(|card| card.kind == kind)
        .map(|card| card.reason.as_str())
        .ok_or_else(|| format!("missing card {kind}"))
}
