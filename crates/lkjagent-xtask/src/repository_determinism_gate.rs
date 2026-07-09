use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::doc_special::check_docs;
use crate::facts::collect_files;
use crate::gate::{check_files, check_lines};
use crate::style::check_style;

const CLEAN_GATE: &str =
    "sh tmp/lkjagent-evidence-first-rebuild-20260710/13-scripts/clean_checkout_gate.sh .";
const REQUIRED: &[&str] = &[
    "Cargo.lock",
    ".gitignore",
    ".dockerignore",
    "Dockerfile",
    "docker-compose.yml",
    ".github/workflows/verify.yml",
    "data/lkjagent.json",
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let files = collect_files(root).map_err(|error| vec![error])?;
    let mut failures = check_docs(&files)
        .into_iter()
        .chain(check_lines(&files))
        .chain(check_files(&files))
        .chain(check_style(&files))
        .map(|violation| violation.message())
        .collect::<Vec<_>>();
    failures.extend(check_inputs(root));
    failures.extend(check_configuration(root));
    failures.extend(check_docker(root));
    failures.extend(check_workflow(root));
    if failures.is_empty() {
        failures.extend(run_suites(root));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

pub fn check_inputs(root: &Path) -> Vec<String> {
    let mut failures = Vec::new();
    for relative in REQUIRED {
        let path = root.join(relative);
        if !path.is_file() {
            failures.push(format!("required repository input is missing: {relative}"));
        } else if root.join(".git").exists() && !tracked(root, relative) {
            failures.push(format!("repository input is not tracked: {relative}"));
        }
    }
    let ignore = read(root.join(".gitignore"), &mut failures);
    if ignore.lines().any(|line| line.trim() == "/Cargo.lock") {
        failures.push("Cargo.lock remains ignored".to_string());
    }
    failures
}

pub fn check_configuration(root: &Path) -> Vec<String> {
    let mut failures = Vec::new();
    let registry = read(
        root.join("docs/product/configuration-registry.md"),
        &mut failures,
    );
    let keys = registry
        .lines()
        .filter_map(|line| line.strip_prefix("| "))
        .filter_map(|line| line.split_once(" |"))
        .map(|(key, _)| key)
        .filter(|key| key.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_'))
        .collect::<BTreeSet<_>>();
    let text = read(root.join("data/lkjagent.json"), &mut failures);
    let parsed = serde_json::from_str::<Value>(&text);
    match parsed {
        Ok(Value::Object(values)) => {
            let actual = values.keys().map(String::as_str).collect::<BTreeSet<_>>();
            if actual != keys {
                failures.push("tracked configuration keys differ from the registry".to_string());
            }
            if values
                .values()
                .any(|value| !matches!(value, Value::String(_) | Value::Number(_) | Value::Bool(_)))
            {
                failures.push("tracked configuration contains a non-scalar value".to_string());
            }
        }
        Ok(_) => failures.push("tracked configuration root is not an object".to_string()),
        Err(error) => failures.push(format!("tracked configuration is invalid JSON: {error}")),
    }
    if let Err(error) = lkjagent_app::config::load_client(&root.join("data")) {
        failures.push(format!(
            "tracked configuration violates the typed registry: {error}"
        ));
    }
    failures
}

pub fn check_docker(root: &Path) -> Vec<String> {
    let mut failures = Vec::new();
    let dockerfile = read(root.join("Dockerfile"), &mut failures);
    for line in dockerfile.lines().map(str::trim) {
        if !line.starts_with("COPY ") || line.starts_with("COPY --from=") {
            continue;
        }
        let fields = line[5..].split_whitespace().collect::<Vec<_>>();
        for source in fields.iter().take(fields.len().saturating_sub(1)) {
            let source = source.trim_matches(['[', ']', ',', '"']);
            if source.is_empty() {
                continue;
            }
            if !root.join(source).exists() {
                failures.push(format!("Docker COPY source is missing: {source}"));
            } else if root.join(".git").exists() && !tracked(root, source) {
                failures.push(format!("Docker COPY source is not tracked: {source}"));
            }
        }
    }
    if !dockerfile.contains("cargo build --locked") {
        failures.push("Docker release build is not locked".to_string());
    }
    let compose = read(root.join("docker-compose.yml"), &mut failures);
    for line in compose
        .lines()
        .filter(|line| line.contains("cargo") && line.contains("run"))
    {
        if !line.contains("--locked") {
            failures.push(format!("Compose cargo run is not locked: {}", line.trim()));
        }
    }
    failures
}

pub fn check_workflow(root: &Path) -> Vec<String> {
    let mut failures = Vec::new();
    let workflow = read(root.join(".github/workflows/verify.yml"), &mut failures);
    if !workflow.contains("actions/checkout@") || !workflow.contains(CLEAN_GATE) {
        failures.push("public workflow does not directly run the anchored clean gate".to_string());
    }
    failures
}

fn run_suites(root: &Path) -> Vec<String> {
    let mut failures = Vec::new();
    for (package, test) in [
        ("lkjagent-app", "configuration_contract"),
        ("lkjagent-xtask", "repository_determinism_gate"),
    ] {
        let args = ["test", "--locked", "-p", package, "--test", test];
        let output = Command::new("cargo").args(args).current_dir(root).output();
        match output {
            Ok(output) if output.status.success() => {}
            Ok(output) => failures.push(format!(
                "focused suite failed: cargo {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr)
                    .lines()
                    .last()
                    .unwrap_or("no stderr")
            )),
            Err(error) => failures.push(format!("focused suite could not start: {error}")),
        }
    }
    failures
}

fn tracked(root: &Path, relative: &str) -> bool {
    Command::new("git")
        .args(["ls-files", "--error-unmatch", "--", relative])
        .current_dir(root)
        .output()
        .is_ok_and(|output| output.status.success())
}

fn read(path: PathBuf, failures: &mut Vec<String>) -> String {
    match fs::read_to_string(&path) {
        Ok(text) if !text.is_empty() => text,
        Ok(_) => {
            failures.push(format!("repository input is empty: {}", path.display()));
            String::new()
        }
        Err(error) => {
            failures.push(format!("could not read {}: {error}", path.display()));
            String::new()
        }
    }
}
