use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

pub(crate) fn pairs(path: PathBuf, failures: &mut Vec<String>) -> BTreeMap<String, String> {
    read(path, failures)
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

pub(crate) fn expect(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
    failures: &mut Vec<String>,
) {
    if values.get(key).map(String::as_str) != Some(expected) {
        failures.push(format!("expected {key}={expected}"));
    }
}

pub(crate) fn read(path: PathBuf, failures: &mut Vec<String>) -> String {
    match fs::read_to_string(&path) {
        Ok(text) if !text.is_empty() => text,
        Ok(_) => {
            failures.push(format!("evidence file is empty: {}", path.display()));
            String::new()
        }
        Err(error) => {
            failures.push(format!("could not read {}: {error}", path.display()));
            String::new()
        }
    }
}
