use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_context::ContextFramePlan;
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
        stop: "</tool_call>".to_string(),
    };
    let plan = build_prompt_card_plan(
        &decision,
        &prompt,
        &ContextFramePlan {
            included: Vec::new(),
            excluded: Vec::new(),
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
