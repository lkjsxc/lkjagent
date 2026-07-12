use std::path::Path;
use std::process::Command;

use crate::model::{RepoFile, Violation};

const DOC_FILE_LIMIT: usize = 100;
const PRODUCT_SOURCE_LIMIT: usize = 190;
const PRODUCT_CRATES: &[&str] = &[
    "lkjagent-core",
    "lkjagent-store",
    "lkjagent-llm",
    "lkjagent-effects",
    "lkjagent-app",
    "lkjagent-xtask",
];

pub enum Gate {
    Acceptance(Vec<String>),
    CheckDocs,
    CheckLines,
    CheckFiles,
    CheckStyle,
    QuietTest,
    QuietVerify,
    HygieneCheck,
    Node(String),
    Benchmark(Vec<String>),
    Experiment(Vec<String>),
    Proof(Vec<String>),
    Smoke(Vec<String>),
    Structure(Vec<String>),
    Evidence(Vec<String>),
    Campaign(Vec<String>),
}

pub fn check_lines(files: &[RepoFile]) -> Vec<Violation> {
    files
        .iter()
        .filter(|file| !file.path.starts_with("data/logs/") && !file.path.starts_with("tmp/"))
        .filter_map(|file| {
            let count = file.line_count();
            (count > 200).then(|| {
                Violation::new(
                    &file.path,
                    "line limit",
                    format!("has {count} lines, limit is 200; split by ownership"),
                )
            })
        })
        .collect()
}

pub fn check_files(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let docs = files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .count();
    if docs > DOC_FILE_LIMIT {
        violations.push(Violation::new(
            "docs",
            "file budget",
            format!("has {docs} markdown files, limit is {DOC_FILE_LIMIT}"),
        ));
    }
    let source = files
        .iter()
        .filter(|file| product_source(&file.path))
        .count();
    if source > PRODUCT_SOURCE_LIMIT {
        violations.push(Violation::new(
            "crates",
            "file budget",
            format!("has {source} product source files, limit is {PRODUCT_SOURCE_LIMIT}"),
        ));
    }
    violations
}

fn product_source(path: &str) -> bool {
    PRODUCT_CRATES
        .iter()
        .any(|name| path.starts_with(&format!("crates/{name}/src/")))
        && path.ends_with(".rs")
}

pub fn run_quiet_test(root: &Path) -> Result<(), Vec<String>> {
    run_step(root, "cargo fmt --check", &["fmt", "--check"])?;
    run_step(
        root,
        "cargo clippy --workspace --all-targets -- -D warnings",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_step(root, "cargo test --workspace", &["test", "--workspace"])?;
    Ok(())
}

fn run_step(root: &Path, label: &str, args: &[&str]) -> Result<(), Vec<String>> {
    let output = Command::new("cargo")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| {
            vec![
                format!("quiet test failed at {label}"),
                "exit status: 1".to_string(),
                format!("could not start command: {error}"),
            ]
        })?;
    if output.status.success() {
        return Ok(());
    }
    let status = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |code| code.to_string(),
    );
    let mut lines = vec![
        format!("quiet test failed at {label}"),
        format!("exit status: {status}"),
    ];
    lines.extend(tail(&String::from_utf8_lossy(&output.stdout)));
    lines.extend(tail(&String::from_utf8_lossy(&output.stderr)));
    Err(lines)
}

fn tail(text: &str) -> Vec<String> {
    let lines = text.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(20);
    lines
        .into_iter()
        .skip(start)
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

pub fn parse_gate(args: &[String]) -> Result<Gate, Vec<String>> {
    match args {
        [first, rest @ ..] if first == "acceptance" => Ok(Gate::Acceptance(rest.to_vec())),
        [one] if one == "check-docs" || one == "docs-check" => Ok(Gate::CheckDocs),
        [one] if one == "check-lines" => Ok(Gate::CheckLines),
        [one] if one == "check-files" => Ok(Gate::CheckFiles),
        [one] if one == "check-style" => Ok(Gate::CheckStyle),
        [one] if one == "hygiene-check" => Ok(Gate::HygieneCheck),
        [first, second] if first == "quiet" && second == "test" => Ok(Gate::QuietTest),
        [first, second] if first == "quiet" && second == "verify" => Ok(Gate::QuietVerify),
        [first, node] if first == "gate" => Ok(Gate::Node(node.clone())),
        [first, rest @ ..] if first == "benchmark" || first == "bench" => {
            Ok(Gate::Benchmark(rest.to_vec()))
        }
        [first, rest @ ..] if first == "experiment" => Ok(Gate::Experiment(rest.to_vec())),
        [first, rest @ ..] if first == "proof" => Ok(Gate::Proof(rest.to_vec())),
        [first, rest @ ..] if first == "smoke" => Ok(Gate::Smoke(rest.to_vec())),
        [first, rest @ ..] if first == "structure" => Ok(Gate::Structure(rest.to_vec())),
        [first, rest @ ..] if first == "evidence" => Ok(Gate::Evidence(rest.to_vec())),
        [first, rest @ ..] if first == "campaign" => Ok(Gate::Campaign(rest.to_vec())),
        _ => Err(vec![
            "xtask failed".to_string(),
            "exit status: 2".to_string(),
            "use: acceptance verify ... | check-docs | check-lines | check-files | check-style | hygiene-check | quiet test | quiet verify | gate NODE | bench ... | experiment ... | proof ... | smoke ... | structure ... | evidence check --campaign baseline [--source FULL_COMMIT] | campaign probe-endpoint|run ...".to_string(),
        ]),
    }
}
