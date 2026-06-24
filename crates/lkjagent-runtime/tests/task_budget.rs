mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, startup_state_with_budget, take_daemon_lock, DaemonTick, ResidentDaemon,
    ResidentRuntime,
};
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::queue;
use support::http::{completion, serve_responses};
use support::{prefix, runtime_state, store, temp_workspace, TestResult};

const WRITE_ACTION: &str = "<action>
<tool>fs.write</tool>
<path>out.txt</path>
<content>hello</content>
</action>";

#[test]
fn owner_step_uses_supplied_task_budget() -> TestResult<()> {
    let opened = step(
        runtime_state()?,
        StepInput::Owner {
            content: "large task".to_string(),
            tokens: 4,
            graph: None,
            turn_budget: 9,
        },
    );
    assert!(matches!(
        opened.state.task,
        TaskState::Open { turns_remaining: 9 }
    ));
    Ok(())
}

#[test]
fn daemon_owner_delivery_uses_runtime_task_budget() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "large task", "owner-send", "101")?;
    let workspace = temp_workspace("task-budget")?;
    let server = serve_responses(vec![completion(WRITE_ACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace, 5)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;
    assert!(matches!(
        daemon.state.task,
        TaskState::Open { turns_remaining: 4 }
    ));
    Ok(())
}

#[test]
fn startup_summary_uses_configured_task_budget() -> TestResult<()> {
    let state = startup_state_with_budget(prefix()?, Some("resume task".to_string()), 11);
    assert!(matches!(
        state.task,
        TaskState::Open {
            turns_remaining: 11
        }
    ));
    Ok(())
}

fn daemon(base_url: &str, workspace: &Path, turn_budget: u16) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    )
    .with_task_turn_budget(turn_budget);
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
