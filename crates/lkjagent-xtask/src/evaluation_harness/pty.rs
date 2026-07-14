use super::{clock, hash, scenario, semantic, snapshot};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

pub use super::pty_cast::validate as validate_cast;
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
    let env = runtime_env(&capture, &endpoint);
    if !probe && scenario.id == "slow-japanese-pty" {
        return terminal_schedule(root, &source, &scenario, &capture, &before, &env);
    }
    let first = &scenario.turns[0].1;
    public(&capture, &env, &["send", "--new", first], Duration::from_secs(30))?;
    if probe {
        let run = public_output(&capture, &env, &["run", "--once"], Duration::from_secs(1900))?;
        let status = public_output(&capture, &env, &["status"], Duration::from_secs(30))?;
        let (message, provider, _) = finish(root, &source, &scenario, &capture, &before, &[run, status], "probe", 0)?;
        return if provider == 0 { Err(message) } else {
            Ok(format!("ok campaign probe-endpoint source={source} provider_exchange_count={provider} semantic_status=not-evaluated")) };
    }
    run_schedule(root, &source, &scenario, &capture, &before, &env)?;
    Ok(format!("ok campaign source={source} scenario={alias} semantic_status=evaluated outcome=passed"))
}
#[rustfmt::skip]
fn terminal_schedule(root: &Path, source: &str, scenario: &scenario::Scenario,
    capture: &snapshot::Capture, before: &str, env: &BTreeMap<String,String>) -> Result<String,String> {
    let recorder=fs::canonicalize(root.join("evaluation/pty-recorder.py")).map_err(|error|error.to_string())?;
    let schedule=fs::canonicalize(scenario.path.join("owner-schedule.tsv")).map_err(|error|error.to_string())?;
    let args = [recorder.display().to_string(), capture.raw.join("terminal.cast").display().to_string(),
        capture.binary.display().to_string(), capture.data.display().to_string(), schedule.display().to_string()];
    let output = clock::command(Path::new("/usr/bin/python3"), &args, &capture.root, env, Duration::from_secs(970))?;
    if output.code != Some(0) || output.timed_out {
        let stderr=String::from_utf8_lossy(&output.stderr);let cause=if stderr.contains("can't open file"){"script-missing"}else if stderr.contains("usage: pty-recorder.py"){"argument-count"}else if stderr.contains("TUI exited early"){"tui-exited"}else if stderr.contains("restart marker"){"restart-marker"}else{"recorder-error"};
        return Err(format!("real PTY campaign capture failed cause={cause} code={:?} timed_out={}", output.code, output.timed_out));
    }
    let duration = output.elapsed.as_secs();
    let (message, _, passed) = finish(root, source, scenario, capture, before, &[output], "run", duration)?;
    if passed { Ok(format!("ok campaign source={source} scenario={} semantic_status=evaluated outcome=passed",scenario.id)) }
    else { Err(message) }
}
#[rustfmt::skip]
fn run_schedule(root: &Path, source: &str, scenario: &scenario::Scenario, capture: &snapshot::Capture,
    before: &str, env: &BTreeMap<String,String>) -> Result<(), String> {
    if matches!(scenario.id.as_str(),"long-artifact-recovery"|"multi-project-development") {
        return restart_schedule(root,source,scenario,capture,before,env);
    }
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
        let duration = started.elapsed().as_secs();
        let (message, _, passed) = finish(root, source, scenario, capture, before, &[status], "run", duration)?;
        if passed { Ok(()) } else { Err(message) }
    })
}
#[rustfmt::skip]
fn restart_schedule(root:&Path,source:&str,scenario:&scenario::Scenario,capture:&snapshot::Capture,before:&str,env:&BTreeMap<String,String>)->Result<(),String>{
 let restart=620;let total=901;
 let started=Instant::now();thread::scope(|scope|->Result<(),String>{
  let binary=capture.binary.clone();let cwd=capture.root.clone();let first_env=env.clone();let first=scope.spawn(move||clock::command(&binary,&["--data".into(),cwd.join("data").display().to_string(),"run".into()],&cwd,&first_env,Duration::from_secs(restart)));
  for (offset,text) in scenario.turns.iter().skip(1).take_while(|turn|turn.0<restart){wait_until(started,Duration::from_secs(*offset));public(capture,env,&["send",text],Duration::from_secs(30))?}
  wait_until(started,Duration::from_secs(restart));let first=first.join().map_err(|_|"first daemon thread failed")??;if !first.timed_out{return Err("first daemon did not reach restart boundary".into())}record_restart(capture)?;
  let binary=capture.binary.clone();let cwd=capture.root.clone();let second_env=env.clone();let second=scope.spawn(move||clock::command(&binary,&["--data".into(),cwd.join("data").display().to_string(),"run".into()],&cwd,&second_env,Duration::from_secs(total-restart+2)));
  for (offset,text) in scenario.turns.iter().skip(1).filter(|turn|turn.0>=restart){wait_until(started,Duration::from_secs(*offset));public(capture,env,&["send",text],Duration::from_secs(30))?}
  wait_until(started,Duration::from_secs(total));let status=public_output(capture,env,&["status"],Duration::from_secs(30))?;let second=second.join().map_err(|_|"second daemon thread failed")??;if !second.timed_out||status.code!=Some(0){return Err("restarted daemon did not reach observation boundary".into())}let duration=started.elapsed().as_secs();let(message,_,passed)=finish(root,source,scenario,capture,before,&[status],"run",duration)?;if passed{Ok(())}else{Err(message)}
 })
}
fn record_restart(capture: &snapshot::Capture) -> Result<(), String> {
    let db = rusqlite::Connection::open(capture.data.join("lkjagent.sqlite3"))
        .map_err(|error| error.to_string())?;
    let mut query=db.prepare("SELECT current_revision_id FROM workspace_documents WHERE current_revision_id IS NOT NULL ORDER BY current_path").map_err(|error|error.to_string())?;
    let rows = query
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    if rows.is_empty() {
        return Err("restart boundary has no settled workspace revision".into());
    }
    let body = rows
        .into_iter()
        .map(|id| format!("revision\t{id}\n"))
        .collect::<String>();
    fs::write(capture.raw.join("restart.marker"), body).map_err(|error| error.to_string())
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
    env.insert("HOME".into(), capture.root.display().to_string());
    env.insert("TERM".into(), "xterm-256color".into()); env.insert("LANG".into(), "C.UTF-8".into()); env
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
#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn finish(root: &Path, source: &str, scenario: &scenario::Scenario, capture: &snapshot::Capture, before: &str,
    outputs: &[clock::Output], mode: &str, duration: u64) -> Result<(String,u64,bool),String> {
    let after = snapshot::manifest(&capture.workspace)?;
    let facts = snapshot::sqlite_facts(root, &capture.data.join("lkjagent.sqlite3"), &capture.raw.join("state.sqlite3"))?;
    let provider = table_count(&facts, "provider_exchanges");
    let activity = ["runtime_decisions", "effect_journal", "checks"].iter().map(|name| table_count(&facts, name)).sum::<u64>();
    let binary = hash::bytes(&fs::read(&capture.binary).map_err(|e| e.to_string())?); let mut command_bytes = Vec::new();
    for output in outputs { command_bytes.extend_from_slice(&output.stdout); command_bytes.push(0); command_bytes.extend_from_slice(&output.stderr);
        command_bytes.push(0); command_bytes.extend_from_slice(format!("{:?}:{}:{}", output.code, output.elapsed.as_millis(), output.timed_out).as_bytes()); }
    let metrics = (mode == "run").then(|| semantic::measure(scenario, capture, before, &after, &facts)).transpose()?;
    let passed = metrics.as_ref().is_some_and(|item| item.passed);
    let detail = metrics.as_ref().and_then(|item| item.fields.iter().find(|(key,_)| key=="semantic_detail").map(|(_,value)|value.clone()))
        .unwrap_or_else(|| if mode == "run" { "measured-native-facts".into() } else { "probe-only".into() });
    let semantic_status = if mode == "run" { "evaluated" } else { "not-evaluated" };
    let outcome = if mode != "run" { "not-evaluated" } else if passed { "passed" } else { "failed" };
    let mut sanitized = format!("field\tvalue\nsource_commit\t{source}\nscenario\t{}\nscenario_sha256\t{}\nbinary_sha256\t{binary}\nmode\t{mode}\nsemantic_status\t{semantic_status}\noutcome\t{outcome}\nsemantic_detail\t{detail}\nduration_seconds\t{duration}\nprovider_exchange_count\t{provider}\nactivity_count\t{activity}\ncommand_count\t{}\ncommand_capture_sha256\t{}\nworkspace_before_sha256\t{}\nworkspace_after_sha256\t{}\nworkspace_diff_sha256\t{}\n", scenario.id, scenario.fingerprint, outputs.len(), hash::bytes(&command_bytes), hash::bytes(before.as_bytes()), hash::bytes(after.as_bytes()), hash::bytes(snapshot::diff(before, &after).as_bytes()));
    if let Some(metrics)=metrics { for (key,value) in metrics.fields { if key!="semantic_detail" { append_field(&mut sanitized,&key,&value)?; } } }
    let directory = root.join("evaluation/evidence").join(source); fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    fs::write(directory.join(format!("campaign-{}-{mode}.tsv", scenario.id)), &sanitized).map_err(|e| e.to_string())?;
    Ok((format!("sanitized durable facts provider_exchange_count={provider} activity_count={activity} semantic_status={semantic_status} outcome={outcome} detail={detail}"), provider, passed))
}
fn append_field(output: &mut String, key: &str, value: &str) -> Result<(), String> {
    if key.is_empty() || key.contains(['\t', '\n', '\r']) || value.contains(['\t', '\n', '\r']) {
        return Err("semantic fact is not a sanitized scalar".into());
    }
    output.push_str(key);
    output.push('\t');
    output.push_str(value);
    output.push('\n');
    Ok(())
}
#[rustfmt::skip]
fn table_count(facts: &str, table: &str) -> u64 { facts.lines().skip(1).find_map(|line| {
    let (name,count)=line.split_once('\t')?; (name==table).then(|| count.parse().unwrap_or(0)) }).unwrap_or(0) }
