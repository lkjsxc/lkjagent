mod daemon_loop_actions;
mod support;

use std::fs;
use std::net::TcpListener;
use std::path::Path;

use daemon_loop_actions::*;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_store::{events, memory, queue, state, token_usage};
use support::http::{completion, length_completion, serve_responses};
use support::{runtime_state, store, temp_workspace, TestResult};

#[test]
fn daemon_delivers_queue_writes_file_and_records_done() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "write the file", "owner-send", "101")?;
    let workspace = temp_workspace("daemon-write")?;
    let server = serve_responses(vec![
        completion(PLAN_WRITE_ACTION),
        completion(WRITE_ACTION),
        completion(READ_OUT_ACTION),
        completion(EVIDENCE_WRITE_ACTION),
        completion(DONE_WRITE_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    let model_log_path = workspace.join("current-model-run.md");
    daemon.runtime.model_log_path = Some(model_log_path.clone());

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert!(fs::read_to_string(&model_log_path)?.contains("Model Run Log"));
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(fs::read_to_string(workspace.join("out.txt"))?, "hello");
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    assert_eq!(state::get(&conn, "open task")?, Some("none".to_string()));
    assert!(memory::find(&conn, "wrote", 5)?
        .iter()
        .any(|row| row.kind == "task-summary"));
    let log = events::read_events(&conn)?;
    assert!(log.iter().any(|event| event.content.contains("fs.write")));
    assert!(log
        .iter()
        .any(|event| event.content.contains("task-summary")));
    let usage = token_usage::latest(&conn)?.ok_or("endpoint token usage")?;
    assert_eq!(usage.input_tokens, Some(5));
    assert_eq!(usage.output_tokens, Some(3));
    assert_eq!(usage.source, "endpoint");
    Ok(())
}

#[test]
fn daemon_waits_on_ask_and_resumes_from_next_send() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "start", "owner-send", "101")?;
    let workspace = temp_workspace("daemon-ask")?;
    let server = serve_responses(vec![
        completion(OWNER_QUESTION_NOTE_ACTION),
        completion(ASK_ACTION),
        completion(PLAN_RESUME_ACTION),
        completion(WORKSPACE_ACTION),
        completion(EVIDENCE_GENERIC_ACTION),
        completion(DONE_GENERIC_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Waiting);
    assert_eq!(
        state::get(&conn, "daemon question")?,
        Some("Need detail?".to_string())
    );
    queue::enqueue(&mut conn, "guidance", "owner-send", "103")?;
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "106")?, DaemonTick::Done);
    server.join()?;

    let log = events::read_events(&conn)?;
    let owners = log.iter().filter(|event| event.kind == "owner").count();
    assert_eq!(owners, 2);
    assert!(log
        .iter()
        .any(|event| event.kind == "owner" && event.content == "guidance"));
    assert!(!daemon
        .state
        .context
        .log
        .iter()
        .any(|frame| frame.content.contains("<kind>delivery</kind>")));
    assert_eq!(state::get(&conn, "daemon state")?, Some("idle".to_string()));
    Ok(())
}

#[test]
fn daemon_records_endpoint_error_without_losing_delivered_queue() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "fail endpoint", "owner-send", "101")?;
    let workspace = temp_workspace("daemon-error")?;
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let base_url = format!("http://{}", listener.local_addr()?);
    drop(listener);
    let mut daemon = daemon(&base_url, &workspace)?;

    assert_eq!(
        daemon.poll_once(&mut conn, "101")?,
        DaemonTick::EndpointError
    );
    assert_eq!(
        state::get(&conn, "daemon state")?,
        Some("error".to_string())
    );
    assert!(events::read_events(&conn)?
        .iter()
        .any(|event| event.kind == "error"));
    assert!(queue::list(&conn)?
        .first()
        .is_some_and(|row| row.status == "delivered"));
    Ok(())
}

#[test]
fn daemon_recovers_from_max_token_completion_without_retry_loop() -> TestResult<()> {
    let mut conn = store()?;
    take_lock(&conn)?;
    queue::enqueue(&mut conn, "write many files", "owner-send", "101")?;
    let workspace = temp_workspace("daemon-oversize")?;
    let server = serve_responses(vec![
        length_completion("<act>\n<tool>shell.run</tool>\n<command>"),
        completion(PLAN_RESUME_ACTION),
        completion(WORKSPACE_ACTION),
        completion(EVIDENCE_GENERIC_ACTION),
        completion(DONE_GENERIC_ACTION),
    ])?;
    let mut daemon = daemon(&server.base_url, &workspace)?;

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    assert_eq!(daemon.endpoint_attempt, 0);
    assert_eq!(daemon.poll_once(&mut conn, "102")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "103")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "104")?, DaemonTick::Working);
    assert_eq!(daemon.poll_once(&mut conn, "105")?, DaemonTick::Done);
    server.join()?;

    let log = events::read_events(&conn)?;
    assert!(log
        .iter()
        .any(|event| event.content.contains("completion hit max tokens")));
    assert!(log
        .iter()
        .any(|event| event.content.contains("next act must stay bounded")));
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
