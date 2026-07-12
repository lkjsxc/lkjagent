mod evidence;
mod experiments;
mod git;
mod history;
mod markers;
mod plans;
mod secret;
mod source;
mod table;
mod workgraph;

use std::fs;
use std::path::{Path, PathBuf};

struct Args {
    source: String,
    evidence: PathBuf,
    allow_incomplete: bool,
}

pub fn run(args: &[String], root: &Path) -> i32 {
    let parsed = match parse_args(args) {
        Ok(value) => value,
        Err(errors) => {
            print_errors(&errors);
            return 2;
        }
    };
    let report = verify(root, &parsed.source, &parsed.evidence);
    if report.errors.is_empty() && report.missing.is_empty() {
        println!("ok acceptance verify");
        return 0;
    }
    let mode = if parsed.allow_incomplete {
        "acceptance verify incomplete"
    } else {
        "acceptance verify failed"
    };
    eprintln!("{mode}");
    print_errors(&report.errors);
    for id in &report.missing {
        eprintln!("missing derivation: {id}");
    }
    1
}

#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub missing: Vec<String>,
}

pub fn verify(root: &Path, source: &str, evidence: &Path) -> Report {
    let mut report = Report::default();
    if let Err(error) = source::validate(root, source) {
        report.errors.push(error);
    }
    report.errors.extend(git::validate_plan_inputs(root));
    let required = match plans::validate(root) {
        Ok(value) => value,
        Err(errors) => {
            report.errors.extend(errors);
            Vec::new()
        }
    };
    match git::evidence_files(root, source, evidence) {
        Ok(files) => inspect_files(&files, source, &mut report.errors),
        Err(errors) => report.errors.extend(errors),
    }
    report.errors.extend(history::secret_errors(root));
    report.missing = required;
    report.errors.sort();
    report.errors.dedup();
    report.missing.sort();
    report.missing.dedup();
    report
}

pub fn inspect_attachment(path: &Path, bytes: &[u8], source: &str) -> Vec<String> {
    evidence::inspect(path, bytes, source)
}

pub fn validate_plans(root: &Path) -> Result<Vec<String>, Vec<String>> {
    plans::validate(root)
}

pub fn scan_history(root: &Path) -> Vec<String> {
    history::secret_errors(root)
}

fn inspect_files(files: &[PathBuf], source: &str, errors: &mut Vec<String>) {
    for path in files {
        match fs::read(path) {
            Ok(bytes) => errors.extend(evidence::inspect(path, &bytes, source)),
            Err(error) => errors.push(format!(
                "{}: cannot read evidence: {error}",
                path.to_string_lossy()
            )),
        }
    }
}

fn parse_args(args: &[String]) -> Result<Args, Vec<String>> {
    if args.first().is_none_or(|value| value != "verify") {
        return Err(usage("expected acceptance verify"));
    }
    let mut source = None;
    let mut evidence = None;
    let mut allow_incomplete = false;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--allow-incomplete" if !allow_incomplete => {
                allow_incomplete = true;
                index += 1;
            }
            "--source" if source.is_none() => {
                source = args.get(index + 1).cloned();
                index += 2;
            }
            "--evidence" if evidence.is_none() => {
                evidence = args.get(index + 1).map(PathBuf::from);
                index += 2;
            }
            flag => return Err(usage(&format!("unknown or duplicate argument: {flag}"))),
        }
    }
    match (source, evidence) {
        (Some(source), Some(evidence)) if !source.is_empty() => Ok(Args {
            source,
            evidence,
            allow_incomplete,
        }),
        _ => Err(usage("--source and --evidence are required")),
    }
}

fn usage(problem: &str) -> Vec<String> {
    vec![
        "acceptance command failed".to_string(),
        format!("error: {problem}"),
        "use: acceptance verify --source SOURCE --evidence PATH [--allow-incomplete]".to_string(),
    ]
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("{error}");
    }
}
