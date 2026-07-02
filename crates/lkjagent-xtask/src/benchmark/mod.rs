mod corpus;

use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [cmd] if cmd == "check-corpus" => check_corpus_gate(root),
        [cmd, rest @ ..] if cmd == "run" => run_suite(root, rest),
        _ => fail(
            "benchmark",
            "use: bench check-corpus | bench run --suite tiny --data DIR",
        ),
    }
}

pub fn validate_corpus(root: &Path) -> Result<usize, String> {
    corpus::validate_all(root).map(|entries| entries.len())
}

fn check_corpus_gate(root: &Path) -> i32 {
    match validate_corpus(root) {
        Ok(_) => {
            println!("ok bench check-corpus");
            0
        }
        Err(error) => fail("bench check-corpus", &error),
    }
}

fn run_suite(root: &Path, args: &[String]) -> i32 {
    let options = match parse_run(root, args) {
        Ok(options) => options,
        Err(error) => return fail("bench run", &error),
    };
    let entries = match corpus::validate_all(root) {
        Ok(entries) => entries,
        Err(error) => return fail("bench run", &error),
    };
    if let Err(error) = fs::create_dir_all(&options.data_dir) {
        return fail("bench run", &format!("create data dir: {error}"));
    }
    let report = report(&options.suite, &entries);
    let path = options.data_dir.join("benchmark-report.md");
    match fs::write(&path, report) {
        Ok(()) => {
            println!("ok bench run report={}", path.display());
            0
        }
        Err(error) => fail("bench run", &format!("write report: {error}")),
    }
}

fn parse_run(root: &Path, args: &[String]) -> Result<RunOptions, String> {
    let mut suite = "tiny".to_string();
    let mut data_dir: Option<PathBuf> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--suite" => {
                suite = args.get(index + 1).ok_or("--suite needs a value")?.clone();
                index += 2;
            }
            "--data" => {
                data_dir = Some(root.join(args.get(index + 1).ok_or("--data needs a value")?));
                index += 2;
            }
            other => return Err(format!("unknown bench run argument: {other}")),
        }
    }
    Ok(RunOptions {
        suite,
        data_dir: data_dir.ok_or("--data is required")?,
    })
}

fn report(suite: &str, entries: &[corpus::Entry]) -> String {
    let mut lines = vec![
        format!("# Benchmark Report"),
        String::new(),
        format!("suite: {suite}"),
    ];
    lines.push(format!("entries: {}", entries.len()));
    lines.push("score: not-run".to_string());
    for entry in entries {
        lines.push(format!(
            "- {} checks={} objective={}",
            entry.name,
            entry.checks.len(),
            entry.objective.trim()
        ));
    }
    lines.join("\n")
}

fn fail(name: &str, message: &str) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}

struct RunOptions {
    suite: String,
    data_dir: PathBuf,
}
