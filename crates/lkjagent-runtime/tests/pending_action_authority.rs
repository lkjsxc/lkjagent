mod support;

use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::PendingActionAuthority;
use support::{runtime_state, TestResult};

#[test]
fn authorized_completion_carries_authority_into_pending_action() -> TestResult<()> {
    let state = runtime_state()?;
    let authority = PendingActionAuthority {
        authority_decision_id: Some("decision-7".to_string()),
        prompt_frame_id: Some("frame-9".to_string()),
        staleness_fingerprint: Some("stale-11".to_string()),
    };

    let result = step(
        state,
        StepInput::AuthorizedCompletion(
            "<action>\n<tool>graph.state</tool>\n</action>".to_string(),
            8,
            authority,
        ),
    );

    let pending = result
        .state
        .pending_action
        .ok_or("missing pending action")?;
    assert_eq!(pending.authority_decision_id.as_deref(), Some("decision-7"));
    assert_eq!(pending.prompt_frame_id.as_deref(), Some("frame-9"));
    assert_eq!(pending.staleness_fingerprint.as_deref(), Some("stale-11"));
    Ok(())
}
