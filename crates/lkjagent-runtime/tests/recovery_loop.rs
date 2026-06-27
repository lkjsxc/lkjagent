mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, memory, queue, state};
use support::http::{completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

const READ_MISSING: &str = "<action>
<tool>fs.read</tool>
<path>missing.md</path>
</action>";
const WRITE_ONE: &str = "<action>
<tool>fs.write</tool>
<path>one.txt</path>
<content>one</content>
</action>";
const PLAN_ONE: &str = "<action>
<tool>graph.plan</tool>
<objective>recover first</objective>
<steps>write one.txt; read one.txt; record verification</steps>
<checks>fs.read one.txt confirms one</checks>
<paths>one.txt</paths>
<reason>recovery task still needs a graph plan before mutation</reason>
</action>";
const READ_ONE: &str = "<action>
<tool>fs.read</tool>
<path>one.txt</path>
</action>";
const EVIDENCE_ONE: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed one.txt content</summary>
<path>one.txt</path>
</action>";
const DONE_ONE: &str = "<action>
<tool>agent.done</tool>
<summary>first task recovered</summary>
</action>";
const WRITE_TWO: &str = "<action>
<tool>fs.write</tool>
<path>two.txt</path>
<content>final</content>
</action>";
const PLAN_TWO: &str = "<action>
<tool>graph.plan</tool>
<objective>recover second</objective>
<steps>write two.txt; read two.txt; record verification</steps>
<checks>fs.read two.txt confirms final</checks>
<paths>two.txt</paths>
<reason>second recovery task needs a graph plan before mutation</reason>
</action>";
const READ_TWO: &str = "<action>
<tool>fs.read</tool>
<path>two.txt</path>
</action>";
const EVIDENCE_TWO: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed final in two.txt</summary>
<path>two.txt</path>
</action>";
const DONE_TWO: &str = "<action>
<tool>agent.done</tool>
<summary>second task recovered</summary>
</action>";

#[test]
fn daemon_recovers_from_repeated_transient_errors_across_tasks() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "recover first", "owner-send", "101")?;
    let workspace = temp_workspace("recovery-loop")?;
    let server = serve_responses(vec![
        completion("not an act"),
        completion(READ_MISSING),
        completion(PLAN_ONE),
        completion(WRITE_ONE),
        completion(READ_ONE),
        completion(EVIDENCE_ONE),
        completion(DONE_ONE),
        completion(PLAN_TWO),
        completion(WRITE_TWO),
        completion(WRITE_TWO),
        completion(READ_TWO),
        completion(EVIDENCE_TWO),
        completion(DONE_TWO),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "107")?, DaemonTick::Done);
    queue::enqueue(&mut conn, "recover second", "owner-send", "108")?;
    assert_eq!(daemon.poll_once(&mut conn, "108")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "109")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "110")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "111")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "112")?, DaemonTick::Working);
    let final_tick = daemon.poll_once(&mut conn, "113")?;
    assert_eq!(
        final_tick,
        DaemonTick::Done,
        "log={}",
        daemon
            .state
            .context
            .log
            .iter()
            .map(|frame| frame.content.as_str())
            .collect::<Vec<_>>()
            .join("\n-- frame --\n")
    );
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
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
