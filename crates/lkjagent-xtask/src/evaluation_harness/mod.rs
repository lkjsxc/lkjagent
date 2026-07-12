mod clock;
mod evidence;
mod hash {
    use sha2::{Digest, Sha256};
    pub fn bytes(input: &[u8]) -> String {
        format!("sha256:{:x}", Sha256::digest(input))
    }
}
mod pty;
mod scenario;
mod snapshot;

pub use clock::{FakeClock, FaultInjector};
pub use evidence::{fixture_errors, validate, Facts};
pub use hash::bytes as sha256;
pub use pty::validate_cast;
pub use scenario::{endpoint_file, validate as validate_scenario};
use std::path::Path;

#[rustfmt::skip]
pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let faults = clock::exercise(&root.join("evaluation/fault-schedule.tsv"))?;
    let scenarios = scenario::check(root, &faults)?;
    for item in &scenarios { scenario::validate_seed(item).map_err(|error| vec![error])?; }
    if scenarios.len() != scenario::ALIASES.len() { return Err(vec!["tracked scenario coverage differs".into()]); }
    if pty::reject().is_ok() { return Err(vec!["incomplete PTY scenario was accepted".into()]); }
    check_false_positives(root)
}
#[rustfmt::skip]
fn check_false_positives(root: &Path) -> Result<(), Vec<String>> {
    let directory = root.join("evaluation/false-positive-fixtures");
    let expected = [("idle-as-complete.tsv", "useful decision floor not met"),
        ("blocked-as-complete.tsv", "claimed terminal contradicts raw terminal"),
        ("skipped-command.tsv", "required command was skipped"),
        ("zero-test-filter.tsv", "test count must be positive"),
        ("generated-placeholder.tsv", "generated placeholder is not evidence")];
    let mut errors = Vec::new();
    for (name, message) in expected {
        match fixture_errors(&directory.join(name)) {
            Ok(found) if found == [message] => {}, Ok(found) => errors.push(format!("fixture {name} rejected as {found:?}")),
            Err(error) => errors.push(error),
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
#[rustfmt::skip]
pub fn check_source(root: &Path) -> Result<usize, Vec<String>> { check(root).map(|()| scenario::ALIASES.len()) }
#[rustfmt::skip]
pub fn validate_corpus(root: &Path) -> Result<usize, String> { check_source(root).map_err(|errors| errors.join("\n")) }
#[rustfmt::skip]
pub fn run_replay(root: &Path) -> Result<(), String> { check(root).map_err(|errors| errors.join("\n")) }

#[rustfmt::skip]
pub fn run_evidence(args: &[String], root: &Path) -> i32 {
    let source = match args {
        [check, campaign, name] if check == "check" && campaign == "--campaign" && name == "baseline" => None,
        [check, campaign, name, flag, source] if check == "check" && campaign == "--campaign" && name == "baseline" && flag == "--source" => Some(source.as_str()),
        _ => return fail_code("evidence", "use: evidence check --campaign baseline [--source FULL_COMMIT]", 2),
    };
    match evidence::check_baseline(root, source) {
        Ok(line) => { println!("{line}"); 0 }, Err(errors) => fail("evidence check", &errors.join("\n")),
    }
}
#[rustfmt::skip]
pub fn run_campaign(args: &[String], root: &Path) -> i32 {
    let (probe, alias, endpoint) = match args {
        [command, alias, flag, file] if (command == "run" || command == "probe-endpoint") && flag == "--endpoint-file" =>
            (command == "probe-endpoint", alias.as_str(), Path::new(file)),
        _ => return fail_code("campaign", "use: campaign run|probe-endpoint TRACKED_ALIAS --endpoint-file FILE", 2),
    };
    match pty::campaign(root, alias, endpoint, probe) {
        Ok(message) => { println!("{message}"); 0 },
        Err(error) => fail(if probe { "campaign probe-endpoint" } else { "campaign run" }, &error),
    }
}
#[rustfmt::skip]
pub fn run_smoke(args: &[String], root: &Path) -> i32 {
    let supported = args.is_empty() || matches!(args, [value] if value == "replay");
    if !supported { return fail_code("smoke", "use: smoke replay", 2); }
    match run_replay(root) { Ok(()) => { println!("ok smoke replay semantic_status=not-evaluated"); 0 }, Err(e) => fail("smoke replay", &e) }
}
#[rustfmt::skip]
pub fn run_benchmark(args: &[String], root: &Path) -> i32 {
    match args { [x] if x == "check-corpus" => match validate_corpus(root) {
        Ok(_) => { println!("ok bench check-corpus semantic_status=not-evaluated"); 0 }, Err(e) => fail("bench check-corpus", &e),
    }, _ => fail("benchmark", "live benchmark semantics are not implemented") }
}
#[rustfmt::skip]
pub fn run_experiment(_args: &[String], _root: &Path) -> i32 { fail("experiment", "stale scripted semantic paths are rejected") }
#[rustfmt::skip]
pub fn reject_unbound_command(name: &str) -> i32 { fail(name, "summary-only command lacks source-bound authority") }
#[rustfmt::skip]
fn fail(name: &str, message: &str) -> i32 { fail_code(name, message, 1) }
#[rustfmt::skip]
fn fail_code(name: &str, message: &str, code: i32) -> i32 { eprintln!("{name} failed\nexit status: {code}\n{message}"); code }
