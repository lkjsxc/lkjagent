pub mod benchmark;
pub mod doc_catalog;
pub mod doc_common;
pub mod doc_links;
pub mod doc_special;
pub mod doc_topology;
pub mod docs;
pub mod facts;
pub mod lines;
pub mod model;
pub mod runner;
pub mod smoke;
pub mod structure;
pub mod style;

use std::path::Path;

use docs::check_docs;
use facts::collect_files;
use lines::check_lines;
use lkjagent_benchmark::check_corpus;
use runner::run_quiet_test;
use style::check_style;

pub fn run(args: &[String], root: &Path) -> i32 {
    match parse_gate(args) {
        Ok(Gate::CheckDocs) => run_static_gate(root, "check-docs", check_docs),
        Ok(Gate::CheckLines) => run_static_gate(root, "check-lines", check_lines),
        Ok(Gate::CheckStyle) => run_static_gate(root, "check-style", check_style),
        Ok(Gate::HygieneCheck) => run_hygiene(root),
        Ok(Gate::QuietTest) => run_command_gate(root, "test"),
        Ok(Gate::QuietVerify) => run_verify(root),
        Ok(Gate::Benchmark(rest)) => benchmark::run(&rest, root),
        Ok(Gate::Smoke(rest)) => smoke::run(&rest, root),
        Ok(Gate::Structure(rest)) => structure::run(&rest, root),
        Err(lines) => {
            print_failure(&lines);
            2
        }
    }
}

fn run_static_gate(
    root: &Path,
    name: &'static str,
    check: fn(&[model::RepoFile]) -> Vec<model::Violation>,
) -> i32 {
    match collect_files(root) {
        Ok(files) => report_static(name, check(&files)),
        Err(message) => {
            print_failure(&[
                format!("{name} failed"),
                "exit status: 1".to_string(),
                message,
            ]);
            1
        }
    }
}

fn run_command_gate(root: &Path, ok_name: &'static str) -> i32 {
    match run_quiet_test(root) {
        Ok(()) => {
            println!("ok {ok_name}");
            0
        }
        Err(lines) => {
            print_failure(&lines);
            1
        }
    }
}

fn run_hygiene(root: &Path) -> i32 {
    let files = match collect_files(root) {
        Ok(files) => files,
        Err(message) => {
            print_failure(&[
                "hygiene-check failed".to_string(),
                "exit status: 1".to_string(),
                message,
            ]);
            return 1;
        }
    };
    let mut violations = check_lines(&files);
    violations.extend(check_style(&files));
    report_static("hygiene-check", violations)
}

fn run_verify(root: &Path) -> i32 {
    let files = match collect_files(root) {
        Ok(files) => files,
        Err(message) => {
            print_failure(&[
                "quiet verify failed".to_string(),
                "exit status: 1".to_string(),
                message,
            ]);
            return 1;
        }
    };
    for (name, check) in [
        (
            "check-docs",
            check_docs as fn(&[model::RepoFile]) -> Vec<model::Violation>,
        ),
        ("check-lines", check_lines),
        ("check-style", check_style),
    ] {
        let violations = check(&files);
        if !violations.is_empty() {
            return report_static(name, violations);
        }
    }
    if let Err(error) = check_corpus() {
        print_failure(&[
            "benchmark check-corpus failed".to_string(),
            "exit status: 1".to_string(),
            error.to_string(),
        ]);
        return 1;
    }
    run_command_gate(root, "verify")
}

fn report_static(name: &'static str, violations: Vec<model::Violation>) -> i32 {
    if violations.is_empty() {
        println!("ok {name}");
        return 0;
    }
    let mut lines = vec![format!("{name} failed"), "exit status: 1".to_string()];
    lines.extend(violations.into_iter().map(|violation| violation.message()));
    print_failure(&lines);
    1
}

fn print_failure(lines: &[String]) {
    for line in lines {
        eprintln!("{line}");
    }
}

enum Gate {
    CheckDocs,
    CheckLines,
    CheckStyle,
    QuietTest,
    QuietVerify,
    HygieneCheck,
    Benchmark(Vec<String>),
    Smoke(Vec<String>),
    Structure(Vec<String>),
}

fn parse_gate(args: &[String]) -> Result<Gate, Vec<String>> {
    match args {
        [one] if one == "check-docs" || one == "docs-check" => Ok(Gate::CheckDocs),
        [one] if one == "check-lines" => Ok(Gate::CheckLines),
        [one] if one == "check-style" => Ok(Gate::CheckStyle),
        [one] if one == "hygiene-check" => Ok(Gate::HygieneCheck),
        [first, second] if first == "quiet" && second == "test" => Ok(Gate::QuietTest),
        [first, second] if first == "quiet" && second == "verify" => Ok(Gate::QuietVerify),
        [first, rest @ ..] if first == "benchmark" => Ok(Gate::Benchmark(rest.to_vec())),
        [first, rest @ ..] if first == "smoke" => Ok(Gate::Smoke(rest.to_vec())),
        [first, rest @ ..] if first == "structure" => Ok(Gate::Structure(rest.to_vec())),
        _ => Err(vec![
            "xtask failed".to_string(),
            "exit status: 2".to_string(),
            "use: check-docs | docs-check | check-lines | check-style | hygiene-check | quiet test | quiet verify | benchmark ... | smoke ... | structure ..."
                .to_string(),
        ]),
    }
}
