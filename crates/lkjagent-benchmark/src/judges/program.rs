use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub fn judge_fibonacci(workspace: &Path) -> Result<(), String> {
    let script = workspace.join("solve.sh");
    for n in [0_u32, 1, 2, 7, 19, 64, 127, 200] {
        let output = run_shell(&script, &format!("{n}\n"), Duration::from_secs(3))?;
        let expected = fib_mod(n).to_string();
        if output.trim() != expected {
            return Err(format!("n={n}: expected {expected}, got {}", output.trim()));
        }
    }
    Ok(())
}

pub fn judge_rank(workspace: &Path) -> Result<(), String> {
    let script = workspace.join("rank.sh");
    let cases = [
        ("10\n2\n2\n-1\n", "-1\n2\n10\n"),
        ("5\n5\n4\n3\n4\n", "3\n4\n5\n"),
        ("100\n9\n-20\n0\n9\n", "-20\n0\n9\n100\n"),
    ];
    for (input, expected) in cases {
        let output = run_shell(&script, input, Duration::from_secs(3))?;
        if normalize(&output) != normalize(expected) {
            return Err(format!(
                "expected {}, got {}",
                compact(expected),
                compact(&output)
            ));
        }
    }
    Ok(())
}

fn run_shell(script: &Path, input: &str, timeout: Duration) -> Result<String, String> {
    if !script.exists() {
        return Err(format!("script missing: {}", script.display()));
    }
    let mut child = Command::new("sh")
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not start script: {error}"))?;
    let Some(mut stdin) = child.stdin.take() else {
        return Err("could not open script stdin".to_string());
    };
    stdin
        .write_all(input.as_bytes())
        .map_err(|error| format!("could not write script input: {error}"))?;
    drop(stdin);
    let start = Instant::now();
    loop {
        if child
            .try_wait()
            .map_err(|error| format!("script wait failed: {error}"))?
            .is_some()
        {
            break;
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err("script timed out".to_string());
        }
        thread::sleep(Duration::from_millis(20));
    }
    let output = child
        .wait_with_output()
        .map_err(|error| format!("script output failed: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "script exited with error: {}",
            compact(&String::from_utf8_lossy(&output.stderr))
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .chars()
        .take(4096)
        .collect())
}

fn fib_mod(n: u32) -> u32 {
    let mut a = 0_u32;
    let mut b = 1_u32;
    for _ in 0..n {
        let c = (a + b) % 9973;
        a = b;
        b = c;
    }
    a
}

fn normalize(text: &str) -> String {
    let mut output = text.trim_end().to_string();
    output.push('\n');
    output
}

fn compact(text: &str) -> String {
    text.replace('\n', "\\n").chars().take(160).collect()
}
