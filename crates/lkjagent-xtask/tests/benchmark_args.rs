use std::path::PathBuf;

use lkjagent_xtask::benchmark::args::{parse, BenchmarkCommand, RunArgs};

#[test]
fn parses_benchmark_judge_arguments() {
    let args = strings(&["judge", "--task", "crt-exact-001", "--workspace", "work"]);

    assert_eq!(
        parse(&args),
        Ok(BenchmarkCommand::Judge {
            task: "crt-exact-001".to_string(),
            workspace: PathBuf::from("work"),
        })
    );
}

#[test]
fn parses_benchmark_run_arguments() {
    let args = strings(&[
        "run",
        "--suite",
        "tiny",
        "--data",
        "data/benchmark",
        "--task",
        "crt-exact-001",
        "--min-points",
        "1",
    ]);

    assert_eq!(
        parse(&args),
        Ok(BenchmarkCommand::Run(RunArgs {
            suite: "tiny".to_string(),
            data: PathBuf::from("data/benchmark"),
            task: Some("crt-exact-001".to_string()),
            min_points: Some(1),
        }))
    );
}

#[test]
fn rejects_incomplete_benchmark_run_arguments() {
    let args = strings(&["run", "--suite", "tiny"]);

    assert!(parse(&args).is_err());
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
