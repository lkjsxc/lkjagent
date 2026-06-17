use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellReport {
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub timeout_seconds: u64,
    pub output: String,
}

pub fn run(workspace: &Path, command: &str, timeout_seconds: u64) -> ToolResult<ShellReport> {
    if command.trim().is_empty() {
        return Err(ToolError::invalid("command must not be empty"));
    }
    if timeout_seconds == 0 {
        return Err(ToolError::invalid("timeout must be positive"));
    }
    let mut child = Command::new("/bin/sh")
        .arg("-lc")
        .arg(command)
        .current_dir(workspace)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ToolError::Io(format!("spawn failed: {error}")))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| ToolError::Io("stdout pipe unavailable".to_string()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| ToolError::Io("stderr pipe unavailable".to_string()))?;
    let stdout_reader = read_pipe(stdout);
    let stderr_reader = read_pipe(stderr);
    let timeout = Duration::from_secs(timeout_seconds);
    let started = Instant::now();
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= timeout {
            timed_out = true;
            let _ = child.kill();
            break child.wait()?;
        }
        thread::sleep(Duration::from_millis(20));
    };
    let mut output = String::new();
    output.push_str(&String::from_utf8_lossy(&join_pipe(stdout_reader)?));
    output.push_str(&String::from_utf8_lossy(&join_pipe(stderr_reader)?));
    Ok(ShellReport {
        exit_code: status.code(),
        timed_out,
        timeout_seconds,
        output,
    })
}

pub fn observation(report: &ShellReport) -> String {
    let exit = report
        .exit_code
        .map_or_else(|| "signal".to_string(), |code| code.to_string());
    let mut content = format!("exit_code={exit}\n");
    if report.timed_out {
        content.push_str(&format!("timeout_seconds={}\n", report.timeout_seconds));
    }
    content.push_str(&report.output);
    content
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

fn join_pipe(handle: thread::JoinHandle<std::io::Result<Vec<u8>>>) -> ToolResult<Vec<u8>> {
    match handle.join() {
        Ok(result) => Ok(result?),
        Err(_) => Err(ToolError::Io("pipe reader thread failed".to_string())),
    }
}
