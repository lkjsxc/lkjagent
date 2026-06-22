use std::path::Path;
use std::process::Command;

pub fn run_quiet_test(root: &Path) -> Result<(), Vec<String>> {
    run_step(root, "cargo fmt --check", &["fmt", "--check"])?;
    run_step(
        root,
        "cargo clippy --workspace --all-targets -- -D warnings",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_step(root, "cargo test --workspace", &["test", "--workspace"])?;
    Ok(())
}

fn run_step(root: &Path, label: &str, args: &[&str]) -> Result<(), Vec<String>> {
    let output = Command::new("cargo")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| {
            vec![
                format!("quiet test failed at {label}"),
                "exit status: 1".to_string(),
                format!("could not start command: {error}"),
            ]
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(command_failure(label, &output))
    }
}

fn command_failure(label: &str, output: &std::process::Output) -> Vec<String> {
    let status = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |code| code.to_string(),
    );
    let mut lines = vec![
        format!("quiet test failed at {label}"),
        format!("exit status: {status}"),
    ];
    lines.extend(tail(&String::from_utf8_lossy(&output.stdout)));
    lines.extend(tail(&String::from_utf8_lossy(&output.stderr)));
    lines
}

fn tail(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(20);
    lines
        .into_iter()
        .skip(start)
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}
