use std::path::Path;
use std::process::Command;

use crate::docs::check_docs;
use crate::docs_authority_contract;
use crate::docs_authority_product;
use crate::facts::collect_files;
use crate::gate::check_lines;
use crate::model::RepoFile;

const PRODUCT_BASE: &str = "ae5ff551457adce869dee6159200c85a63aab3de";
const PRODUCT_PATHS: &[&str] = &[
    "crates/lkjagent-app",
    "crates/lkjagent-core",
    "crates/lkjagent-effects",
    "crates/lkjagent-llm",
    "crates/lkjagent-store",
    "evaluation",
    "Cargo.toml",
    "Cargo.lock",
    "Dockerfile",
    "docker-compose.yml",
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let files = collect_files(root).map_err(|error| vec![error])?;
    let mut failures = check_contract(&files);
    if root.join(".git").is_dir() {
        match changed_product_paths(root) {
            Ok(paths) => failures.extend(check_changed_paths(&paths)),
            Err(error) => failures.push(error),
        }
    } else {
        failures.extend(docs_authority_product::check(root));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

pub fn check_contract(files: &[RepoFile]) -> Vec<String> {
    let mut failures = check_docs(files)
        .into_iter()
        .chain(check_lines(files))
        .map(|violation| violation.message())
        .collect::<Vec<_>>();
    docs_authority_contract::check(files, &mut failures);
    failures
}

pub fn check_changed_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|path| !path.trim().is_empty())
        .map(|path| {
            format!("docs-authority must be behavior-identical; product path changed: {path}")
        })
        .collect()
}

fn changed_product_paths(root: &Path) -> Result<Vec<String>, String> {
    let mut changed = git_paths(root, &["diff", "--name-only", PRODUCT_BASE, "--"])?;
    changed.extend(git_paths(
        root,
        &["ls-files", "--others", "--exclude-standard", "--"],
    )?);
    changed.sort();
    changed.dedup();
    Ok(changed)
}

fn git_paths(root: &Path, prefix: &[&str]) -> Result<Vec<String>, String> {
    let mut args = prefix.to_vec();
    args.extend(PRODUCT_PATHS);
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("git source-drift check could not start: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git source-drift check failed: {}",
            stderr.lines().last().unwrap_or("no stderr")
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect())
}
