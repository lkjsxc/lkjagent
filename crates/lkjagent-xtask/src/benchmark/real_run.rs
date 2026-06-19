use std::fs;
use std::path::Path;
use std::time::Instant;

use lkjagent_benchmark::fixture::materialize_starter;
use lkjagent_benchmark::metrics::metrics_from_status_and_log;
use lkjagent_benchmark::model::BenchmarkTask;
use lkjagent_benchmark::report::ReportEntry;
use lkjagent_benchmark::runner::endpoint_config;
use lkjagent_benchmark::{judge_task, task_by_id, tasks_in_suite};

use super::args::RunArgs;
use super::docker;
use super::meta::{absolute, git_state, join_reason, run_id, sanitize, timestamp, write_reports};
use super::print_failure;
use super::wait::execute_task;

pub fn run(root: &Path, args: &RunArgs) -> i32 {
    let dotenv = fs::read_to_string(root.join(".env")).ok();
    let endpoint = match endpoint_config(|key| std::env::var(key).ok(), dotenv.as_deref()) {
        Ok(endpoint) => endpoint,
        Err(message) => {
            print_failure(&[
                "benchmark run failed".to_string(),
                "exit status: 2".to_string(),
                message,
            ]);
            return 2;
        }
    };
    let tasks = match selected_tasks(args) {
        Ok(tasks) => tasks,
        Err(message) => {
            print_failure(&[
                "benchmark run failed".to_string(),
                "exit status: 2".to_string(),
                message,
            ]);
            return 2;
        }
    };
    let data_root = absolute(root, &args.data);
    let run_id = run_id();
    let run_dir = data_root.join("runs").join(&run_id);
    let git_state = git_state(root);
    let mut entries = Vec::new();
    for task in tasks {
        entries.push(run_one(
            root, &run_dir, &run_id, &git_state, &endpoint, task,
        ));
    }
    if let Err(error) = write_reports(&run_dir, &entries) {
        print_failure(&[
            "benchmark run failed".to_string(),
            "exit status: 1".to_string(),
            error,
        ]);
        return 1;
    }
    let earned: u16 = entries.iter().map(|entry| entry.points_earned).sum();
    let possible: u16 = entries.iter().map(|entry| entry.points_possible).sum();
    println!(
        "benchmark report={}\nscore={earned}/{possible}",
        run_dir.join("summary.md").display()
    );
    if args.min_points.is_some_and(|minimum| earned < minimum) {
        1
    } else {
        0
    }
}

fn selected_tasks(args: &RunArgs) -> Result<Vec<&'static BenchmarkTask>, String> {
    if let Some(task_id) = &args.task {
        return task_by_id(task_id)
            .map(|task| vec![task])
            .map_err(|error| error.to_string());
    }
    let tasks = tasks_in_suite(&args.suite);
    if tasks.is_empty() {
        Err(format!("unknown or empty benchmark suite: {}", args.suite))
    } else {
        Ok(tasks)
    }
}

fn run_one(
    root: &Path,
    run_dir: &Path,
    run_id: &str,
    git_state: &str,
    endpoint: &lkjagent_benchmark::runner::EndpointConfig,
    task: &BenchmarkTask,
) -> ReportEntry {
    let task_dir = run_dir.join("tasks").join(task.id);
    let data_dir = task_dir.join("data");
    let workspace = data_dir.join("workspace");
    let transcript_path = task_dir.join("transcript.txt");
    let _ = fs::create_dir_all(&workspace);
    let _ = materialize_starter(task.starter_files, &workspace);
    let project = format!("lkjagent-bench-{}-{}", sanitize(run_id), task.id);
    let start = Instant::now();
    let wait = execute_task(root, &project, &data_dir, task);
    docker::down(root, &project, &data_dir);
    let _ = fs::create_dir_all(&task_dir);
    let _ = fs::write(&transcript_path, &wait.log);
    let outcome = judge_task(task, &workspace).unwrap_or_else(|error| {
        lkjagent_benchmark::model::JudgeOutcome::fail(task.points, error.to_string())
    });
    let reason = if outcome.passed {
        outcome.reason
    } else {
        join_reason(&outcome.reason, &wait.reason)
    };
    ReportEntry {
        run_id: run_id.to_string(),
        timestamp: timestamp(),
        git_state: git_state.to_string(),
        model_label: endpoint.model_label.clone(),
        endpoint_host: endpoint.endpoint_host.clone(),
        suite: task.suite.to_string(),
        task_id: task.id.to_string(),
        family: task.family.as_str().to_string(),
        difficulty: task.difficulty.as_str().to_string(),
        passed: outcome.passed,
        points_earned: outcome.points_earned,
        points_possible: outcome.points_possible,
        judge_reason: reason,
        elapsed_ms: start.elapsed().as_millis(),
        end_state: wait.end_state,
        metrics: metrics_from_status_and_log(&wait.status, &wait.log),
        workspace_path: workspace.to_string_lossy().to_string(),
        transcript_path: transcript_path.to_string_lossy().to_string(),
    }
}
