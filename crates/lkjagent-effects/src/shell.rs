use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::os::fd::AsFd;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{EffectError, EffectResult};
use crate::observation::bound;

pub const SHELL_TIMEOUT_SECONDS: u64 = 30;
pub const SHELL_OUTPUT_BYTES: usize = 4_000;
const SHELL_CLEANUP_MILLIS: u64 = 500;

static NEXT_SHELL_SCOPE: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellReport {
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub output: String,
}

impl ShellReport {
    pub fn success(&self) -> bool {
        !self.timed_out && self.exit_code == Some(0)
    }
}

pub fn run(root: &Path, command: &str, timeout_seconds: u64) -> EffectResult<ShellReport> {
    let ordinal = NEXT_SHELL_SCOPE.fetch_add(1, Ordering::Relaxed);
    let nonce = fs::read_to_string("/proc/sys/kernel/random/uuid")?;
    let scope = format!("{}-{ordinal}", nonce.trim());
    run_supervised(root, command, timeout_seconds, &scope)
}

fn run_supervised(
    root: &Path,
    command: &str,
    timeout_seconds: u64,
    scope: &str,
) -> EffectResult<ShellReport> {
    if command.trim().is_empty() {
        return Err(EffectError::Invalid(
            "command must not be empty".to_string(),
        ));
    }
    let mut process = Command::new("/bin/sh");
    process
        .arg("-lc")
        .arg(command)
        .current_dir(root)
        .env("LKJAGENT_SHELL_SCOPE", scope)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let mut child = process.spawn()?;
    let raw_pid = i32::try_from(child.id())
        .map_err(|error| EffectError::Io(format!("child pid is invalid: {error}")))?;
    let group = rustix::process::Pid::from_raw(raw_pid)
        .ok_or_else(|| EffectError::Io("child process group is invalid".to_string()))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| EffectError::Io("stdout missing".to_string()))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| EffectError::Io("stderr missing".to_string()))?;
    set_nonblocking(&stdout)?;
    set_nonblocking(&stderr)?;
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let timeout = Duration::from_secs(timeout_seconds.max(1));
    let started = Instant::now();
    let mut timed_out = false;
    let status = loop {
        drain_pipe(&mut stdout, &mut out)?;
        drain_pipe(&mut stderr, &mut err)?;
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if started.elapsed() >= timeout {
            timed_out = true;
            let _kill_result =
                rustix::process::kill_process_group(group, rustix::process::Signal::Kill);
            break None;
        }
        thread::sleep(Duration::from_millis(20));
    };
    let cleanup_deadline = Instant::now() + Duration::from_millis(SHELL_CLEANUP_MILLIS);
    timed_out |= terminate_descendants(scope, cleanup_deadline)?;
    timed_out |= !drain_until_closed(
        &mut stdout,
        &mut stderr,
        &mut out,
        &mut err,
        cleanup_deadline,
    )?;
    let status = match status {
        Some(status) => Some(status),
        None => child.try_wait()?,
    };
    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&out));
    output.push_str(&String::from_utf8_lossy(&err));
    Ok(ShellReport {
        exit_code: status.and_then(|value| value.code()),
        timed_out,
        output: bound(&output, SHELL_OUTPUT_BYTES),
    })
}

#[rustfmt::skip]
fn marked_pids(scope: &str, deadline: Instant) -> EffectResult<BTreeSet<i32>> {
    let marker = format!("LKJAGENT_SHELL_SCOPE={scope}");
    let mut pids = BTreeSet::new();
    for entry in fs::read_dir("/proc")? {
        if Instant::now() >= deadline { return Err(EffectError::Timeout("shell cleanup exceeded its bound".to_string())); }
        let entry = entry?;
        let Ok(pid) = entry.file_name().to_string_lossy().parse::<i32>() else { continue };
        let Ok(environ) = fs::read(entry.path().join("environ")) else { continue };
        if environ.split(|byte| *byte == 0).any(|value| value == marker.as_bytes()) { pids.insert(pid); }
    }
    if Instant::now() >= deadline { return Err(EffectError::Timeout("shell cleanup exceeded its bound".to_string())); }
    Ok(pids)
}

#[rustfmt::skip]
fn terminate_descendants(scope: &str, deadline: Instant) -> EffectResult<bool> {
    let mut frozen = BTreeMap::new();
    let mut failure = None;
    loop {
        if Instant::now() >= deadline { failure = Some(EffectError::Timeout("shell cleanup exceeded its bound".to_string())); break; }
        let marked = match marked_pids(scope, deadline) { Ok(marked) => marked, Err(error) => { failure = Some(error); break; } };
        let known = frozen.keys().copied().collect::<BTreeSet<_>>();
        let new = marked.difference(&known).copied().collect::<Vec<_>>();
        if new.is_empty() { break; }
        for raw in new {
            let Some(pid) = rustix::process::Pid::from_raw(raw) else { continue };
            let Ok(pidfd) = rustix::process::pidfd_open(pid, rustix::process::PidfdFlags::empty()) else { continue };
            if rustix::process::pidfd_send_signal(&pidfd, rustix::process::Signal::Stop).is_ok() { frozen.insert(raw, pidfd); }
        }
    }
    for pidfd in frozen.values() {
        let _kill = rustix::process::pidfd_send_signal(pidfd, rustix::process::Signal::Kill);
    }
    if let Some(error) = failure { return Err(error); }
    Ok(!frozen.is_empty())
}

fn set_nonblocking<F: AsFd>(fd: &F) -> EffectResult<()> {
    let flags = rustix::fs::fcntl_getfl(fd)
        .map_err(|error| EffectError::Io(format!("pipe flags failed: {error}")))?;
    rustix::fs::fcntl_setfl(fd, flags | rustix::fs::OFlags::NONBLOCK)
        .map_err(|error| EffectError::Io(format!("pipe nonblocking failed: {error}")))
}

fn drain_pipe<R: Read>(pipe: &mut R, target: &mut Vec<u8>) -> EffectResult<bool> {
    let mut bytes = [0_u8; 1_024];
    for _chunk in 0..64 {
        match pipe.read(&mut bytes) {
            Ok(0) => return Ok(true),
            Ok(count) => {
                let keep = count.min(SHELL_OUTPUT_BYTES.saturating_sub(target.len()));
                target.extend_from_slice(&bytes[..keep]);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(false),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(false)
}

fn drain_until_closed<O: Read, E: Read>(
    out: &mut O,
    err: &mut E,
    out_bytes: &mut Vec<u8>,
    err_bytes: &mut Vec<u8>,
    deadline: Instant,
) -> EffectResult<bool> {
    let (mut out_closed, mut err_closed) = (false, false);
    while Instant::now() < deadline {
        out_closed |= drain_pipe(out, out_bytes)?;
        err_closed |= drain_pipe(err, err_bytes)?;
        if out_closed && err_closed {
            return Ok(true);
        }
        thread::sleep(Duration::from_millis(5));
    }
    Ok(false)
}
