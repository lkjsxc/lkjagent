mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::queue;
use support::http::{completion, length_completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn direct_manuscript_request_does_not_auto_scaffold_structured_output() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "Write one 700 to 900 word chapter at stories/the-bell-rings-twice/manuscript/chapter-01.md. Do not create structured-output.",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("story-manuscript-no-scaffold")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    let tick = daemon.poll_once(&mut conn, "101")?;
    server.join()?;

    assert_ne!(tick, DaemonTick::Idle);
    assert!(!workspace.join("structured-output").exists());
    Ok(())
}

#[test]
fn repeated_oversize_manuscript_write_waits_with_exact_path() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "200", "0")?;
    queue::enqueue(&mut conn, OWNER_TASK, "owner-send", "201")?;
    let workspace = temp_workspace("story-manuscript-oversize-handoff")?;
    let server = serve_responses(vec![
        completion(PLAN_ACTION),
        length_completion(BATCH_PREVIEW),
        length_completion(BATCH_PREVIEW),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "201")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "202")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "203")?, DaemonTick::Waiting);
    server.join()?;

    let question = lkjagent_store::state::get(&conn, "daemon question")?.unwrap_or_default();
    assert!(
        question.contains("remaining_path=stories/the-bell-rings-twice/manuscript/chapter-01.md")
    );
    assert!(!workspace.join("structured-output").exists());
    Ok(())
}

const OWNER_TASK: &str = "Write one 700 to 900 word finished high-school romance chapter at stories/the-bell-rings-twice/manuscript/chapter-01.md. Do not create structured-output.";

const PLAN_ACTION: &str = "<action>\n<tool>graph.plan</tool>\n<objective>Write the requested manuscript chapter.</objective>\n<steps>write exact chapter path</steps>\n<checks>chapter path exists</checks>\n<paths>stories/the-bell-rings-twice</paths>\n<reason>Graph planning requirement</reason>\n</action>";

const BATCH_PREVIEW: &str = "<action>\n<tool>fs.batch_write</tool>\n<files>\npath: stories/the-bell-rings-twice/manuscript/chapter-01.md\\ncontent:\n# Chapter One\n\nThe bell rang twice and the chapter kept going";

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
