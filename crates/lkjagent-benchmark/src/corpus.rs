use std::fs;
use std::path::PathBuf;

use crate::error::{BenchError, BenchResult};
use crate::fixture::{materialize_fixture, validate_relative_path};
use crate::judge_task;
use crate::model::BenchmarkTask;
use crate::tasks;

pub fn list_tasks() -> &'static [BenchmarkTask] {
    tasks::TINY
}

pub fn task_by_id(id: &str) -> BenchResult<&'static BenchmarkTask> {
    list_tasks()
        .iter()
        .find(|task| task.id == id)
        .ok_or_else(|| BenchError::UnknownTask(format!("unknown benchmark task: {id}")))
}

pub fn tasks_in_suite(suite: &str) -> Vec<&'static BenchmarkTask> {
    list_tasks()
        .iter()
        .filter(|task| task.suite == suite)
        .collect()
}

pub fn check_corpus() -> BenchResult<()> {
    for task in list_tasks() {
        validate_task(task)?;
        check_fixtures(task)?;
    }
    Ok(())
}

fn validate_task(task: &BenchmarkTask) -> BenchResult<()> {
    if !valid_id(task.id) {
        return Err(BenchError::InvalidTask(format!(
            "invalid task id: {}",
            task.id
        )));
    }
    if task.suite != "tiny" {
        return Err(BenchError::InvalidTask(format!(
            "task {} is outside tiny suite",
            task.id
        )));
    }
    if task.prompt.trim().is_empty() || task.tags.is_empty() {
        return Err(BenchError::InvalidTask(format!(
            "task {} needs prompt and tags",
            task.id
        )));
    }
    if task.good.is_empty() || task.bad.len() < 2 || task.points == 0 {
        return Err(BenchError::InvalidTask(format!(
            "task {} needs one good fixture, two bad fixtures, and points",
            task.id
        )));
    }
    for file in task.starter_files {
        validate_relative_path(file.path)?;
    }
    Ok(())
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        && id.contains('-')
}

fn check_fixtures(task: &BenchmarkTask) -> BenchResult<()> {
    let root = temp_root(task.id)?;
    for fixture in task.good {
        let workspace = root.join(fixture.name);
        materialize_fixture(fixture, &workspace)?;
        let outcome = judge_task(task, &workspace)?;
        if !outcome.passed {
            return Err(BenchError::Judge(format!(
                "good fixture failed: {} {} {}",
                task.id, fixture.name, outcome.reason
            )));
        }
    }
    for fixture in task.bad {
        let workspace = root.join(fixture.name);
        materialize_fixture(fixture, &workspace)?;
        let outcome = judge_task(task, &workspace)?;
        if outcome.passed {
            return Err(BenchError::Judge(format!(
                "bad fixture passed: {} {}",
                task.id, fixture.name
            )));
        }
    }
    if root.exists() {
        fs::remove_dir_all(root)?;
    }
    Ok(())
}

fn temp_root(task_id: &str) -> BenchResult<PathBuf> {
    let mut root = std::env::temp_dir();
    root.push(format!(
        "lkjagent-benchmark-{}-{task_id}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(&root)?;
    Ok(root)
}
