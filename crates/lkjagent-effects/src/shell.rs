use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{EffectError, EffectResult};
use crate::observation::bound;

pub const SHELL_TIMEOUT_SECONDS: u64 = 30;
pub const SHELL_OUTPUT_BYTES: usize = 4_000;

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
    if command.trim().is_empty() {
        return Err(EffectError::Invalid(
            "command must not be empty".to_string(),
        ));
    }
    let mut child = Command::new("/bin/sh")
        .arg("-lc")
        .arg(command)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| EffectError::Io("stdout missing".to_string()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| EffectError::Io("stderr missing".to_string()))?;
    let out = read_pipe(stdout);
    let err = read_pipe(stderr);
    let timeout = Duration::from_secs(timeout_seconds.max(1));
    let started = Instant::now();
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= timeout {
            timed_out = true;
            let _kill_result = child.kill();
            break child.wait()?;
        }
        thread::sleep(Duration::from_millis(20));
    };
    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&join_pipe(out)?));
    output.push_str(&String::from_utf8_lossy(&join_pipe(err)?));
    Ok(ShellReport {
        exit_code: status.code(),
        timed_out,
        output: bound(&output, SHELL_OUTPUT_BYTES),
    })
}

fn read_pipe<R>(mut pipe: R) -> thread::JoinHandle<std::io::Result<Vec<u8>>>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut bytes = Vec::new();
        pipe.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

fn join_pipe(handle: thread::JoinHandle<std::io::Result<Vec<u8>>>) -> EffectResult<Vec<u8>> {
    match handle.join() {
        Ok(result) => Ok(result?),
        Err(_) => Err(EffectError::Io("pipe reader thread failed".to_string())),
    }
}
