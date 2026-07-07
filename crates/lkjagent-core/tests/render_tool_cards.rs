use lkjagent_core::classify::instantiate;
use lkjagent_core::parse::{parse_expected_for_decision, ParsedOutput};
use lkjagent_core::render::render_prompt_for_decision;
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_catalog::explore_tool_view;

#[test]
fn rendered_filled_tool_example_parses_and_admits() -> Result<(), String> {
    let snapshot = instantiate(3, "Survey workspace and report.");
    let mut decision = RuntimeDecision::new(
        "decision-1",
        "case-1",
        OperationKey("model.call/1".to_string()),
        explore_tool_view(),
        OutputEnvelope::Action,
    );
    decision.context_frame_fingerprint = "ctx-1".to_string();
    let prompt = render_prompt_for_decision(
        &snapshot.task,
        &snapshot.steps,
        &snapshot.steps[0],
        &decision,
    );
    let example = prompt
        .user
        .split("Safe filled example:\n")
        .last()
        .unwrap_or("");
    let parsed =
        parse_expected_for_decision(&decision, example).map_err(|fault| format!("{fault:?}"))?;
    let ParsedOutput::Action(action) = parsed else {
        return Err("example did not parse as action".to_string());
    };
    let admission = admit_action(
        &decision,
        &ModelAction {
            tool: action.tool,
            params: action.params.into_iter().collect(),
        },
    )
    .map_err(|error| error.message)?;

    assert!(prompt.user.contains("<lkjagent_action>"));
    assert!(prompt.user.contains("<name>path</name>"));
    assert!(prompt.user.contains("<value>README.md</value>"));
    assert_eq!(admission.status, AdmissionStatus::Admitted);
    Ok(())
}
