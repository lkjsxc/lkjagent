use std::path::Path;
use std::process::Command;

use super::secret;

pub fn secret_errors(root: &Path) -> Vec<String> {
    let commits = match git(root, &["rev-list", "HEAD"]) {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>(),
        _ => return vec!["cannot enumerate reachable Git history for secret scan".to_string()],
    };
    let mut args = vec!["--objects".to_string()];
    args.extend(commits);
    let objects = match Command::new("git")
        .arg("rev-list")
        .args(args)
        .current_dir(root)
        .output()
    {
        Ok(output) if output.status.success() => output.stdout,
        _ => return vec!["cannot enumerate reachable Git objects for secret scan".to_string()],
    };
    let mut errors = inspect_objects(root, &objects);
    errors.extend(inspect_index(root));
    errors.sort();
    errors.dedup();
    errors
}

fn inspect_index(root: &Path) -> Vec<String> {
    let output = match git(root, &["ls-files", "--stage", "-z"]) {
        Ok(value) if value.status.success() => value.stdout,
        _ => return vec!["cannot enumerate Git index for secret scan".to_string()],
    };
    output
        .split(|byte| *byte == 0)
        .filter_map(|entry| {
            let text = String::from_utf8_lossy(entry);
            let object = text.split_whitespace().nth(1)?;
            inspect_object(root, object)
        })
        .collect()
}

fn inspect_objects(root: &Path, objects: &[u8]) -> Vec<String> {
    let mut errors = Vec::new();
    for line in String::from_utf8_lossy(objects).lines() {
        let Some(object) = line.split_whitespace().next() else {
            continue;
        };
        if let Some(error) = inspect_object(root, object) {
            errors.push(error);
        }
    }
    errors.sort();
    errors.dedup();
    errors
}

fn inspect_object(root: &Path, object: &str) -> Option<String> {
    let kind = git(root, &["cat-file", "-t", object]).ok()?;
    if !kind.status.success() || kind.stdout != b"blob\n" {
        return None;
    }
    let blob = git(root, &["cat-file", "blob", object]).ok()?.stdout;
    if let Some(kind) = secret::kind(&blob) {
        return Some(format!(
            "Git object {object}: {kind} pattern detected; bytes suppressed"
        ));
    }
    if secret::contains_loaded(&blob) {
        return Some(format!(
            "Git object {object}: loaded secret fingerprint detected; bytes suppressed"
        ));
    }
    None
}

fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))
}
