use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use super::hash;

const IDS: [&str; 3] = [
    "daily-life-recall",
    "multi-project-development",
    "long-artifact-recovery",
];
const FILES: [&str; 5] = [
    "scenario.tsv",
    "matters.tsv",
    "owner-schedule.tsv",
    "seed-manifest.tsv",
    "checks.tsv",
];

pub struct Scenario {
    pub id: String,
    pub fingerprint: String,
    pub required_check_count: usize,
}

pub fn check(root: &Path, faults: &BTreeSet<String>) -> Result<Vec<Scenario>, Vec<String>> {
    let mut scenarios = Vec::new();
    let mut failures = Vec::new();
    for id in IDS {
        match check_one(&root.join("evaluation/scenarios").join(id), id, faults) {
            Ok(scenario) => scenarios.push(scenario),
            Err(mut errors) => failures.append(&mut errors),
        }
    }
    if failures.is_empty() {
        Ok(scenarios)
    } else {
        Err(failures)
    }
}

fn check_one(path: &Path, id: &str, faults: &BTreeSet<String>) -> Result<Scenario, Vec<String>> {
    let mut failures = Vec::new();
    let fields = pairs(&path.join("scenario.tsv"), &mut failures);
    for (key, expected) in [
        ("scenario_id", id),
        ("minimum_duration_seconds", "840"),
        ("minimum_owner_turns", "5"),
        ("minimum_owner_span_seconds", "840"),
        ("minimum_decision_span_seconds", "600"),
        ("minimum_decisions", "8"),
        ("minimum_useful_decisions", "5"),
        ("minimum_progress_decisions", "3"),
        ("allowed_terminal_states", "completed"),
        ("source_binding", "required-run-source-commit"),
    ] {
        if fields.get(key).map(String::as_str) != Some(expected) {
            failures.push(format!("scenario {id} requires {key}={expected}"));
        }
    }
    for fault in csv(fields.get("required_fault_ids")) {
        if !faults.contains(fault) {
            failures.push(format!("scenario {id} requires unknown fault {fault}"));
        }
    }
    if csv(fields.get("negative_predicates")).len() < 4 {
        failures.push(format!(
            "scenario {id} has fewer than four negative predicates"
        ));
    }
    check_matters(path, id, &mut failures);
    check_schedule(path, id, &mut failures);
    super::check_scenario_seed(path, id, &mut failures);
    let checks = check_checks(path, id, &mut failures);
    let fingerprint = bundle(path, &mut failures);
    if failures.is_empty() {
        Ok(Scenario {
            id: id.into(),
            fingerprint,
            required_check_count: checks,
        })
    } else {
        Err(failures)
    }
}

fn check_matters(path: &Path, id: &str, failures: &mut Vec<String>) {
    let text = read(&path.join("matters.tsv"), failures);
    let rows = text.lines().skip(1).collect::<Vec<_>>();
    if rows.len() < 3
        || rows.iter().any(|row| {
            let fields = row.split('\t').collect::<Vec<_>>();
            fields.len() != 2 || fields[0].is_empty() || fields[1] != "completed"
        })
    {
        failures.push(format!("scenario {id} matter expectations are incomplete"));
    }
}

fn check_schedule(path: &Path, id: &str, failures: &mut Vec<String>) {
    let text = read(&path.join("owner-schedule.tsv"), failures);
    let mut offsets = Vec::new();
    for row in text.lines().skip(1) {
        let fields = row.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            failures.push(format!("scenario {id} owner schedule row is malformed"));
            continue;
        }
        offsets.push(fields[0]);
        if fields[2] != hash::bytes(fields[3].as_bytes()) {
            failures.push(format!("scenario {id} owner text fingerprint differs"));
        }
    }
    if offsets != ["0", "180", "420", "660", "840"] {
        failures.push(format!("scenario {id} owner schedule offsets differ"));
    }
}

fn check_checks(path: &Path, id: &str, failures: &mut Vec<String>) -> usize {
    let text = read(&path.join("checks.tsv"), failures);
    let checks = text
        .lines()
        .skip(1)
        .filter_map(|row| row.split_once('\t'))
        .filter(|(check, checker)| !check.is_empty() && !checker.is_empty())
        .map(|(check, _)| check)
        .collect::<BTreeSet<_>>();
    for required in required_checks(id) {
        if !checks.contains(required) {
            failures.push(format!("scenario {id} lacks required check {required}"));
        }
    }
    checks.len()
}

fn required_checks(id: &str) -> &'static [&'static str] {
    match id {
        "daily-life-recall" => &[
            "journal-path-date",
            "journal-body-semantic",
            "multi-intent-decomposition",
            "old-record-recall",
            "todo-roundtrip",
        ],
        "multi-project-development" => &[
            "context-project-isolation",
            "project-separation",
            "source-edit-verified",
            "workspace-visible",
        ],
        _ => &[
            "artifact-units-complete",
            "all-files-verified",
            "output-limit-recovered",
            "strategy-changed",
        ],
    }
}

fn bundle(path: &Path, failures: &mut Vec<String>) -> String {
    let mut bytes = Vec::new();
    for name in FILES {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&fs::read(path.join(name)).unwrap_or_else(|error| {
            failures.push(format!("could not read scenario bundle {name}: {error}"));
            Vec::new()
        }));
        bytes.push(0);
    }
    hash::bytes(&bytes)
}

fn pairs(path: &Path, failures: &mut Vec<String>) -> BTreeMap<String, String> {
    read(path, failures)
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once('\t'))
        .map(|(key, value)| (key.into(), value.into()))
        .collect()
}

fn read(path: &Path, failures: &mut Vec<String>) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        failures.push(format!("could not read {}: {error}", path.display()));
        String::new()
    })
}

fn csv(value: Option<&String>) -> Vec<&str> {
    value.map_or_else(Vec::new, |value| {
        value.split(',').filter(|item| !item.is_empty()).collect()
    })
}
