mod support;

use std::fs;
use std::path::Path;

use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{queue, state};
use support::http::serve_responses;
use support::{store, temp_workspace, TestResult};

#[test]
fn aggregate_total_auto_scaffold_keeps_hundred_target() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "400", "0")?;
    queue::enqueue(
        &mut conn,
        "Create around 100 total for a structured story, including twenty outline files and ordered main files.",
        "owner-send",
        "401",
    )?;
    let workspace = temp_workspace("file-count-aggregate-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "401")?, DaemonTick::Idle);
    server.join()?;

    let root = workspace.join("structured-output");
    assert_eq!(file_count(&root)?, 100);
    assert!(root.join(support::design_path(20)).exists());
    assert!(!root.join(support::design_path(21)).exists());
    assert!(root.join(support::main_path(77)).exists());
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn japanese_artifact_auto_scaffold_routes_to_document_construction() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "500", "0")?;
    queue::enqueue(
        &mut conn,
        "合計百ファイルほどの大きな成果物を、二十個の設計メモと本文に分けて作ってください。",
        "owner-send",
        "501",
    )?;
    let workspace = temp_workspace("file-count-japanese-artifact-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "501")?, DaemonTick::Idle);
    server.join()?;

    let root = workspace.join("structured-output");
    assert_eq!(file_count(&root)?, 100);
    assert!(root.join(support::design_path(20)).exists());
    assert!(!root.join(support::design_path(21)).exists());
    assert!(root.join(support::main_path(77)).exists());
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn architecture_artifact_auto_scaffold_routes_to_document_construction() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "600", "0")?;
    queue::enqueue(
        &mut conn,
        "Create about 100 files total for a structured architecture artifact, including twenty outline files and ordered main files.",
        "owner-send",
        "601",
    )?;
    let workspace = temp_workspace("file-count-architecture-artifact-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "601")?, DaemonTick::Idle);
    server.join()?;

    let root = workspace.join("structured-output");
    assert_eq!(file_count(&root)?, 100);
    assert!(root.join(support::design_path(20)).exists());
    assert!(!root.join(support::design_path(21)).exists());
    assert!(root.join(support::main_path(77)).exists());
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn planning_notes_and_body_files_auto_scaffold() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "700", "0")?;
    queue::enqueue(
        &mut conn,
        "Create about 100 files total, split between twenty planning notes and body files.",
        "owner-send",
        "701",
    )?;
    let workspace = temp_workspace("file-count-body-files-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "701")?, DaemonTick::Idle);
    server.join()?;

    let root = workspace.join("structured-output");
    assert_eq!(file_count(&root)?, 100);
    assert!(root.join(support::design_path(20)).exists());
    assert!(!root.join(support::design_path(21)).exists());
    assert!(root.join(support::main_path(77)).exists());
    assert_eq!(state::get(&conn, "completion guard")?, None);
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    Ok(())
}

#[test]
fn japanese_design_and_main_files_auto_scaffold() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "800", "0")?;
    queue::enqueue(
        &mut conn,
        "合計百ファイルほどを、二十個の設計メモと本編ファイルに分けて作ってください。",
        "owner-send",
        "801",
    )?;
    let workspace = temp_workspace("file-count-japanese-main-auto")?;
    let server = serve_responses(vec![])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "801")?, DaemonTick::Idle);
    server.join()?;

    let root = workspace.join("structured-output");
    assert_eq!(file_count(&root)?, 100);
    assert!(root.join(support::design_path(20)).exists());
    assert!(!root.join(support::design_path(21)).exists());
    assert!(root.join(support::main_path(77)).exists());
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
