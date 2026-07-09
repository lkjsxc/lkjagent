use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::{ContextFramePlan, ContextLanePlan, ContextPlanEntry};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
use lkjagent_core::runtime_prompt_kernel::build_prompt_card_plan;

#[test]
fn card_plan_has_ordered_profiles_and_fingerprints() -> Result<(), String> {
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("finish", "finish").with_params(vec!["summary"], Vec::new())
        ]),
        OutputEnvelope::Action,
    );
    decision.recovery_policy = "retry-same-decision".to_string();
    let prompt = Prompt {
        system: "system".to_string(),
        user: "user".to_string(),
        fingerprint: "prompt-fp".to_string(),
        max_tokens: 500,
        stop: "</lkjagent_action>".to_string(),
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
    let recovery = reason(&plan, "recovery")?;
    assert!(recovery.starts_with("policy-ref=fnv1a64:"));
    assert!(!recovery.contains("retry-same-decision"));
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
            lanes: vec![ContextLanePlan {
                name: "relevant-records".to_string(),
                budget_tokens: 1200,
                source_refs: vec!["record:records/life/notes/a.md".to_string()],
                included_item_ids: vec!["ctx-good".to_string()],
                excluded_item_ids: Vec::new(),
                fingerprint: "lane-fp".to_string(),
            }],
        },
    )
    .map_err(|error| error.message)?;

    let facts = reason(&plan, "facts")?;
    assert!(facts.contains("<context_item>"));
    assert!(facts.contains("<id>ctx-good</id>"));
    assert!(facts.contains("<reason>clean-current</reason>"));
    assert!(facts.contains("<rank>7</rank>"));
    assert!(facts.contains("<source_ref>test:source@fp</source_ref>"));
    assert!(facts.contains("<reason>contamination:FailedModelOutput</reason>"));
    assert!(facts.contains("<reason>unresolved-conflict</reason>"));
    assert!(facts.contains("relevant-records:lane-fp"));
    assert!(facts.contains("refs=record:records/life/notes/a.md"));
    let conflicts = reason(&plan, "conflicts")?;
    assert!(conflicts.contains("<id>ctx-conflict</id>"));
    assert!(conflicts.contains("<source_ref>test:source@fp</source_ref>"));
    Ok(())
}

fn entry(item_id: &str, reason: &str) -> ContextPlanEntry {
    ContextPlanEntry {
        item_id: item_id.to_string(),
        reason: reason.to_string(),
        rank: 7,
        source_ref: "test:source@fp".to_string(),
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
