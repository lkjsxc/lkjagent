use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use lkjagent_benchmark::model::BenchmarkTask;

use super::docker;

pub struct WaitResult {
    pub end_state: String,
    pub status: String,
    pub log: String,
    pub reason: String,
}

pub fn execute_task(
    root: &Path,
    project: &str,
    data_dir: &Path,
    task: &BenchmarkTask,
) -> WaitResult {
    let up = docker::strings(&["up", "-d", "--build", "agent"]);
    if let Err(error) = docker::compose(root, project, data_dir, &up) {
        return failed("harness_error", error);
    }
    if let Err(error) = send(root, project, data_dir, task.prompt) {
        return failed("harness_error", error);
    }
    if let Some(follow_up) = task.follow_up {
        if let Err(error) = send(root, project, data_dir, follow_up) {
            return failed("harness_error", error);
        }
    }
    wait_for_end(
        root,
        project,
        data_dir,
        Duration::from_secs(task.timeout_seconds),
    )
}

fn send(root: &Path, project: &str, data_dir: &Path, text: &str) -> Result<String, String> {
    let args = vec![
        "run".to_string(),
        "--rm".to_string(),
        "agent".to_string(),
        "send".to_string(),
        text.to_string(),
    ];
    docker::compose(root, project, data_dir, &args)
}

fn wait_for_end(root: &Path, project: &str, data_dir: &Path, timeout: Duration) -> WaitResult {
    let start = Instant::now();
    let mut last_status = String::new();
    let mut last_log = String::new();
    while start.elapsed() < timeout {
        last_status = status(root, project, data_dir).unwrap_or_default();
        last_log = log(root, project, data_dir).unwrap_or_default();
        if last_log.contains("task-summary memory_id") {
            return ok("agent.done", last_status, last_log);
        }
        if last_status.contains("daemon_state=waiting") {
            return ok("agent.ask", last_status, last_log);
        }
        if daemon_error(&last_status).is_some() {
            return ok("endpoint_error", last_status, last_log);
        }
        if last_log.contains("turn budget exhausted") {
            return ok("budget_exhausted", last_status, last_log);
        }
        thread::sleep(Duration::from_secs(1));
    }
    WaitResult {
        end_state: "timeout".to_string(),
        status: last_status,
        log: last_log,
        reason: "benchmark task timed out".to_string(),
    }
}

fn status(root: &Path, project: &str, data_dir: &Path) -> Result<String, String> {
    docker::compose(
        root,
        project,
        data_dir,
        &docker::strings(&["run", "--rm", "agent", "status"]),
    )
}

fn log(root: &Path, project: &str, data_dir: &Path) -> Result<String, String> {
    docker::compose(
        root,
        project,
        data_dir,
        &docker::strings(&["run", "--rm", "agent", "log", "--full"]),
    )
}

fn ok(end_state: &str, status: String, log: String) -> WaitResult {
    WaitResult {
        end_state: end_state.to_string(),
        status,
        log,
        reason: String::new(),
    }
}

fn failed(end_state: &str, reason: String) -> WaitResult {
    WaitResult {
        end_state: end_state.to_string(),
        status: String::new(),
        log: String::new(),
        reason,
    }
}

fn daemon_error(status: &str) -> Option<String> {
    status
        .lines()
        .find_map(|line| line.strip_prefix("daemon_error="))
        .filter(|value| *value != "none")
        .map(str::to_string)
}
