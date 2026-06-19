mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{queue, state};
use support::http::{completion, serve_responses};
use support::{store, temp_workspace, TestResult};

const BATCH_WRITE: &str = "<act>
<tool>shell.run</tool>
<command>
set -eu
rm -rf deliverable
mkdir -p deliverable/docs deliverable/main
printf '# Deliverable\n' > deliverable/README.md
printf '# Doc\n' > deliverable/docs/plan.md
printf '# Main 1\n' > deliverable/main/part-001.md
printf '# Main 2\n' > deliverable/main/part-002.md
printf '# Main 3\n' > deliverable/main/part-003.md
printf 'files='
find deliverable -type f | wc -l
</command>
</act>";

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>created a README-indexed five-file deliverable</summary>
</act>";

#[test]
fn counted_documentation_task_closes_after_batch_shell_write() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "Create docs and main deliverable with exactly 5 files total.",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("file-count-daemon")?;
    let server = serve_responses(vec![completion(BATCH_WRITE), completion(DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(file_count(&workspace.join("deliverable"))?, 5);
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
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

fn file_count(path: &Path) -> TestResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(file_count(&child)?);
        } else {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
