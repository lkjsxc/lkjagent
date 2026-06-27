mod support;

use lkjagent_llm::wire::{ProviderAnomaly, ProviderAnomalyKind};
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::kernel::{
    build_snapshot, provider_anomaly_event, reduce, RuntimeMission, SnapshotAdapterInput,
};
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use lkjagent_store::queue;
use support::http::serve_responses;
use support::{runtime_state, store, temp_workspace, TestResult};

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

#[test]
fn provider_anomaly_sets_endpoint_retry_without_parse_fault() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "inspect state", "owner-send", "101")?;
    let workspace = temp_workspace("provider-anomaly-retry")?;
    let server = serve_responses(vec![empty_content_response()])?;
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(&server.base_url, "local-model", None, 180, 2_048),
        workspace,
        "100",
    );
    let mut daemon = ResidentDaemon::new(runtime_state()?, runtime);

    assert_eq!(
        daemon.poll_once(&mut conn, "101")?,
        DaemonTick::EndpointError
    );
    server.join()?;

    assert_eq!(daemon.state.parse_faults, 0);
    assert_eq!(daemon.endpoint_attempt, 1);
    assert!(daemon.endpoint_retry_at.is_some());
    Ok(())
}

#[test]
fn provider_anomaly_retry_budget_pauses_task() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "inspect state", "owner-send", "101")?;
    let workspace = temp_workspace("provider-anomaly-budget")?;
    let server = serve_responses(vec![
        empty_content_response(),
        empty_content_response(),
        empty_content_response(),
    ])?;
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(&server.base_url, "local-model", None, 180, 2_048),
        workspace,
        "100",
    );
    let mut daemon = ResidentDaemon::new(runtime_state()?, runtime);

    assert_eq!(
        daemon.poll_once(&mut conn, "101")?,
        DaemonTick::EndpointError
    );
    assert_eq!(
        daemon.poll_once(&mut conn, "104")?,
        DaemonTick::EndpointError
    );
    assert_eq!(daemon.poll_once(&mut conn, "109")?, DaemonTick::Paused);
    server.join()?;

    assert_eq!(daemon.state.parse_faults, 0);
    assert_eq!(daemon.endpoint_attempt, 3);
    assert!(daemon.endpoint_retry_at.is_none());
    assert!(matches!(daemon.state.task, TaskState::Paused { .. }));
    Ok(())
}

#[test]
fn provider_anomaly_classifier_output_becomes_kernel_event() -> TestResult<()> {
    let anomaly = ProviderAnomaly::new(
        ProviderAnomalyKind::ReasoningOnlyResponse,
        "reasoning-only response",
    );
    let event = provider_anomaly_event(&anomaly);
    let snapshot = build_snapshot(SnapshotAdapterInput {
        snapshot_id: 55,
        case_id: Some("case-55".to_string()),
        queue_head: Some("queue-55".to_string()),
        pending_owner_count: 1,
        provider_anomaly_class: Some(anomaly.kind.as_str().to_string()),
        ..SnapshotAdapterInput::default()
    })
    .map_err(|error| format!("{error:?}"))?;
    let decision = reduce(&snapshot, event).map_err(|error| format!("{error:?}"))?;
    assert_eq!(decision.mission, RuntimeMission::OwnerRecovery);
    assert_eq!(snapshot.provider.retry_count, 0);
    Ok(())
}

fn empty_content_response() -> String {
    "{\"choices\":[{\"message\":{\"content\":\"\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":5,\"completion_tokens\":9}}".to_string()
}
