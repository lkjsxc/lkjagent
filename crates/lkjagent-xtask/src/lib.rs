pub mod doc_common;
pub mod doc_special;
pub mod doc_topology;
pub mod docs;
pub mod facts;
pub mod lines;
pub mod model;
pub mod runner;
pub mod style;

use std::path::Path;

use docs::check_docs;
use facts::collect_files;
use lines::check_lines;
use runner::run_quiet_test;
use style::check_style;

pub fn run(args: &[String], root: &Path) -> i32 {
    match parse_gate(args) {
        Ok(Gate::CheckDocs) => run_static_gate(root, "check-docs", check_docs),
        Ok(Gate::CheckLines) => run_static_gate(root, "check-lines", check_lines),
        Ok(Gate::CheckStyle) => run_static_gate(root, "check-style", check_style),
        Ok(Gate::QuietTest) => run_command_gate(root, "test"),
        Ok(Gate::QuietVerify) => run_verify(root),
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
}

fn parse_gate(args: &[String]) -> Result<Gate, Vec<String>> {
    match args {
        [one] if one == "check-docs" => Ok(Gate::CheckDocs),
        [one] if one == "check-lines" => Ok(Gate::CheckLines),
        [one] if one == "check-style" => Ok(Gate::CheckStyle),
        [first, second] if first == "quiet" && second == "test" => Ok(Gate::QuietTest),
        [first, second] if first == "quiet" && second == "verify" => Ok(Gate::QuietVerify),
        _ => Err(vec![
            "xtask failed".to_string(),
            "exit status: 2".to_string(),
            "use: check-docs | check-lines | check-style | quiet test | quiet verify".to_string(),
        ]),
    }
}
