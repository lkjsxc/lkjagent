use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::docs::check_docs;
use crate::docs_authority_contract;
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
        failures.extend(product_check(root));
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

const EXPECTED_PRODUCT_FINGERPRINT: &str = "fnv1a64:ac36561d94dc9d06";
const FINGERPRINT_DIRS: &[&str] = &[
    "crates/lkjagent-app",
    "crates/lkjagent-core",
    "crates/lkjagent-effects",
    "crates/lkjagent-llm",
    "crates/lkjagent-store",
    "evaluation",
];
const FINGERPRINT_FILES: &[&str] = &["Cargo.toml", "Dockerfile", "docker-compose.yml"];

fn product_check(root: &Path) -> Vec<String> {
    match fingerprint(root) {
        Ok(actual) if actual == EXPECTED_PRODUCT_FINGERPRINT => Vec::new(),
        Ok(actual) => vec![format!("docs-authority must be behavior-identical; product fingerprint is {actual}, expected {EXPECTED_PRODUCT_FINGERPRINT}")],
        Err(error) => vec![error],
    }
}

fn fingerprint(root: &Path) -> Result<String, String> {
    let mut paths = Vec::new();
    for relative in FINGERPRINT_DIRS {
        collect(root, &root.join(relative), &mut paths)?;
    }
    for relative in FINGERPRINT_FILES {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("product fingerprint input is missing: {relative}"));
        }
        paths.push(path);
    }
    paths.sort();
    let mut hash = 0xcbf29ce484222325_u64;
    for path in paths {
        let relative = path
            .strip_prefix(root)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        update(&mut hash, relative.to_string_lossy().as_bytes());
        update(&mut hash, &[0]);
        update(
            &mut hash,
            &fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?,
        );
        update(&mut hash, &[0xff]);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

fn collect(root: &Path, path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!(
            "product fingerprint directory is missing: {}",
            path.strip_prefix(root).unwrap_or(path).display()
        ));
    }
    for entry in fs::read_dir(path).map_err(|error| format!("{}: {error}", path.display()))? {
        let child = entry.map_err(|error| error.to_string())?.path();
        if child.is_dir() {
            collect(root, &child, paths)?;
        } else if child.is_file() {
            paths.push(child);
        }
    }
    Ok(())
}

fn update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
