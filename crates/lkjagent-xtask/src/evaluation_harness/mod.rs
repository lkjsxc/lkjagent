mod clock;
mod evidence;
mod hash;
mod pty;
mod scenario;
mod scenario_seed;
mod snapshot;

use std::path::Path;

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
