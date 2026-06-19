mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use lkjagent_store::{queue, state};
use support::http::{completion, serve_responses};
use support::{error_output, repeat_notice, runtime_state, store, temp_workspace, TestResult};

#[test]
fn third_parse_fault_waits_for_owner_guidance() -> TestResult<()> {
    let mut state = open("parse loop")?;
    for _ in 0..2 {
        state = parse_fault(state).state;
    }

    let waited = parse_fault(state);

    assert_eq!(waited.stop_reason, Some(StopReason::Ask));
    assert!(matches!(
        waited.state.task,
        TaskState::Waiting { ref question }
            if question.contains("Consecutive parse faults reached count=3")
    ));
    assert!(waited.effects.iter().any(|effect| {
        matches!(
            effect,
            lkjagent_runtime::step::Effect::RecordEvent { content, .. }
                if content.contains("Consecutive parse faults")
        )
    }));
    Ok(())
}

#[test]
fn third_repeat_action_waits_for_owner_guidance() -> TestResult<()> {
    let mut state = open("repeat loop")?;
    for _ in 0..3 {
        state = pending_read(state).state;
        state = step(state, StepInput::ToolOutput(repeat_notice())).state;
    }

    assert!(matches!(
        state.task,
        TaskState::Waiting { ref question }
            if question.contains("Consecutive repeated actions reached count=3")
    ));
    Ok(())
}

#[test]
fn third_tool_error_waits_and_owner_guidance_resets_faults() -> TestResult<()> {
    let mut state = open("tool loop")?;
    for _ in 0..3 {
        state = pending_read(state).state;
        state = step(state, StepInput::ToolOutput(error_output("missing file"))).state;
    }
    assert!(matches!(state.task, TaskState::Waiting { .. }));

    let resumed = step(
        state,
        StepInput::Owner {
            content: "use another path".to_string(),
            tokens: 4,
            graph: None,
            turn_budget: 5,
        },
    );

    assert_eq!(resumed.state.parse_faults, 0);
    assert_eq!(resumed.state.repeat_faults, 0);
    assert_eq!(resumed.state.tool_faults, 0);
    assert!(matches!(
        resumed.state.task,
        TaskState::Open { turns_remaining: 5 }
    ));
    Ok(())
}

#[test]
fn daemon_waits_after_three_endpoint_parse_faults() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "recover parse loop", "owner-send", "101")?;
    let workspace = temp_workspace("fault-wait-daemon")?;
    let server = serve_responses(vec![
        completion("not an act"),
        completion("still not an act"),
        completion("again not an act"),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Waiting);
    server.join()?;

    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("waiting".to_string())
    );
    assert!(state::get(&conn, "daemon question")?
        .is_some_and(|question| question.contains("Consecutive parse faults reached count=3")));
    Ok(())
}

fn open(content: &str) -> TestResult<lkjagent_runtime::task::RuntimeState> {
    Ok(step(
        runtime_state()?,
        StepInput::Owner {
            content: content.to_string(),
            tokens: 3,
            graph: None,
            turn_budget: 64,
        },
    )
    .state)
}

fn parse_fault(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "no act block".to_string(),
            tokens: 3,
        },
    )
}

fn pending_read(state: lkjagent_runtime::task::RuntimeState) -> lkjagent_runtime::step::StepResult {
    step(
        state,
        StepInput::Completion {
            content: "<act>\n<tool>fs.read</tool>\n<path>missing.md</path>\n</act>".to_string(),
            tokens: 10,
        },
    )
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
