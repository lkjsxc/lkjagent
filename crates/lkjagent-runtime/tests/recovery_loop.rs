mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, memory, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, seed_skill_path, store, temp_workspace, TestResult};

const READ_MISSING: &str = "<act>
<tool>fs.read</tool>
<path>missing.md</path>
</act>";
const WRITE_ONE: &str = "<act>
<tool>fs.write</tool>
<path>one.txt</path>
<content>one</content>
</act>";
const DONE_ONE: &str = "<act>
<tool>agent.done</tool>
<summary>first task recovered</summary>
</act>";
const WRITE_TWO: &str = "<act>
<tool>fs.write</tool>
<path>two.txt</path>
<content>draft</content>
</act>";
const WRITE_TWO_FIXED: &str = "<act>
<tool>fs.write</tool>
<path>two.txt</path>
<content>final</content>
</act>";
const DONE_TWO: &str = "<act>
<tool>agent.done</tool>
<summary>second task recovered</summary>
</act>";

#[test]
fn daemon_recovers_from_repeated_transient_errors_across_tasks() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "recover first", "owner-send", "101")?;
    let workspace = temp_workspace("recovery-loop")?;
    let server = serve_responses(vec![
        completion("not an act"),
        completion(READ_MISSING),
        completion(WRITE_ONE),
        completion(DONE_ONE),
        completion(WRITE_TWO),
        completion(WRITE_TWO),
        completion(WRITE_TWO_FIXED),
        completion(DONE_TWO),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Done);
    queue::enqueue(&mut conn, "recover second", "owner-send", "105")?;
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "107")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "108")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(fs::read_to_string(workspace.join("one.txt"))?, "one");
    assert_eq!(fs::read_to_string(workspace.join("two.txt"))?, "final");
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(state::get(&conn, "daemon error")?, None);
    assert!(memory::find(&conn, "recovered", 5)?.len() >= 2);
    let log = events::read_events(&conn)?;
    assert!(log
        .iter()
        .any(|event| event.content.contains("parse fault")));
    assert!(log.iter().any(|event| {
        event.content.contains("tool error recorded") || event.content.contains("missing file")
    }));
    assert!(log
        .iter()
        .any(|event| event.content.contains("repeated action was refused")));
    Ok(())
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180),
        workspace.to_path_buf(),
        seed_skill_path(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
