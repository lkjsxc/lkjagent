use std::path::Path;
use std::process::{Command, Output};

pub fn validate(root: &Path, source: &str) -> Result<(), String> {
    validate_shape(source)?;
    let revision = format!("{source}^{{commit}}");
    let resolved = git(root, &["rev-parse", "--verify", &revision])?;
    if !resolved.status.success() || String::from_utf8_lossy(&resolved.stdout).trim() != source {
        return Err("source is not an exact reachable commit".to_string());
    }
    let ancestor = git(root, &["merge-base", "--is-ancestor", source, "HEAD"])?;
    if !ancestor.status.success() {
        return Err("source is not an ancestor of Git HEAD".to_string());
    }
    validate_later_paths(root, source)
}

fn validate_shape(source: &str) -> Result<(), String> {
    if source.len() != 40
        || !source
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("source must be a full lowercase Git commit ID".to_string());
    }
    Ok(())
}

fn validate_later_paths(root: &Path, source: &str) -> Result<(), String> {
    let range = format!("{source}..HEAD");
    let output = git(root, &["diff", "--name-only", "-z", &range, "--"])?;
    if !output.status.success() {
        return Err("cannot compare source with Git HEAD".to_string());
    }
    let allowed = format!("evaluation/evidence/{source}/");
    let changed = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path))
        .find(|path| !path.starts_with(&allowed));
    match changed {
        Some(path) => Err(format!(
            "Git HEAD changed outside source evidence after freeze: {path}"
        )),
        None => Ok(()),
    }
}

fn git(root: &Path, args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))
}
