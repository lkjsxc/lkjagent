use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::hash;

const FIXTURES: [&str; 5] = [
    "idle-as-complete.tsv",
    "blocked-as-complete.tsv",
    "skipped-command.tsv",
    "zero-test-filter.tsv",
    "generated-placeholder.tsv",
];

#[derive(Clone)]
pub struct Facts {
    values: BTreeMap<String, String>,
}

impl Facts {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
        let values = text
            .lines()
            .filter_map(|line| line.split_once('\t'))
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        Ok(Self { values })
    }

    pub fn computed(scenario_fingerprint: &str, raw_artifact_count: usize) -> Self {
        let rows = [
            ("fixture_id", "computed-valid-run".to_string()),
            ("actual_terminal", "completed".into()),
            ("claimed_terminal", "completed".into()),
            ("duration_seconds", "900".into()),
            ("decision_span_seconds", "700".into()),
            ("decision_count", "8".into()),
            ("useful_decision_count", "5".into()),
            ("progress_decision_count", "3".into()),
            ("required_check_count", "5".into()),
            ("passed_check_count", "5".into()),
            ("command_exit", "0".into()),
            ("test_count", "4".into()),
            ("skipped", "false".into()),
            ("generated_placeholder", "false".into()),
            ("raw_artifact_count", raw_artifact_count.to_string()),
            ("snapshot_method", "sqlite-online-backup".into()),
            (
                "source_commit",
                "1111111111111111111111111111111111111111".into(),
            ),
            ("scenario_fingerprint", scenario_fingerprint.into()),
        ];
        Self {
            values: rows
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        }
    }

    pub fn expected_error(&self) -> Option<&str> {
        self.values.get("expected_error").map(String::as_str)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.into(), value.into());
    }
}

pub fn validate(facts: &Facts) -> Vec<String> {
    let mut failures = Vec::new();
    if value(facts, "actual_terminal") != value(facts, "claimed_terminal") {
        failures.push("claimed terminal contradicts raw terminal".into());
    }
    if boolean(facts, "skipped") != Some(false) {
        failures.push("required command was skipped".into());
    }
    if boolean(facts, "generated_placeholder") != Some(false) {
        failures.push("generated placeholder is not evidence".into());
    }
    if number(facts, "command_exit") != Some(0) {
        failures.push("required command did not exit zero".into());
    }
    if number(facts, "test_count").is_none_or(|count| count == 0) {
        failures.push("test count must be positive".into());
    }
    for (key, minimum, message) in [
        ("duration_seconds", 840, "duration floor not met"),
        ("decision_span_seconds", 600, "decision span floor not met"),
        ("decision_count", 8, "decision floor not met"),
        ("useful_decision_count", 5, "useful decision floor not met"),
        (
            "progress_decision_count",
            3,
            "progress decision floor not met",
        ),
        ("raw_artifact_count", 4, "raw artifact floor not met"),
    ] {
        if number(facts, key).is_none_or(|count| count < minimum) {
            failures.push(message.into());
        }
    }
    let required = number(facts, "required_check_count");
    if required.is_none_or(|count| count == 0) || required != number(facts, "passed_check_count") {
        failures.push("required checks do not match passed checks".into());
    }
    if value(facts, "snapshot_method") != Some("sqlite-online-backup") {
        failures.push("snapshot was not produced with SQLite Online Backup".into());
    }
    if !valid_commit(value(facts, "source_commit").unwrap_or("")) {
        failures.push("source commit binding is malformed".into());
    }
    if !value(facts, "scenario_fingerprint").is_some_and(hash::valid) {
        failures.push("scenario fingerprint is malformed".into());
    }
    failures
}

pub fn check_fixtures(root: &Path, fingerprint: &str, raw_count: usize) -> Vec<String> {
    let mut failures = Vec::new();
    let directory = root.join("evaluation/false-positive-fixtures");
    for name in FIXTURES {
        match Facts::from_path(&directory.join(name)) {
            Ok(facts) => {
                let expected = facts.expected_error().unwrap_or("missing expected error");
                let found = validate(&facts);
                if found != [expected] {
                    failures.push(format!(
                        "fixture {name} rejected as {found:?}, expected {expected}"
                    ));
                }
            }
            Err(error) => failures.push(format!("fixture {name} unreadable: {error}")),
        }
    }
    let valid = Facts::computed(fingerprint, raw_count);
    failures.extend(
        validate(&valid)
            .into_iter()
            .map(|error| format!("computed valid fixture failed: {error}")),
    );
    failures
}

pub fn fixture_errors(path: &Path) -> Result<Vec<String>, String> {
    Facts::from_path(path).map(|facts| validate(&facts))
}

fn value<'a>(facts: &'a Facts, key: &str) -> Option<&'a str> {
    facts.values.get(key).map(String::as_str)
}

fn number(facts: &Facts, key: &str) -> Option<u64> {
    value(facts, key)?.parse().ok()
}

fn boolean(facts: &Facts, key: &str) -> Option<bool> {
    match value(facts, key) {
        Some("true") => Some(true),
        Some("false") => Some(false),
        _ => None,
    }
}

fn valid_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
