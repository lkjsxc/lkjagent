mod support;

use std::fs;
use std::path::Path;

use lkjagent_context::model::FrameKind;
use lkjagent_runtime::daemon::{
    client_config, restore_completion_guard, take_daemon_lock, DaemonTick, ResidentDaemon,
    ResidentRuntime,
};
use lkjagent_store::{memory, queue, state};
use lkjagent_tools::control::CompletionGuard;
use lkjagent_tools::dispatch::DispatchState;
use support::http::{completion, serve_responses};
use support::{seed_skill_path, store, temp_workspace, TestResult};

const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>recursive structure complete</summary>
</act>";

#[test]
fn recursive_structure_task_refuses_one_file_done_then_finishes_tree() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    queue::enqueue(
        &mut conn,
        "高度に再帰的に構造化された workspace を作ってください",
        "owner-send",
        "101",
    )?;
    let workspace = temp_workspace("recursive-guard")?;
    let responses = scripted_responses();
    let server = serve_responses(responses)?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(daemon
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.kind == FrameKind::SkillBody
            && frame.content.contains("# Skill: Recursive Structure")));
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(
        state::get(&conn, "open task")?,
        Some("高度に再帰的に構造化された workspace を作ってください".to_string())
    );
    assert_eq!(
        state::get(&conn, "completion guard")?,
        Some("recursive-structure".to_string())
    );
    let mut restored = DispatchState::default();
    restore_completion_guard(&conn, &mut restored)?;
    assert_eq!(restored.control.guard, CompletionGuard::RecursiveStructure);
    assert!(memory::find(&conn, "recursive structure complete", 5)?.is_empty());

    for stamp in 103..115 {
        assert_eq!(
            daemon.poll_once(&mut conn, &stamp.to_string())?,
            DaemonTick::Working
        );
    }
    assert_eq!(daemon.poll_once(&mut conn, "115")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert!(memory::find(&conn, "recursive structure complete", 5)?
        .iter()
        .any(|row| row.kind == "task-summary"));
    assert!(workspace
        .join("docs/product/contracts/domain/model.md")
        .exists());
    assert_no_unindexed_directory(&workspace.join("docs"))?;
    Ok(())
}

fn scripted_responses() -> Vec<String> {
    let mut responses = vec![
        completion(&write_action("docs/README.md", DOCS_README)),
        completion(DONE),
    ];
    for (path, content) in TREE_FILES {
        responses.push(completion(&write_action(path, content)));
    }
    responses.push(completion(DONE));
    responses
}

fn write_action(path: &str, content: &str) -> String {
    format!(
        "<act>\n<tool>fs.write</tool>\n<path>{path}</path>\n<content>\n{content}</content>\n</act>"
    )
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        seed_skill_path(),
        "100",
    );
    Ok(ResidentDaemon::new(support::runtime_state()?, runtime))
}

fn assert_no_unindexed_directory(root: &Path) -> TestResult<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            assert!(path.join("README.md").exists(), "missing {:?}", path);
            assert_no_unindexed_directory(&path)?;
        }
    }
    Ok(())
}

const DOCS_README: &str = "# Docs\n\n## Purpose\n\nRoot docs index.\n\n## Table of Contents\n\n- [product/](product/README.md): product contracts.\n- [architecture/](architecture/README.md): architecture contracts.\n";

const PRODUCT_README: &str = "# Product\n\n## Purpose\n\nProduct index.\n\n## Table of Contents\n\n- [contracts/](contracts/README.md): product contracts.\n- [surfaces.md](surfaces.md): product surfaces.\n";

const CONTRACTS_README: &str = "# Contracts\n\n## Purpose\n\nContract index.\n\n## Table of Contents\n\n- [domain/](domain/README.md): domain contracts.\n- [acceptance.md](acceptance.md): acceptance rules.\n";

const DOMAIN_README: &str = "# Domain\n\n## Purpose\n\nDomain index.\n\n## Table of Contents\n\n- [model.md](model.md): domain model.\n- [glossary.md](glossary.md): domain terms.\n";

const ARCHITECTURE_README: &str = "# Architecture\n\n## Purpose\n\nArchitecture index.\n\n## Table of Contents\n\n- [runtime/](runtime/README.md): runtime contracts.\n";

const RUNTIME_README: &str = "# Runtime\n\n## Purpose\n\nRuntime index.\n\n## Table of Contents\n\n- [loop.md](loop.md): loop contract.\n- [tools.md](tools.md): tool contract.\n";

const LEAF: &str = "# Leaf\n\n## Purpose\n\nLeaf contract.\n\n## Status\n\nimplemented.\n";

const TREE_FILES: &[(&str, &str)] = &[
    ("docs/product/README.md", PRODUCT_README),
    ("docs/product/contracts/README.md", CONTRACTS_README),
    ("docs/product/contracts/domain/README.md", DOMAIN_README),
    ("docs/architecture/README.md", ARCHITECTURE_README),
    ("docs/architecture/runtime/README.md", RUNTIME_README),
    ("docs/product/surfaces.md", LEAF),
    ("docs/product/contracts/acceptance.md", LEAF),
    ("docs/product/contracts/domain/model.md", LEAF),
    ("docs/product/contracts/domain/glossary.md", LEAF),
    ("docs/architecture/runtime/loop.md", LEAF),
    ("docs/architecture/runtime/tools.md", LEAF),
    ("docs/architecture/runtime/operations.md", LEAF),
];
