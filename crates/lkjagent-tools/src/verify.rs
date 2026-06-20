use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::error::{ToolError, ToolResult};

pub fn cargo(workspace: &Path, gate: &str, package: &str, timeout: u64) -> ToolResult<String> {
    let mut args = match gate {
        "fmt" => vec!["fmt".to_string(), "--check".to_string()],
        "check" => vec!["check".to_string()],
        "test" => vec!["test".to_string()],
        "clippy" => vec!["clippy".to_string()],
        _ => return Err(ToolError::invalid("unknown cargo gate")),
    };
    if !package.trim().is_empty() && gate != "fmt" {
        args.extend(["-p".to_string(), package.to_string()]);
    }
    run(workspace, "cargo", &args, timeout)
}

pub fn xtask(workspace: &Path, gate: &str, timeout: u64) -> ToolResult<String> {
    let args = match gate {
        "check-docs" => vec!["run", "-p", "lkjagent-xtask", "--", "check-docs"],
        "check-lines" => vec!["run", "-p", "lkjagent-xtask", "--", "check-lines"],
        "check-style" => vec!["run", "-p", "lkjagent-xtask", "--", "check-style"],
        "benchmark-check-corpus" => vec![
            "run",
            "-p",
            "lkjagent-xtask",
            "--",
            "benchmark",
            "check-corpus",
        ],
        "quiet-test" => vec!["run", "-p", "lkjagent-xtask", "--", "quiet", "test"],
        "quiet-verify" => vec!["run", "-p", "lkjagent-xtask", "--", "quiet", "verify"],
        _ => return Err(ToolError::invalid("unknown xtask gate")),
    };
    let args = args.into_iter().map(str::to_string).collect::<Vec<_>>();
    run(workspace, "cargo", &args, timeout)
}

fn run(workspace: &Path, program: &str, args: &[String], timeout: u64) -> ToolResult<String> {
    if timeout == 0 || timeout > 900 {
        return Err(ToolError::invalid("timeout must be 1..900 seconds"));
    }
    let mut child = Command::new(program)
        .args(args)
        .current_dir(workspace)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ToolError::Io(format!("spawn failed: {error}")))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| ToolError::Io("stdout pipe unavailable".to_string()))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| ToolError::Io("stderr pipe unavailable".to_string()))?;
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if start.elapsed() >= Duration::from_secs(timeout) {
            let _ = child.kill();
            return Err(ToolError::invalid("verification timed out"));
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let mut bytes = Vec::new();
    stdout.read_to_end(&mut bytes)?;
    stderr.read_to_end(&mut bytes)?;
    let text = String::from_utf8_lossy(&bytes);
    Ok(format!(
        "program={program}\nargs={}\nexit_code={}\n{}",
        args.join(" "),
        status
            .code()
            .map_or_else(|| "signal".to_string(), |code| code.to_string()),
        text
    ))
}
