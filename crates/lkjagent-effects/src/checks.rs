use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::checks::{evaluate, CommandFact, FileFact};
use lkjagent_core::model::{CheckResult, CheckSpec};

use crate::error::EffectResult;
use crate::shell;
use crate::workspace::resolve;

pub fn run_check(root: &Path, spec: &CheckSpec) -> EffectResult<CheckResult> {
    let files = gather_files(root, spec)?;
    let commands = match spec {
        CheckSpec::Command { cmd } => vec![CommandFact {
            cmd: cmd.clone(),
            success: shell::run(root, cmd, shell::SHELL_TIMEOUT_SECONDS)?.success(),
        }],
        _ => Vec::new(),
    };
    Ok(evaluate(spec, &files, &commands))
}

pub fn gather_files(root: &Path, spec: &CheckSpec) -> EffectResult<Vec<FileFact>> {
    match spec {
        CheckSpec::FileExists { path }
        | CheckSpec::MinWords { path, .. }
        | CheckSpec::MaxLines { path, .. }
        | CheckSpec::Contains { path, .. }
        | CheckSpec::Absent { path, .. }
        | CheckSpec::Judged { path, .. } => read_one(root, path),
        CheckSpec::MinWordsTotal { glob, .. } | CheckSpec::FileCount { glob, .. } => {
            read_glob(root, glob)
        }
        CheckSpec::ReadmeCoverage { root: subroot } | CheckSpec::LinksResolve { root: subroot } => {
            read_tree(root, subroot)
        }
        CheckSpec::Command { .. } => Ok(Vec::new()),
    }
}

fn read_one(root: &Path, path: &str) -> EffectResult<Vec<FileFact>> {
    let full = resolve(root, path)?;
    if !full.exists() {
        return Ok(Vec::new());
    }
    Ok(vec![FileFact {
        path: path.to_string(),
        content: fs::read_to_string(full).unwrap_or_default(),
    }])
}

fn read_glob(root: &Path, glob: &str) -> EffectResult<Vec<FileFact>> {
    let mut files = Vec::new();
    collect(root, root, &mut files)?;
    let facts = files
        .into_iter()
        .filter_map(|path| fact_for(root, &path))
        .filter(|fact| glob_match(glob, &fact.path))
        .collect();
    Ok(facts)
}

fn read_tree(root: &Path, subroot: &str) -> EffectResult<Vec<FileFact>> {
    let base = resolve(root, subroot)?;
    if !base.exists() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    collect(root, &base, &mut paths)?;
    Ok(paths
        .into_iter()
        .filter_map(|path| fact_for(root, &path))
        .collect())
}

fn collect(_root: &Path, path: &Path, files: &mut Vec<PathBuf>) -> EffectResult<()> {
    let meta = fs::metadata(path)?;
    if meta.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }
    for entry in fs::read_dir(path)?.filter_map(Result::ok) {
        collect(_root, &entry.path(), files)?;
    }
    Ok(())
}

fn fact_for(root: &Path, path: &Path) -> Option<FileFact> {
    let rel = path.strip_prefix(root).ok()?.to_string_lossy().to_string();
    let content = fs::read_to_string(path).unwrap_or_default();
    Some(FileFact { path: rel, content })
}

fn glob_match(glob: &str, path: &str) -> bool {
    if let Some((prefix, suffix)) = glob.split_once('*') {
        path.starts_with(prefix) && path.ends_with(suffix)
    } else {
        path == glob
    }
}
