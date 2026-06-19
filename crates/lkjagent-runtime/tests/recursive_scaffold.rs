mod support;

use std::fs;
use std::path::Path;

use lkjagent_context::model::FrameKind;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{memory, queue, state};
use lkjagent_tools::structure::verify_recursive_tree;
use lkjagent_tools::structure_network::verify_knowledge_network;
use support::http::{completion, serve_responses};
use support::{seed_skill_path, store, temp_workspace, TestResult};

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>recursive docs scaffold complete</summary>
</act>";

#[test]
fn recursive_docs_task_auto_scaffolds_before_done() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "build a highly recursive docs structure",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("recursive-scaffold")?;
    let server = serve_responses(vec![completion(DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    assert!(daemon
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.kind == FrameKind::SkillBody));
    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("profile=generic") && frame.content.contains("verification=ok")
    }));
    verify_recursive_tree(&workspace)?;

    server.join()?;
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert!(memory::find(&conn, "recursive docs scaffold complete", 5)?
        .iter()
        .any(|row| row.kind == "task-summary"));
    assert!(workspace.join("docs/api/v1/users/README.md").exists());
    assert_no_unindexed_directory(&workspace.join("docs"))?;
    Ok(())
}

#[test]
fn encyclopedia_task_auto_scaffolds_knowledge_network_before_done() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(&mut conn, "百科事典を作ってください。", "owner-send", "101")?;
    let workspace = temp_workspace("knowledge-scaffold")?;
    let server = serve_responses(vec![completion(DONE)])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    assert!(daemon.state.context.log.iter().any(|frame| {
        frame.content.contains("knowledge nucleus")
            && frame.content.contains("growth=incremental")
            && frame.content.contains("verification=ok")
    }));
    verify_knowledge_network(&workspace)?;

    server.join()?;
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert!(workspace.join("docs/maps/concept-network.md").exists());
    assert!(workspace.join("docs/execution/expansion-queue.md").exists());
    assert!(workspace.join("docs/execution/rebalance-plan.md").exists());
    assert!(workspace.join("docs/reference/ontology.md").exists());
    assert!(!workspace.join("docs/timelines").exists());
    assert!(!workspace.join("docs/questions").exists());
    assert!(markdown_count(&workspace.join("docs"))? <= 24);
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
    Ok(ResidentDaemon::new(support::runtime_state()?, runtime))
}

fn assert_no_unindexed_directory(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            assert!(path.join("README.md").exists(), "missing {:?}", path);
            assert_no_unindexed_directory(&path)?;
        }
    }
    Ok(())
}

fn markdown_count(path: &Path) -> TestResult<usize> {
    let mut count: usize = 0;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(markdown_count(&child)?);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
