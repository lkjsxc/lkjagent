use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::RepoFile;

pub fn collect_files(root: &Path) -> Result<Vec<RepoFile>, String> {
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
        let path = root.join(relative);
        if path.is_file() {
            files.push(read_file(&path, relative)?);
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
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
