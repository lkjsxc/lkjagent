mod support;

use std::{fs, path::Path};

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const MAINT_DONE: &str = "<action>
<tool>agent.done</tool>
<summary>maintenance cycle checked current state</summary>
</action>";
const WRITE_ACTION: &str = "<action>
<tool>fs.write</tool>
<path>owner.txt</path>
<content>owner wins</content>
</action>";
const MAINT_ASK: &str = "<action>
<tool>agent.ask</tool>
<question>Should maintenance wait?</question>
</action>";
const PLAN_ACTION: &str = "<action>
<tool>graph.plan</tool>
<objective>write owner file</objective>
<steps>Write owner file; read owner file; record verification evidence</steps>
<checks>fs.read owner.txt confirms content</checks>
<paths>owner.txt</paths>
<reason>owner request needs planned mutation</reason>
</action>";
const DONE_ACTION: &str = "<action>
<tool>agent.done</tool>
<summary>owner task complete</summary>
</action>";
const READ_ACTION: &str = "<action>
<tool>fs.read</tool>
<path>owner.txt</path>
</action>";
const EVIDENCE_ACTION: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed owner wins in owner.txt</summary>
<path>owner.txt</path>
</action>";

#[test]
fn idle_daemon_runs_maintenance_and_delays_after_empty_cycle() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("auto-maintenance")?;
    let server = serve_responses(vec![completion(MAINT_DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("working".to_string())
    );
    assert!(state::get(&conn, "open task")?.is_some_and(|task| task.starts_with("maintenance:")));
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Idle);
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(daemon.poll_once(&mut conn, "162")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert!(events::read_events(&conn)?.iter().any(|event| {
        event.kind == "notice" && event.content.contains("maintenance cycle opened")
    }));
    Ok(())
}

#[test]
fn maintenance_ask_closes_cycle_without_waiting_for_owner() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("auto-maintenance-ask")?;
    let server = serve_responses(vec![completion(MAINT_ASK)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;

    assert!(daemon.state.maintenance.is_none());
    assert!(!daemon.dispatch_state.control.question_outstanding);
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(state::get(&conn, "daemon question")?, None);
    Ok(())
}

#[test]
fn maintenance_labeled_queue_ask_does_not_wait_for_owner() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(
        &mut conn,
        "maintenance: resume distillation after compaction",
        "maintenance-resume",
        "101",
    )?;
    let workspace = temp_workspace("auto-maintenance-labeled-ask")?;
    let server = serve_responses(vec![completion(MAINT_ASK)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    server.join()?;

    assert!(daemon.state.maintenance.is_none());
    assert!(!daemon.dispatch_state.control.question_outstanding);
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    assert_eq!(state::get(&conn, "daemon question")?, None);
    Ok(())
}

#[test]
fn owner_queue_preempts_idle_maintenance_at_turn_boundary() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    let workspace = temp_workspace("auto-maintenance-preempt")?;
    let server = serve_responses(vec![
        completion(PLAN_ACTION),
        completion(WRITE_ACTION),
        completion(READ_ACTION),
        completion(EVIDENCE_ACTION),
        completion(DONE_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    queue::enqueue(&mut conn, "write owner file", "owner-send", "102")?;
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(
        fs::read_to_string(workspace.join("owner.txt"))?,
        "owner wins"
    );

    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Done);
    server.join()?;
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn closed_owner_task_delays_maintenance_until_cooldown_passes() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "write owner file", "owner-send", "101")?;
    let workspace = temp_workspace("auto-maintenance-after-task")?;
    let server = serve_responses(vec![
        completion(PLAN_ACTION),
        completion(WRITE_ACTION),
        completion(READ_ACTION),
        completion(EVIDENCE_ACTION),
        completion(DONE_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Done);
    server.join()?;
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));

    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Idle);
    assert!(daemon.state.maintenance.is_none());
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));

    assert_eq!(daemon.poll_once(&mut conn, "166")?, DaemonTick::Working);
    assert!(daemon.state.maintenance.is_some());
    assert!(state::get(&conn, "open task")?.is_some_and(|task| task.starts_with("maintenance:")));
    Ok(())
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

fn take_lock(conn: &rusqlite::Connection) -> TestResult<()> {
    take_daemon_lock(conn, "test", "100", "0")?;
    Ok(())
}
