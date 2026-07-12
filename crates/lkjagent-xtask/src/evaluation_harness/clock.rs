use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const CAPTURE_LIMIT: u64 = 1_048_576;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fault {
    pub injection_id: String,
    pub boundary: String,
    pub outcome: String,
    pub advance_ms: u64,
}
#[derive(Default)]
pub struct FakeClock {
    now_ms: u64,
}
impl FakeClock {
    #[rustfmt::skip]
    pub fn now_ms(&self) -> u64 { self.now_ms }
    #[rustfmt::skip]
    pub fn advance_to(&mut self, target_ms: u64) -> Result<(), String> {
        if target_ms < self.now_ms { return Err("fake clock monotonic regression".into()); }
        self.now_ms = target_ms; Ok(())
    }
}
#[derive(Clone)]
pub struct FaultInjector {
    faults: Vec<Fault>,
    cursor: usize,
}
impl FaultInjector {
    #[rustfmt::skip]
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
        let mut faults = Vec::new();
        for (index, line) in text.lines().enumerate().skip(1) {
            let fields = line.split('\t').collect::<Vec<_>>();
            if fields.len() != 4 { return Err(format!("fault row {} is malformed", index + 1)); }
            faults.push(Fault { injection_id: fields[0].into(), boundary: fields[1].into(), outcome: fields[2].into(),
                advance_ms: fields[3].parse().map_err(|_| "invalid fault advance")? });
        }
        if faults.len() < 10 { return Err("fault schedule has fewer than ten injections".into()); }
        let unique = faults.iter().map(|item| &item.injection_id).collect::<BTreeSet<_>>();
        if unique.len() != faults.len() { return Err("fault injection IDs are not unique".into()); }
        Ok(Self { faults, cursor: 0 })
    }
    #[rustfmt::skip]
    pub fn faults(&self) -> &[Fault] { &self.faults }
    #[rustfmt::skip]
    pub fn consume(&mut self, id: &str, boundary: &str, clock: &mut FakeClock) -> Result<String, String> {
        let expected = self.faults.get(self.cursor).ok_or("fault schedule was consumed more than once")?;
        if expected.injection_id != id || expected.boundary != boundary {
            return Err(format!("fault order mismatch: expected {} at {}", expected.injection_id, expected.boundary));
        }
        clock.advance_to(clock.now_ms().checked_add(expected.advance_ms).ok_or("fake clock overflow")?)?;
        self.cursor += 1; Ok(expected.outcome.clone())
    }
    #[rustfmt::skip]
    pub fn finish(&self) -> Result<(), String> {
        if self.cursor == self.faults.len() { Ok(()) } else { Err(format!("{} declared faults were not consumed", self.faults.len() - self.cursor)) }
    }
}
#[rustfmt::skip]
pub fn exercise(path: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let schedule = FaultInjector::from_path(path).map_err(|error| vec![error])?;
    let mut replay = schedule.clone(); let mut clock = FakeClock::default();
    for fault in schedule.faults() { replay.consume(&fault.injection_id, &fault.boundary, &mut clock).map_err(|e| vec![e])?; }
    replay.finish().map_err(|e| vec![e])?;
    Ok(schedule.faults().iter().map(|fault| fault.injection_id.clone()).collect())
}
pub struct Output {
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub elapsed: Duration,
    pub timed_out: bool,
}
#[rustfmt::skip]
pub fn command(binary: &Path, args: &[String], cwd: &Path, env: &BTreeMap<String, String>, timeout: Duration) -> Result<Output, String> {
    let mut command = Command::new(binary);
    command.args(args).current_dir(cwd).env_clear().envs(env).stdin(Stdio::null())
        .stdout(Stdio::piped()).stderr(Stdio::piped()).process_group(0);
    let started = Instant::now();
    let mut child = command.spawn().map_err(|error| format!("start public command: {error}"))?;
    let pid = child.id();
    let stdout = reader(child.stdout.take().ok_or("stdout capture unavailable")?);
    let stderr = reader(child.stderr.take().ok_or("stderr capture unavailable")?);
    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? { break (status, false); }
        if started.elapsed() >= timeout { terminate_group(pid); break (child.wait().map_err(|error| error.to_string())?, true); }
        thread::sleep(Duration::from_millis(20));
    };
    terminate_group(pid);
    let stdout = stdout.join().map_err(|_| "stdout capture thread failed")??;
    let stderr = stderr.join().map_err(|_| "stderr capture thread failed")??;
    Ok(Output { code: status.code(), stdout, stderr, elapsed: started.elapsed(), timed_out })
}
#[rustfmt::skip]
fn reader<R: Read + Send + 'static>(stream: R) -> thread::JoinHandle<Result<Vec<u8>, String>> {
    thread::spawn(move || {
        let mut bytes = Vec::new(); stream.take(CAPTURE_LIMIT + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        if bytes.len() as u64 > CAPTURE_LIMIT { return Err("public command output exceeded capture limit".into()); }
        Ok(bytes)
    })
}
#[rustfmt::skip]
fn terminate_group(pid: u32) {
    let group = format!("-{pid}");
    let _ = Command::new("kill").args(["-TERM", "--", &group]).stdout(Stdio::null()).stderr(Stdio::null()).status();
    thread::sleep(Duration::from_millis(20));
    let _ = Command::new("kill").args(["-KILL", "--", &group]).stdout(Stdio::null()).stderr(Stdio::null()).status();
}
