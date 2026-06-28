use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;
use crate::structure::{
    hidden, readme_without_toc, Evidence, MIN_DEPTH, MIN_DIRECTORIES, MIN_MARKDOWN_FILES,
};
use crate::structure_quality::is_weak_markdown_file;

pub(crate) fn report_failure(
    workspace: &Path,
    best: Option<(PathBuf, Evidence)>,
) -> ToolResult<String> {
    let minimum = format!(
        "recursive structure incomplete: need README-indexed tree with at least {MIN_DIRECTORIES} directories, {MIN_MARKDOWN_FILES} markdown files, depth {MIN_DEPTH}, no missing README.md, every README.md containing ## Table of Contents, and no scaffold-only Markdown"
    );
    let Some((path, evidence)) = best else {
        return Ok(format!("{minimum}; no README.md candidate found"));
    };
    let shown = relative(&path, workspace);
    let missing = sample_missing_readmes(&path, workspace)?;
    let without_toc = sample_readmes_without_toc(&path, workspace)?;
    let weak = sample_weak_markdown(&path, workspace)?;
    Ok(format!(
        "{minimum}; next action should use doc.audit, artifact.next, fs.mkdir, or fs.batch_write to repair README-indexed structure; best={shown} directories={} markdown_files={} depth={} missing_readmes={} readmes_without_toc={} weak_markdown_files={} missing_readme_paths={} readmes_without_toc_paths={} weak_markdown_paths={}",
        evidence.directories,
        evidence.markdown_files,
        evidence.max_depth,
        evidence.missing_readmes,
        evidence.readmes_without_toc,
        evidence.weak_markdown_files,
        sample_list(&missing),
        sample_list(&without_toc),
        sample_list(&weak)
    ))
}

fn sample_missing_readmes(root: &Path, workspace: &Path) -> ToolResult<Vec<String>> {
    let mut samples = Vec::new();
    collect_samples(root, workspace, &mut samples, |path| {
        Ok(path.is_dir() && !path.join("README.md").exists())
    })?;
    Ok(samples)
}

fn sample_readmes_without_toc(root: &Path, workspace: &Path) -> ToolResult<Vec<String>> {
    let mut samples = Vec::new();
    collect_samples(root, workspace, &mut samples, readme_without_toc)?;
    Ok(samples)
}

fn sample_weak_markdown(root: &Path, workspace: &Path) -> ToolResult<Vec<String>> {
    let mut samples = Vec::new();
    collect_samples(root, workspace, &mut samples, is_weak_markdown_file)?;
    Ok(samples)
}

fn collect_samples<F>(
    path: &Path,
    workspace: &Path,
    samples: &mut Vec<String>,
    matches: F,
) -> ToolResult<()>
where
    F: Fn(&Path) -> ToolResult<bool> + Copy,
{
    if samples.len() >= 6 || hidden(path) {
        return Ok(());
    }
    if matches(path)? {
        samples.push(relative(path, workspace));
    }
    if !path.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        collect_samples(&entry?.path(), workspace, samples, matches)?;
    }
    Ok(())
}

fn sample_list(samples: &[String]) -> String {
    if samples.is_empty() {
        "none".to_string()
    } else {
        samples.join(",")
    }
}

fn relative(path: &Path, workspace: &Path) -> String {
    path.strip_prefix(workspace)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}
