mod support;

use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::StopReason;
use support::{runtime_state, TestResult};

#[test]
fn provider_anomaly_does_not_increment_parse_faults() -> TestResult<()> {
    let state = runtime_state()?;

    let result = step(
        state,
        StepInput::ProviderAnomaly(
            "empty_content_with_usage".to_string(),
            "empty content with nonzero completion tokens".to_string(),
        ),
    );

    assert_eq!(result.stop_reason, Some(StopReason::EndpointError));
    assert_eq!(result.state.parse_faults, 0);
    assert!(result.state.context.log.iter().any(|frame| {
        frame
            .content
            .contains("provider anomaly: empty_content_with_usage")
    }));
    Ok(())
}
