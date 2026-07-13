#![cfg(unix)]

use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[test]
fn native_binary_enters_and_restores_a_unix_pty() -> Result<(), Box<dyn Error>> {
    if !Command::new("script")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
    {
        return Ok(());
    }
    let data = std::env::temp_dir().join(format!("lkjagent-tui-pty-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&data);
    let command = format!(
        "{} --data {} tui",
        env!("CARGO_BIN_EXE_lkjagent"),
        data.display()
    );
    let mut child = Command::new("timeout")
        .args(["8s", "script", "-qfec", &command, "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    thread::sleep(Duration::from_millis(500));
    let Some(mut input) = child.stdin.take() else {
        return Err("PTY stdin was not piped".into());
    };
    input.write_all(&[3])?;
    drop(input);
    let output = child.wait_with_output()?;
    assert!(output.status.success(), "PTY command failed: {output:?}");
    assert!(data.join("lkjagent.sqlite3").is_file());
    assert!(output
        .stdout
        .windows(8)
        .any(|bytes| bytes == b"\x1b[?1049h"));
    assert!(output
        .stdout
        .windows(8)
        .any(|bytes| bytes == b"\x1b[?1049l"));
    Ok(())
}
