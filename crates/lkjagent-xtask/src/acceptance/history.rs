use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use super::secret;

pub fn derivations(root: &Path, source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Some(boundary) = introduction(root, "crates/lkjagent-xtask/src/acceptance.rs") else {
        return out;
    };
    let plans = [
        "evaluation/workgraph.tsv",
        "evaluation/acceptance.tsv",
        "evaluation/experiment-plan.tsv",
    ];
    if plans.iter().all(|path| {
        introduction(root, path).is_some_and(|commit| ancestor(root, &commit, &boundary))
    }) && ancestor(root, &boundary, source)
    {
        out.insert("D05".into());
    }
    if out.contains("D05") && coherent_trailers(root, &boundary, source) {
        out.insert("D06".into());
    }
    out
}

fn introduction(root: &Path, path: &str) -> Option<String> {
    let output = git(root, &["log", "--diff-filter=A", "--format=%H", "--", path]).ok()?;
    output
        .status
        .success()
        .then(|| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .last()
                .unwrap_or_default()
                .to_string()
        })
        .filter(|value| !value.is_empty())
}

fn ancestor(root: &Path, older: &str, newer: &str) -> bool {
    git(root, &["merge-base", "--is-ancestor", older, newer])
        .is_ok_and(|output| output.status.success())
}

fn coherent_trailers(root: &Path, boundary: &str, source: &str) -> bool {
    let range = format!("{boundary}..{source}");
    let Ok(commits) = git(root, &["rev-list", "--reverse", &range]) else {
        return false;
    };
    String::from_utf8_lossy(&commits.stdout)
        .lines()
        .all(|commit| {
            let Ok(changes) = git(
                root,
                &["diff-tree", "--no-commit-id", "--name-status", "-r", commit],
            ) else {
                return false;
            };
            let rows = String::from_utf8_lossy(&changes.stdout)
                .lines()
                .map(str::to_string)
                .collect::<Vec<_>>();
            let material = rows.iter().any(|row| {
                row.split('\t').skip(1).any(|path| {
                    path.starts_with("crates/")
                        || path.starts_with("docs/")
                        || path == "Dockerfile"
                        || path == "docker-compose.yml"
                        || path.starts_with("config/")
                })
            });
            if !material {
                return true;
            }
            let deletion_count = rows.iter().filter(|row| row.starts_with('D')).count();
            let bounded =
                rows.len() <= 64 || (rows.len() <= 150 && deletion_count * 3 >= rows.len() * 2);
            let message = git(root, &["show", "-s", "--format=%B", commit])
                .ok()
                .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
                .unwrap_or_default();
            bounded
                && message.lines().any(|line| line.starts_with("Tested:"))
                && message.lines().any(|line| line.starts_with("Not-tested:"))
        })
}

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
