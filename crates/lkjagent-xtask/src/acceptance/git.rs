use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub fn validate_plan_inputs(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    for path in [
        "evaluation/workgraph.tsv",
        "evaluation/acceptance.tsv",
        "evaluation/experiment-plan.tsv",
    ] {
        match git(root, &["ls-files", "--error-unmatch", "--", path]) {
            Ok(output) if output.status.success() => {}
            _ => errors.push(format!("{path}: plan is not tracked")),
        }
        if dirty(root, path) {
            errors.push(format!("{path}: editable plan input is forbidden"));
        }
    }
    errors
}

pub fn evidence_files(
    root: &Path,
    source: &str,
    argument: &Path,
) -> Result<Vec<PathBuf>, Vec<String>> {
    let relative = match safe_relative(root, argument) {
        Ok(path) => path,
        Err(error) => return Err(vec![error]),
    };
    let expected = Path::new("evaluation").join("evidence").join(source);
    if relative != expected && !relative.starts_with(&expected) {
        return Err(vec![
            "evidence path must be under evaluation/evidence/SOURCE".to_string(),
        ]);
    }
    let absolute = root.join(&relative);
    if path_has_symlink(root, &relative) {
        return Err(vec!["evidence path traverses a symlink".to_string()]);
    }
    if !absolute.is_dir() {
        return Err(vec![
            "evidence path is missing or not a directory".to_string()
        ]);
    }
    let mut files = Vec::new();
    let mut errors = Vec::new();
    collect(&absolute, &mut files, &mut errors);
    if files.is_empty() {
        errors.push("evidence path contains no attachments".to_string());
    }
    let tracked = match tracked(root, &relative) {
        Ok(value) => value,
        Err(error) => {
            errors.push(error);
            HashSet::new()
        }
    };
    for file in &files {
        match file.strip_prefix(root) {
            Ok(relative_file) => {
                let rel = relative_file.to_string_lossy().replace('\\', "/");
                if !tracked.contains(&rel) {
                    errors.push(format!("{rel}: evidence attachment is untracked"));
                }
            }
            Err(_) => errors.push("evidence attachment escaped repository".to_string()),
        }
    }
    if dirty(root, &relative.to_string_lossy()) {
        errors.push("evidence attachments are editable or untracked".to_string());
    }
    if errors.is_empty() {
        Ok(files)
    } else {
        Err(errors)
    }
}

fn safe_relative(root: &Path, argument: &Path) -> Result<PathBuf, String> {
    let path = if argument.is_absolute() {
        argument
            .strip_prefix(root)
            .map_err(|_| "evidence path is outside repository")?
            .to_path_buf()
    } else {
        argument.to_path_buf()
    };
    if path.components().any(|part| {
        matches!(
            part,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err("evidence path contains unsafe components".to_string());
    }
    Ok(path
        .components()
        .filter(|part| !matches!(part, Component::CurDir))
        .collect())
}

fn path_has_symlink(root: &Path, relative: &Path) -> bool {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return true;
        }
    }
    false
}

fn collect(path: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
    let entries = match fs::read_dir(path) {
        Ok(value) => value,
        Err(error) => {
            errors.push(format!(
                "{}: cannot read evidence: {error}",
                path.to_string_lossy()
            ));
            return;
        }
    };
    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(value) => value,
            Err(error) => {
                errors.push(format!(
                    "{}: cannot inspect evidence: {error}",
                    entry.path().to_string_lossy()
                ));
                continue;
            }
        };
        if file_type.is_symlink() {
            errors.push(format!(
                "{}: evidence symlink is forbidden",
                entry.path().to_string_lossy()
            ));
        } else if file_type.is_dir() {
            collect(&entry.path(), files, errors);
        } else if file_type.is_file() {
            files.push(entry.path());
        } else {
            errors.push(format!(
                "{}: special evidence attachment is forbidden",
                entry.path().to_string_lossy()
            ));
        }
    }
}

fn tracked(root: &Path, path: &Path) -> Result<HashSet<String>, String> {
    let value = path.to_string_lossy();
    let output = git(root, &["ls-files", "-z", "--", &value])?;
    Ok(output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|item| !item.is_empty())
        .map(|item| String::from_utf8_lossy(item).into_owned())
        .collect())
}

fn dirty(root: &Path, path: &str) -> bool {
    git(
        root,
        &["status", "--porcelain", "--untracked-files=all", "--", path],
    )
    .map_or(true, |output| !output.stdout.is_empty())
}

fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))
}
