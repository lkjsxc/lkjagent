mod support;

use lkjagent_runtime::step::{step, Effect, StepInput};
use lkjagent_runtime::task::StopReason;
use support::{runtime_state, TestResult};

#[test]
fn implicit_envelope_completion_records_normalization_notice() -> TestResult<()> {
    let owner = step(
        runtime_state()?,
        StepInput::Owner {
            content: "inspect state".to_string(),
            tokens: 3,
            graph: None,
            turn_budget: 64,
        },
    );
    let result = step(
        owner.state,
        StepInput::Completion {
            content: "<tool>graph.state</tool>".to_string(),
            tokens: 4,
        },
    );

    assert_eq!(result.stop_reason, Some(StopReason::Acted));
    assert!(result.effects.iter().any(|effect| {
        matches!(effect, Effect::RecordEvent { content, .. }
            if content.contains("envelope_mode=ImplicitActionEnvelope"))
    }));
    assert!(result
        .state
        .context
        .log
        .iter()
        .any(|frame| { frame.content.contains("parse normalization recorded") }));
    Ok(())
}
