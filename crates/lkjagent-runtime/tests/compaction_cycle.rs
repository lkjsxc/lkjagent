mod support;

use std::path::Path;

use lkjagent_context::model::{Frame, FrameKind};
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::TaskState;
use lkjagent_store::{events, memory, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, seed_skill_path, store, temp_workspace, TestResult};

const SAVE_TASK_SUMMARY: &str = "<act>
<tool>memory.save</tool>
<kind>task-summary</kind>
<title>compaction resume</title>
<tags>compaction</tags>
<content>resume after compaction</content>
</act>";
const WRITE_DURING_COMPACTION: &str = "<act>
<tool>fs.write</tool>
<path>should-not-exist.txt</path>
<content>bad write</content>
</act>";

#[test]
fn orange_pressure_distills_before_rebuild() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    state::set(&conn, "open task", "continue the active task")?;
    let workspace = temp_workspace("daemon-orange-compaction")?;
    let server = serve_responses(vec![completion(SAVE_TASK_SUMMARY)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 7 };
    push_orange_pressure(&mut daemon);

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(daemon.state.compaction.is_some());
    assert_eq!(daemon.endpoint_attempt, 0);
    assert!(daemon
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("distill before compaction")));

    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.compaction.is_none());
    assert!(daemon.state.context.used_tokens() <= daemon.runtime.budget.post_compaction_target);
    assert!(memory::find(&conn, "resume", 5)?
        .iter()
        .any(|row| row.kind == "task-summary"));
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "compaction"
            && event.content.contains("memory_ids=[")
            && event.content.contains("context_window=24576")
    }));
    Ok(())
}

#[test]
fn compaction_rejects_non_memory_save_actions() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    state::set(&conn, "open task", "continue the active task")?;
    let workspace = temp_workspace("daemon-compaction-tool-gate")?;
    let server = serve_responses(vec![completion(WRITE_DURING_COMPACTION)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 7 };
    push_orange_pressure(&mut daemon);

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    server.join()?;

    assert!(!workspace.join("should-not-exist.txt").exists());
    assert_eq!(
        daemon
            .state
            .compaction
            .as_ref()
            .map(|cycle| cycle.turns_remaining),
        Some(3)
    );
    assert!(events::read_events(&conn)?
        .iter()
        .any(|event| { event.kind == "notice" && event.content.contains("memory.save actions") }));
    Ok(())
}

fn push_orange_pressure(daemon: &mut ResidentDaemon) {
    let used = daemon.state.context.used_tokens();
    let tokens = daemon
        .runtime
        .budget
        .soft_trigger
        .saturating_sub(used)
        .saturating_add(32);
    daemon.state.context.log.push(Frame::new(
        FrameKind::Observation,
        "large prior output",
        tokens,
    ));
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        seed_skill_path(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}

fn take_lock(conn: &rusqlite::Connection) -> TestResult<()> {
    take_daemon_lock(conn, "test", "100", "0")?;
    Ok(())
}
