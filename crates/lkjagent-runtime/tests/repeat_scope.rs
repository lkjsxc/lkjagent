mod support;

use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, queue};
use support::http::{completion, serve_responses};
use support::{store, temp_workspace, TestResult};

const WRITE: &str = "<act>
<tool>fs.write</tool>
<path>done.txt</path>
<content>done</content>
</act>";

const PLAN: &str = "<act>
<tool>graph.plan</tool>
<objective>write done file</objective>
<steps>write done.txt; read done.txt; record verification</steps>
<checks>fs.read done.txt confirms content</checks>
<paths>done.txt</paths>
<reason>owner mutation needs graph plan</reason>
</act>";

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>wrote done.txt</summary>
</act>";

const READ: &str = "<act>
<tool>fs.read</tool>
<path>done.txt</path>
</act>";

const EVIDENCE: &str = "<act>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed done.txt content</summary>
<path>done.txt</path>
</act>";

const ASK: &str = "<act>
<tool>agent.ask</tool>
<question>maintenance paused</question>
</act>";

#[test]
fn repeat_tracking_resets_between_maintenance_cycles() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "write done file", "owner-send", "101")?;
    let workspace = temp_workspace("repeat-scope")?;
    let server = serve_responses(vec![
        completion(PLAN),
        completion(WRITE),
        completion(READ),
        completion(EVIDENCE),
        completion(DONE),
        completion(ASK),
        completion(ASK),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Done);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Idle);
    assert_eq!(daemon.poll_once(&mut conn, "166")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "167")?, DaemonTick::Done);
    assert_eq!(daemon.poll_once(&mut conn, "168")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "169")?, DaemonTick::Done);
    server.join()?;

    let repeat_notices = events::read_events(&conn)?
        .iter()
        .filter(|event| event.content.contains("repeat action refused"))
        .count();
    assert_eq!(repeat_notices, 0);
    Ok(())
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(support::runtime_state()?, runtime))
}
