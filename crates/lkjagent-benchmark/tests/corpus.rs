use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_benchmark::fixture::materialize_fixture;
use lkjagent_benchmark::{check_corpus, judge_task, list_tasks};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn tiny_corpus_is_valid_and_large_enough() -> TestResult {
    check_corpus()?;
    assert!(list_tasks().len() >= 6);
    Ok(())
}

#[test]
fn judges_accept_good_fixtures_and_reject_bad_fixtures() -> TestResult {
    let root = temp_root("fixtures")?;
    for task in list_tasks() {
        for fixture in task.good {
            let workspace = root.join(task.id).join(fixture.name);
            materialize_fixture(fixture, &workspace)?;
            let outcome = judge_task(task, &workspace)?;
            assert!(outcome.passed, "{} {}", task.id, fixture.name);
        }
        for fixture in task.bad {
            let workspace = root.join(task.id).join(fixture.name);
            materialize_fixture(fixture, &workspace)?;
            let outcome = judge_task(task, &workspace)?;
            assert!(!outcome.passed, "{} {}", task.id, fixture.name);
        }
    }
    fs::remove_dir_all(root)?;
    Ok(())
}

fn temp_root(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-benchmark-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
