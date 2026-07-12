pub mod acceptance;
pub mod doc_common;
pub mod doc_links;
pub mod doc_special;
pub mod doc_topology;
mod docs_authority_contract;
pub mod docs_authority_gate;
pub mod evaluation_harness;
pub mod facts;
pub mod gate;
pub mod node_gate;
pub mod style;

pub mod model {
    pub use crate::facts::{RepoFile, Violation};
}

use std::path::Path;

use doc_special::check_docs;
use facts::collect_files;
use gate::{check_files, check_lines, parse_gate, run_quiet_test, Gate};
use style::check_style;

pub mod docs {
    pub use crate::doc_special::check_docs;
}

pub mod doc_reachability {
    pub use crate::doc_special::check_reachability;
}

pub mod lines {
    pub use crate::gate::check_lines;
}

pub mod structure {
    pub use crate::style::{audit_structure as audit, run_structure as run};
}

pub fn run(args: &[String], root: &Path) -> i32 {
    match parse_gate(args) {
        Ok(Gate::CheckDocs) => run_static_gate(root, "check-docs", check_docs),
        Ok(Gate::CheckLines) => run_static_gate(root, "check-lines", check_lines),
        Ok(Gate::CheckFiles) => run_static_gate(root, "check-files", check_files),
        Ok(Gate::CheckStyle) => run_static_gate(root, "check-style", check_style),
        Ok(Gate::HygieneCheck) => run_hygiene(root),
        Ok(Gate::QuietTest) => run_command_gate(root, "test"),
        Ok(Gate::QuietVerify) => run_verify(root),
        Ok(Gate::Node(identifier)) => run_node_gate(root, &identifier),
        Ok(Gate::Acceptance(rest)) => acceptance::run(&rest, root),
        Ok(Gate::Benchmark(rest)) => evaluation_harness::run_benchmark(&rest, root),
        Ok(Gate::Experiment(rest)) => evaluation_harness::run_experiment(&rest, root),
        Ok(Gate::Proof(_)) => evaluation_harness::reject_unbound_command("proof"),
        Ok(Gate::Smoke(rest)) => evaluation_harness::run_smoke(&rest, root),
        Ok(Gate::Structure(rest)) => structure::run(&rest, root),
        Ok(Gate::Evidence(rest)) => evaluation_harness::run_evidence(&rest, root),
        Ok(Gate::Campaign(rest)) => evaluation_harness::run_campaign(&rest, root),
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

fn run_node_gate(root: &Path, identifier: &str) -> i32 {
    match node_gate::check(root, identifier) {
        Ok(()) => {
            println!("ok gate {identifier}");
            0
        }
        Err(mut lines) => {
            lines.insert(0, "exit status: 1".to_string());
            lines.insert(0, format!("gate {identifier} failed"));
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
        ("check-files", check_files),
        ("check-style", check_style),
    ] {
        let violations = check(&files);
        if !violations.is_empty() {
            return report_static(name, violations);
        }
    }
    if let Err(error) = evaluation_harness::validate_corpus(root) {
        print_failure(&[
            "bench check-corpus failed".to_string(),
            "exit status: 1".to_string(),
            error,
        ]);
        return 1;
    }
    if let Err(error) = evaluation_harness::run_replay(root) {
        print_failure(&[
            "smoke replay failed".to_string(),
            "exit status: 1".to_string(),
            error,
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
