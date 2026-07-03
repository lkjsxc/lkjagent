use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::classify::classify;
use lkjagent_core::model::{CheckSpec, StepKind};
use lkjagent_core::parse::parse_expected;
use serde_json::Value;

pub struct Entry {
    pub name: String,
    pub objective: String,
    pub checks: Vec<CheckSpec>,
}

pub fn validate_all(root: &Path) -> Result<Vec<Entry>, String> {
    let base = root.join("evaluation/corpus");
    let mut entries = Vec::new();
    for suite in read_dirs(&base)? {
        entries.extend(validate_suite_path(&suite)?);
    }
    if entries.is_empty() {
        return Err("benchmark corpus is empty".to_string());
    }
    Ok(entries)
}

pub fn validate_suite(root: &Path, suite: &str) -> Result<Vec<Entry>, String> {
    let path = root.join("evaluation/corpus").join(suite);
    let entries = validate_suite_path(&path)?;
    if entries.is_empty() {
        return Err(format!("benchmark suite {suite} is empty"));
    }
    Ok(entries)
}

fn validate_suite_path(path: &Path) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    for entry in read_dirs(path)? {
        entries.push(validate_entry(&entry)?);
    }
    Ok(entries)
}

fn validate_entry(path: &Path) -> Result<Entry, String> {
    let objective = read(path, "objective.txt")?;
    let template = read(path, "template.txt")?;
    let actual = format!("{:?}", classify(objective.trim())).to_ascii_lowercase();
    if normalize(template.trim()) != normalize(&actual) {
        return Err(format!("{}: template {template:?} != {actual}", name(path)));
    }
    let checks = parse_checks(&read(path, "checks.json")?)?;
    validate_fixtures(path)?;
    Ok(Entry {
        name: name(path),
        objective,
        checks,
    })
}

fn validate_fixtures(path: &Path) -> Result<(), String> {
    let text = read(&path.join("fixtures"), "exchanges.txt")?;
    for (index, line) in text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        let parts = line.splitn(3, '|').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!(
                "{} fixture {}: expected mode|kind|raw",
                name(path),
                index + 1
            ));
        }
        let kind = step_kind(parts[1])?;
        let raw = parts[2].replace("\\n", "\n");
        let parsed = parse_expected(kind, &raw);
        match (parts[0], parsed.is_ok()) {
            ("ok", true) | ("fault", false) => {}
            (mode, _) => {
                return Err(format!(
                    "{} fixture {}: unexpected parse result {mode}",
                    name(path),
                    index + 1
                ))
            }
        }
    }
    Ok(())
}

fn parse_checks(text: &str) -> Result<Vec<CheckSpec>, String> {
    let Value::Array(values) = serde_json::from_str::<Value>(text).map_err(|e| e.to_string())?
    else {
        return Err("checks.json must be an array".to_string());
    };
    values.iter().map(check).collect()
}

fn check(value: &Value) -> Result<CheckSpec, String> {
    let kind = string(value, "kind")?;
    match kind {
        "file_exists" => Ok(CheckSpec::FileExists {
            path: string(value, "path")?.to_string(),
        }),
        "file_count" => Ok(CheckSpec::FileCount {
            glob: string(value, "glob")?.to_string(),
            min: number(value, "min")?,
            max: value.get("max").and_then(Value::as_u64).map(|n| n as usize),
        }),
        "min_words_total" => Ok(CheckSpec::MinWordsTotal {
            glob: string(value, "glob")?.to_string(),
            n: number(value, "n")?,
        }),
        "readme_coverage" => Ok(CheckSpec::ReadmeCoverage {
            root: string(value, "root")?.to_string(),
        }),
        "links_resolve" => Ok(CheckSpec::LinksResolve {
            root: string(value, "root")?.to_string(),
        }),
        other => Err(format!("unknown check kind {other}")),
    }
}

fn step_kind(value: &str) -> Result<StepKind, String> {
    match value {
        "Plan" => Ok(StepKind::Plan),
        "Write" => Ok(StepKind::Write),
        "Respond" => Ok(StepKind::Respond),
        other => Err(format!("unknown step kind {other}")),
    }
}

fn read_dirs(path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = fs::read_dir(path)
        .map_err(|e| format!("read {}: {e}", path.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    dirs.sort();
    Ok(dirs)
}

fn read(path: &Path, file: &str) -> Result<String, String> {
    fs::read_to_string(path.join(file))
        .map_err(|e| format!("read {}/{}: {e}", path.display(), file))
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key}"))
}

fn number(value: &Value, key: &str) -> Result<usize, String> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .ok_or_else(|| format!("missing number {key}"))
}

fn normalize(value: &str) -> String {
    value.replace(['_', '-'], "").to_ascii_lowercase()
}

fn name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("entry")
        .to_string()
}
