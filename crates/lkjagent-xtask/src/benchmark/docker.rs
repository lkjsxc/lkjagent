use std::path::Path;
use std::process::{Command, Output};

pub fn compose(
    root: &Path,
    project: &str,
    data_dir: &Path,
    args: &[String],
) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("compose")
        .arg("-p")
        .arg(project)
        .args(args)
        .env("LKJAGENT_DATA_DIR", data_dir)
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not start docker compose: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(command_failure(&output))
    }
}

pub fn down(root: &Path, project: &str, data_dir: &Path) {
    let args = strings(&["down", "--remove-orphans"]);
    let _ = compose(root, project, data_dir, &args);
}

pub fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn command_failure(output: &Output) -> String {
    let status = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |code| code.to_string(),
    );
    let mut text = format!("docker compose failed with status {status}");
    text.push('\n');
    text.push_str(&tail(&String::from_utf8_lossy(&output.stdout)));
    text.push_str(&tail(&String::from_utf8_lossy(&output.stderr)));
    text
}

fn tail(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(20);
    lines
        .into_iter()
        .skip(start)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
