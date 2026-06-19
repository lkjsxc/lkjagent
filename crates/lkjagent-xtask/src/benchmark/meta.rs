use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_benchmark::report::{render_markdown, render_tsv, ReportEntry};

pub fn write_reports(run_dir: &Path, entries: &[ReportEntry]) -> Result<(), String> {
    fs::create_dir_all(run_dir).map_err(|error| error.to_string())?;
    fs::write(run_dir.join("report.tsv"), render_tsv(entries))
        .map_err(|error| error.to_string())?;
    fs::write(run_dir.join("summary.md"), render_markdown(entries))
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub fn absolute(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

pub fn run_id() -> String {
    format!("{}-{}", timestamp(), std::process::id())
}

pub fn timestamp() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
        |_| "0".to_string(),
        |duration| duration.as_secs().to_string(),
    )
}

pub fn git_state(root: &Path) -> String {
    let commit =
        git(root, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let dirty = git(root, &["status", "--short"]).is_some_and(|status| !status.trim().is_empty());
    if dirty {
        format!("{commit}-dirty")
    } else {
        commit
    }
}

pub fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

pub fn join_reason(judge: &str, harness: &str) -> String {
    if harness.trim().is_empty() {
        judge.to_string()
    } else {
        format!("{judge}; {harness}")
    }
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
