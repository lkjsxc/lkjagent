pub mod args;
pub mod docker;
pub mod meta;
pub mod real_run;
pub mod wait;

use std::fs;
use std::path::Path;

use args::BenchmarkCommand;
use lkjagent_benchmark::{check_corpus, judge_task, list_tasks, report, task_by_id};

pub fn run(args: &[String], root: &Path) -> i32 {
    match args::parse(args) {
        Ok(BenchmarkCommand::List) => list(),
        Ok(BenchmarkCommand::CheckCorpus) => check(),
        Ok(BenchmarkCommand::Judge { task, workspace }) => judge(&task, &workspace),
        Ok(BenchmarkCommand::Run(args)) => real_run::run(root, &args),
        Ok(BenchmarkCommand::Compare {
            old_report,
            new_report,
        }) => compare(&old_report, &new_report),
        Err(lines) => {
            print_failure(&lines);
            2
        }
    }
}

fn list() -> i32 {
    for task in list_tasks() {
        println!(
            "{}\tfamily={}\tdifficulty={}\ttags={}",
            task.id,
            task.family.as_str(),
            task.difficulty.as_str(),
            task.tags.join(",")
        );
    }
    0
}

fn check() -> i32 {
    match check_corpus() {
        Ok(()) => {
            println!("ok benchmark-corpus");
            0
        }
        Err(error) => {
            print_failure(&[
                "benchmark check-corpus failed".to_string(),
                "exit status: 1".to_string(),
                error.to_string(),
            ]);
            1
        }
    }
}

fn judge(task_id: &str, workspace: &Path) -> i32 {
    let task = match task_by_id(task_id) {
        Ok(task) => task,
        Err(error) => {
            print_failure(&[
                "benchmark judge failed".to_string(),
                "exit status: 2".to_string(),
                error.to_string(),
            ]);
            return 2;
        }
    };
    match judge_task(task, workspace) {
        Ok(outcome) if outcome.passed => {
            println!(
                "pass task={} points={}/{}",
                task.id, outcome.points_earned, outcome.points_possible
            );
            0
        }
        Ok(outcome) => {
            println!("fail task={} reason={}", task.id, outcome.reason);
            1
        }
        Err(error) => {
            print_failure(&[
                "benchmark judge failed".to_string(),
                "exit status: 1".to_string(),
                error.to_string(),
            ]);
            1
        }
    }
}

fn compare(old_report: &Path, new_report: &Path) -> i32 {
    match (
        fs::read_to_string(old_report),
        fs::read_to_string(new_report),
    ) {
        (Ok(old), Ok(new)) => {
            print!("{}", report::compare_tsv(&old, &new));
            0
        }
        (old, new) => {
            print_failure(&[
                "benchmark compare failed".to_string(),
                "exit status: 1".to_string(),
                format!("old={:?} new={:?}", old.err(), new.err()),
            ]);
            1
        }
    }
}

pub fn print_failure(lines: &[String]) {
    for line in lines {
        eprintln!("{line}");
    }
}
