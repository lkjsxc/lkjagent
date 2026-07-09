use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoFile {
    pub path: String,
    pub text: String,
}

impl RepoFile {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    pub fn line_count(&self) -> usize {
        self.text.lines().count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub path: String,
    pub rule: String,
    pub fix: String,
}

impl Violation {
    pub fn new(path: impl Into<String>, rule: impl Into<String>, fix: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            rule: rule.into(),
            fix: fix.into(),
        }
    }

    pub fn message(&self) -> String {
        format!("{}: {}: {}", self.path, self.rule, self.fix)
    }
}

pub fn collect_files(root: &Path) -> Result<Vec<RepoFile>, String> {
    if !root.join(".git").exists() {
        return collect_files_without_git(root);
    }
    collect_files_with_git(root)
}

fn collect_files_with_git(root: &Path) -> Result<Vec<RepoFile>, String> {
    let output = Command::new("git")
        .args(["ls-files", "--cached", "--others", "--exclude-standard"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("git ls-files could not start: {error}"))?;
    if !output.status.success() {
        return Err(command_error("git ls-files", &output));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();
    for relative in stdout.lines().filter(|line| !line.trim().is_empty()) {
        if is_ignored(relative) {
            continue;
        }
        let path = root.join(relative);
        if path.is_file() {
            files.push(read_file(&path, relative)?);
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn collect_files_without_git(root: &Path) -> Result<Vec<RepoFile>, String> {
    let mut files = Vec::new();
    walk_dir(root, root, &mut files)?;
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn walk_dir(root: &Path, dir: &Path, files: &mut Vec<RepoFile>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let relative = relative_path(root, &path)?;
        if is_ignored(&relative) {
            continue;
        }
        if path.is_dir() {
            walk_dir(root, &path, files)?;
        } else if path.is_file() {
            files.push(read_file(&path, &relative)?);
        }
    }
    Ok(())
}

fn read_file(path: &PathBuf, relative: &str) -> Result<RepoFile, String> {
    let bytes = fs::read(path).map_err(|error| format!("{relative}: could not read: {error}"))?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    Ok(RepoFile::new(relative.replace('\\', "/"), text))
}

fn command_error(step: &str, output: &std::process::Output) -> String {
    let status = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |code| code.to_string(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let tail = stderr.lines().last().unwrap_or("no stderr");
    format!("{step} failed with status {status}: {tail}")
}

fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn is_ignored(relative: &str) -> bool {
    relative == "Cargo.lock"
        || relative.split('/').any(|part| {
            matches!(
                part,
                ".git"
                    | ".lkjagent-models"
                    | ".lkjagent-workspace"
                    | ".omx"
                    | "data"
                    | "target"
                    | "tmp"
            )
        })
        || relative.ends_with(".sqlite")
        || relative.ends_with(".sqlite3")
        || relative.ends_with(".sqlite3-shm")
        || relative.ends_with(".sqlite3-wal")
}

pub(crate) fn pairs(path: PathBuf, failures: &mut Vec<String>) -> BTreeMap<String, String> {
    read(path, failures)
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

pub(crate) fn expect(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
    failures: &mut Vec<String>,
) {
    if values.get(key).map(String::as_str) != Some(expected) {
        failures.push(format!("expected {key}={expected}"));
    }
}

pub(crate) fn read(path: PathBuf, failures: &mut Vec<String>) -> String {
    match fs::read_to_string(&path) {
        Ok(text) if !text.is_empty() => text,
        Ok(_) => {
            failures.push(format!("evidence file is empty: {}", path.display()));
            String::new()
        }
        Err(error) => {
            failures.push(format!("could not read {}: {error}", path.display()));
            String::new()
        }
    }
}
