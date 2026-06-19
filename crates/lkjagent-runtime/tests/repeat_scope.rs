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

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>wrote done.txt</summary>
</act>";

const VERIFY: &str = "<act>
<tool>shell.run</tool>
<command>test -f done.txt</command>
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
        completion(WRITE),
        completion(VERIFY),
        completion(DONE),
        completion(ASK),
        completion(ASK),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Done);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Idle);
    assert_eq!(daemon.poll_once(&mut conn, "164")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "165")?, DaemonTick::Done);
    assert_eq!(daemon.poll_once(&mut conn, "166")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "167")?, DaemonTick::Done);
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
