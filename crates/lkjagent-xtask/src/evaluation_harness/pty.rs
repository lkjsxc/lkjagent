use super::{clock, hash, scenario, snapshot};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PtyFacts {
    pub cast_fingerprint: String,
    pub frame_count: usize,
}
#[rustfmt::skip]
pub fn validate_cast(_path: &Path) -> Result<PtyFacts, String> { Err("PTY scenario is incomplete; generic PTY evidence is not supported".into()) }
#[rustfmt::skip]
pub fn reject() -> Result<(), String> { Err("PTY scenario is incomplete; canned terminal output is forbidden".into()) }

#[rustfmt::skip]
pub fn campaign(root: &Path, alias: &str, endpoint: &Path, probe: bool) -> Result<String, String> {
    let scenario = scenario::load(root, alias)?; scenario::validate_seed(&scenario)?;
    let endpoint = scenario::endpoint_file(endpoint)?;
    let source = exact_clean_head(root)?; let binary_source = build(root)?;
    let capture = snapshot::create()?;
    fs::copy(binary_source, &capture.binary).map_err(|error| error.to_string())?;
    snapshot::copy_seed(&scenario.path.join("seed"), &capture.workspace)?;
    let before = snapshot::manifest(&capture.workspace)?;
    let env = runtime_env(&capture, &endpoint); let first = &scenario.turns[0].1;
    public(&capture, &env, &["send", "--new", first], Duration::from_secs(30))?;
    if probe {
        let run = public_output(&capture, &env, &["run", "--once"], Duration::from_secs(1900))?;
        let status = public_output(&capture, &env, &["status"], Duration::from_secs(30))?;
        let (message, provider) = finish(root, &source, &scenario, &capture, &before, &[run, status], "probe")?;
        return if provider == 0 { Err(message) } else {
            Ok(format!("ok campaign probe-endpoint source={source} provider_exchange_count={provider} semantic_status=not-evaluated")) };
    }
    run_schedule(root, &source, &scenario, &capture, &before, &env)?;
    Ok(format!("ok campaign source={source} scenario={alias} semantic_status=not-evaluated"))
}
#[rustfmt::skip]
fn run_schedule(root: &Path, source: &str, scenario: &scenario::Scenario, capture: &snapshot::Capture,
    before: &str, env: &BTreeMap<String,String>) -> Result<(), String> {
    let started = Instant::now(); let binary = capture.binary.clone(); let cwd = capture.root.clone(); let daemon_env = env.clone();
    thread::scope(|scope| -> Result<(), String> {
        let daemon = scope.spawn(move || clock::command(&binary, &["--data".into(), cwd.join("data").display().to_string(), "run".into()],
            &cwd, &daemon_env, Duration::from_secs(903)));
        for (offset, text) in scenario.turns.iter().skip(1) {
            wait_until(started, Duration::from_secs(*offset)); public(capture, env, &["send", text], Duration::from_secs(30))?;
        }
        wait_until(started, Duration::from_secs(901));
        let status = public_output(capture, env, &["status"], Duration::from_secs(30))?;
        let daemon = daemon.join().map_err(|_| "daemon capture thread failed")??;
        if !daemon.timed_out || status.code != Some(0) { return Err("daemon was not alive through the bounded observation".into()); }
        finish(root, source, scenario, capture, before, &[status], "run").map(|_| ())
    })
}
#[rustfmt::skip]
fn exact_clean_head(root: &Path) -> Result<String, String> {
    let output = Command::new("git").args(["status", "--porcelain", "--untracked-files=no"]).current_dir(root).output().map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stdout.is_empty() { return Err("campaign requires a clean exact HEAD".into()); }
    let output = Command::new("git").args(["rev-parse", "HEAD"]).current_dir(root).output().map_err(|e| e.to_string())?;
    let source = String::from_utf8(output.stdout).map_err(|e| e.to_string())?.trim().to_string();
    if source.len() != 40 { return Err("HEAD did not resolve to a full commit".into()); } Ok(source)
}
#[rustfmt::skip]
fn build(root: &Path) -> Result<PathBuf, String> {
    let target = std::env::var("CARGO_TARGET_DIR")
        .map_err(|_| "campaign requires an explicit CARGO_TARGET_DIR")?;
    let target = PathBuf::from(target);
    if !target.is_absolute() { return Err("campaign CARGO_TARGET_DIR must be absolute".into()); }
    let mut env = BTreeMap::new();
    env.insert("CARGO_TARGET_DIR".into(), target.display().to_string());
    env.insert("CARGO_INCREMENTAL".into(), "0".into());
    env.insert("PATH".into(), std::env::var("PATH").map_err(|_| "PATH is unavailable")?);
    let args = ["build", "--locked", "--release", "-p", "lkjagent-app"].map(str::to_string);
    let output = clock::command(Path::new("cargo"), &args, root, &env, Duration::from_secs(3600))?;
    if output.code != Some(0) || output.timed_out { return Err("clean exact HEAD release build failed".into()); }
    Ok(target.join("release/lkjagent"))
}
#[rustfmt::skip]
fn runtime_env(capture: &snapshot::Capture, endpoint: &BTreeMap<String,String>) -> BTreeMap<String,String> {
    let mut env = endpoint.clone(); env.insert("LKJAGENT_WORKSPACE_ROOT".into(), capture.workspace.display().to_string());
    env.insert("HOME".into(), capture.root.display().to_string()); env
}
#[rustfmt::skip]
fn public(capture: &snapshot::Capture, env: &BTreeMap<String,String>, args: &[&str], timeout: Duration) -> Result<(), String> {
    let output = public_output(capture, env, args, timeout)?;
    if output.code == Some(0) && !output.timed_out { Ok(()) } else { Err("copied public command failed".into()) }
}
#[rustfmt::skip]
fn public_output(capture: &snapshot::Capture, env: &BTreeMap<String,String>, args: &[&str], timeout: Duration) -> Result<clock::Output,String> {
    let mut bound = vec!["--data".into(), capture.data.display().to_string()]; bound.extend(args.iter().map(|arg| (*arg).to_string()));
    clock::command(&capture.binary, &bound, &capture.root, env, timeout)
}
#[rustfmt::skip]
fn wait_until(start: Instant, duration: Duration) { while start.elapsed() < duration { thread::sleep(Duration::from_millis(20)); } }
#[rustfmt::skip]
fn finish(root: &Path, source: &str, scenario: &scenario::Scenario, capture: &snapshot::Capture, before: &str,
    outputs: &[clock::Output], mode: &str) -> Result<(String,u64),String> {
    let after = snapshot::manifest(&capture.workspace)?;
    let facts = snapshot::sqlite_facts(root, &capture.data.join("lkjagent.sqlite3"), &capture.raw.join("state.sqlite3"))?;
    let provider = table_count(&facts, "provider_exchanges");
    let activity = ["runtime_decisions", "effect_journal", "check_results"].iter().map(|name| table_count(&facts, name)).sum::<u64>();
    let binary = hash::bytes(&fs::read(&capture.binary).map_err(|e| e.to_string())?); let mut command_bytes = Vec::new();
    for output in outputs { command_bytes.extend_from_slice(&output.stdout); command_bytes.push(0); command_bytes.extend_from_slice(&output.stderr);
        command_bytes.push(0); command_bytes.extend_from_slice(format!("{:?}:{}:{}", output.code, output.elapsed.as_millis(), output.timed_out).as_bytes()); }
    let sanitized = format!("field\tvalue\nsource_commit\t{source}\nscenario\t{}\nscenario_sha256\t{}\nbinary_sha256\t{binary}\nmode\t{mode}\nsemantic_status\tnot-evaluated\nprovider_exchange_count\t{provider}\nactivity_count\t{activity}\ncommand_count\t{}\ncommand_capture_sha256\t{}\nworkspace_before_sha256\t{}\nworkspace_after_sha256\t{}\nworkspace_diff_sha256\t{}\n", scenario.id, scenario.fingerprint, outputs.len(), hash::bytes(&command_bytes), hash::bytes(before.as_bytes()), hash::bytes(after.as_bytes()), hash::bytes(snapshot::diff(before, &after).as_bytes()));
    let directory = root.join("evaluation/evidence").join(source); fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    fs::write(directory.join(format!("campaign-{}-{mode}.tsv", scenario.id)), &sanitized).map_err(|e| e.to_string())?;
    Ok((format!("endpoint probe unsupported: sanitized durable facts provider_exchange_count={provider} activity_count={activity} semantic_status=not-evaluated"), provider))
}
#[rustfmt::skip]
fn table_count(facts: &str, table: &str) -> u64 { facts.lines().skip(1).find_map(|line| {
    let (name,count)=line.split_once('\t')?; (name==table).then(|| count.parse().unwrap_or(0)) }).unwrap_or(0) }
