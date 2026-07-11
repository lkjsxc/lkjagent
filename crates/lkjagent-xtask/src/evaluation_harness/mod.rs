mod clock;
mod evidence;
mod hash {
    use sha2::{Digest, Sha256};
    pub fn bytes(input: &[u8]) -> String {
        format!("sha256:{:x}", Sha256::digest(input))
    }
    pub fn valid(value: &str) -> bool {
        value.len() == 71
            && value.starts_with("sha256:")
            && value[7..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    }
}
mod pty;
mod scenario;
mod snapshot;

use std::fs;
use std::path::{Component, Path};

pub use clock::{FakeClock, FaultInjector};
pub use evidence::{fixture_errors, validate, Facts};
pub use hash::bytes as sha256;
pub use pty::validate_cast;

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let scenarios = source_scenarios(root)?;
    let scenario_fingerprint = fingerprint(&scenarios);
    let capture = snapshot::create(root).map_err(|error| vec![error])?;
    let pty = pty::record(root, &capture, &scenario_fingerprint).map_err(|error| vec![error])?;
    if pty.frame_count < 3 || !hash::valid(&pty.cast_fingerprint) {
        return Err(vec!["PTY recorder did not return bound raw frames".into()]);
    }
    let raw_count = snapshot::write_raw_manifest(&capture, &scenario_fingerprint)
        .map_err(|error| vec![error])?;
    snapshot::validate_raw_manifest(&capture, &scenario_fingerprint)
        .map_err(|error| vec![error])?;
    let failures = evidence::check_fixtures(root, &scenario_fingerprint, raw_count);
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

pub fn check_source(root: &Path) -> Result<usize, Vec<String>> {
    source_scenarios(root).map(|scenarios| scenarios.len())
}

fn source_scenarios(root: &Path) -> Result<Vec<scenario::Scenario>, Vec<String>> {
    let fault_ids = clock::exercise(&root.join("evaluation/fault-schedule.tsv"))?;
    let scenarios = scenario::check(root, &fault_ids)?;
    for scenario in &scenarios {
        if scenario.required_check_count < 9 {
            return Err(vec![format!(
                "scenario {} has too few independent checks",
                scenario.id
            )]);
        }
    }
    Ok(scenarios)
}

fn fingerprint(scenarios: &[scenario::Scenario]) -> String {
    let mut source = Vec::new();
    for scenario in scenarios {
        source.extend_from_slice(scenario.id.as_bytes());
        source.push(0);
        source.extend_from_slice(scenario.fingerprint.as_bytes());
        source.push(0);
    }
    hash::bytes(&source)
}

pub fn run_smoke(args: &[String], root: &Path) -> i32 {
    match args {
        [] => report_smoke(root),
        [command] if command == "replay" => report_smoke(root),
        [command] if command == "live" => fail(
            "smoke live",
            "live evidence requires the frozen-source acceptance campaign",
        ),
        _ => fail("smoke", "use: smoke replay"),
    }
}

pub fn run_replay(root: &Path) -> Result<(), String> {
    check(root).map_err(|failures| failures.join("\n"))
}

fn report_smoke(root: &Path) -> i32 {
    match run_replay(root) {
        Ok(()) => {
            println!("ok smoke replay");
            0
        }
        Err(error) => fail("smoke replay", &error),
    }
}

pub fn run_benchmark(args: &[String], root: &Path) -> i32 {
    match args {
        [command] if command == "check-corpus" => match check_source(root) {
            Ok(_) => {
                println!("ok bench check-corpus");
                0
            }
            Err(failures) => fail("bench check-corpus", &failures.join("\n")),
        },
        [command, ..] if command == "run" => fail(
            "benchmark run",
            "live benchmark summary lacks raw source-bound evaluation authority",
        ),
        _ => fail("benchmark", "use: bench check-corpus"),
    }
}

pub fn validate_corpus(root: &Path) -> Result<usize, String> {
    check_source(root).map_err(|failures| failures.join("\n"))
}

pub(super) fn check_scenario_seed(path: &Path, id: &str, failures: &mut Vec<String>) {
    let manifest = path.join("seed-manifest.tsv");
    let text = fs::read_to_string(&manifest).unwrap_or_else(|error| {
        failures.push(format!("could not read {}: {error}", manifest.display()));
        String::new()
    });
    let mut rows = 0;
    for row in text.lines().skip(1) {
        rows += 1;
        let fields = row.split('\t').collect::<Vec<_>>();
        let safe = fields.first().is_some_and(|value| {
            !value.is_empty()
                && Path::new(value)
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
        });
        if fields.len() != 4 || !safe {
            failures.push(format!("scenario {id} seed row is malformed"));
            continue;
        }
        let seed = path.join("seed").join(fields[0]);
        let bytes = fs::read(&seed).unwrap_or_default();
        if bytes.is_empty() || hash::bytes(&bytes) != fields[3] || seed.is_symlink() {
            failures.push(format!("scenario {id} seed differs: {}", fields[0]));
        }
    }
    if rows < 2 {
        failures.push(format!("scenario {id} has fewer than two seed files"));
    }
}

pub fn run_experiment(args: &[String], root: &Path) -> i32 {
    if args.first().map(String::as_str) != Some("run") {
        return fail("experiment", "use: experiment run");
    }
    let status = std::process::Command::new("python3")
        .arg(root.join("evaluation/experiment-runner.py"))
        .arg(root)
        .status();
    match status {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => fail("experiment run", &error.to_string()),
    }
}

pub fn reject_unbound_command(name: &str) -> i32 {
    fail(
        name,
        "summary-only command lacks source-bound authority; use anchored source scenarios",
    )
}

fn fail(name: &str, message: &str) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}
