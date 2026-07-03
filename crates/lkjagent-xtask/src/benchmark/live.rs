use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_app::daemon::run_until_idle;
use lkjagent_app::endpoint::LlmEndpoint;
use lkjagent_core::model::TaskState;
use rusqlite::Connection;

use super::corpus::Entry;

pub fn run(data_dir: &Path, suite: &str, entries: &[Entry]) -> Result<String, String> {
    fs::create_dir_all(data_dir).map_err(|error| format!("create data dir: {error}"))?;
    let config = data_dir.join("lkjagent.json");
    let mut results = Vec::new();
    for entry in entries {
        results.push(run_entry(data_dir, &config, entry)?);
    }
    Ok(report(suite, &results))
}

fn run_entry(data_dir: &Path, config: &Path, entry: &Entry) -> Result<EntryResult, String> {
    let entry_dir = data_dir.join("entries").join(safe_name(&entry.name));
    if entry_dir.exists() {
        fs::remove_dir_all(&entry_dir).map_err(|error| format!("reset {}: {error}", entry.name))?;
    }
    fs::create_dir_all(&entry_dir).map_err(|error| format!("create {}: {error}", entry.name))?;
    if config.exists() {
        fs::copy(config, entry_dir.join("lkjagent.json"))
            .map_err(|error| format!("copy endpoint config: {error}"))?;
    }
    enqueue(&entry_dir, entry)?;
    let mut endpoint = LlmEndpoint::new(&entry_dir);
    let mut final_state = TaskState::Open;
    let mut turns = 0usize;
    for _ in 0..80 {
        let snapshot = run_until_idle(&entry_dir, &mut endpoint, 1)?;
        final_state = snapshot.task.state;
        turns = snapshot.task.budget_used as usize;
        if final_state != TaskState::Open {
            break;
        }
    }
    let (passed, total) = check_counts(&entry_dir)?;
    Ok(EntryResult {
        name: entry.name.clone(),
        state: format!("{:?}", final_state).to_ascii_lowercase(),
        checks_passed: passed,
        checks_total: total,
        expected_checks: entry.checks.len(),
        turns,
        artifact_dir: entry_dir,
    })
}

fn enqueue(data_dir: &Path, entry: &Entry) -> Result<(), String> {
    let db = data_dir.join("lkjagent.sqlite3");
    let conn = Connection::open(db).map_err(|error| error.to_string())?;
    lkjagent_store::plan_schema::setup(&conn).map_err(|error| error.to_string())?;
    lkjagent_store::plan_access::enqueue(&conn, entry.objective.trim(), "bench")
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn check_counts(data_dir: &Path) -> Result<(usize, usize), String> {
    let db = data_dir.join("lkjagent.sqlite3");
    let conn = Connection::open(db).map_err(|error| error.to_string())?;
    let mut stmt = conn
        .prepare("SELECT passed FROM check_results")
        .map_err(|error| error.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, i64>(0))
        .map_err(|error| error.to_string())?;
    let mut passed = 0usize;
    let mut total = 0usize;
    for row in rows {
        total = total.saturating_add(1);
        if row.map_err(|error| error.to_string())? == 1 {
            passed = passed.saturating_add(1);
        }
    }
    Ok((passed, total))
}

fn report(suite: &str, results: &[EntryResult]) -> String {
    let mut lines = vec!["# Benchmark Report".to_string(), String::new()];
    lines.push(format!("suite: {suite}"));
    lines.push(format!("entries: {}", results.len()));
    let passed = results
        .iter()
        .filter(|result| result.state == "closed" && result.checks_passed == result.checks_total)
        .count();
    lines.push(format!("score: {passed}/{}", results.len()));
    lines.push(String::new());
    for result in results {
        lines.push(format!(
            "- {} state={} checks={}/{} expected={} turns={} artifact={}",
            result.name,
            result.state,
            result.checks_passed,
            result.checks_total,
            result.expected_checks,
            result.turns,
            result.artifact_dir.display()
        ));
    }
    lines.join("\n")
}

fn safe_name(name: &str) -> String {
    name.chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => ch,
            _ => '-',
        })
        .collect()
}

struct EntryResult {
    name: String,
    state: String,
    checks_passed: usize,
    checks_total: usize,
    expected_checks: usize,
    turns: usize,
    artifact_dir: PathBuf,
}
