use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

const MIN_DIRECTORIES: usize = 6;
const MIN_MARKDOWN_FILES: usize = 12;
const MIN_DEPTH: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Evidence {
    directories: usize,
    markdown_files: usize,
    max_depth: usize,
    missing_readmes: usize,
    readmes_without_toc: usize,
}

impl Evidence {
    fn accepts(self) -> bool {
        self.directories >= MIN_DIRECTORIES
            && self.markdown_files >= MIN_MARKDOWN_FILES
            && self.max_depth >= MIN_DEPTH
            && self.missing_readmes == 0
            && self.readmes_without_toc == 0
    }

    fn score(self) -> usize {
        self.directories
            .saturating_add(self.markdown_files)
            .saturating_add(self.max_depth)
            .saturating_sub(self.missing_readmes)
            .saturating_sub(self.readmes_without_toc)
    }
}

pub fn verify_recursive_tree(workspace: &Path) -> ToolResult<()> {
    let mut best: Option<(PathBuf, Evidence)> = None;
    for candidate in candidate_roots(workspace)? {
        let evidence = analyze(&candidate, 0)?;
        if evidence.accepts() {
            return Ok(());
        }
        if best
            .as_ref()
            .is_none_or(|(_, current)| evidence.score() > current.score())
        {
            best = Some((candidate, evidence));
        }
    }
    Err(ToolError::invalid(report_failure(workspace, best)))
}

fn candidate_roots(workspace: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut roots = Vec::new();
    collect_candidates(workspace, 0, &mut roots)?;
    Ok(roots)
}

fn collect_candidates(path: &Path, depth: usize, roots: &mut Vec<PathBuf>) -> ToolResult<()> {
    if depth > 2 || hidden(path) {
        return Ok(());
    }
    if path.join("README.md").exists() {
        roots.push(path.to_path_buf());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        if child.is_dir() {
            collect_candidates(&child, depth.saturating_add(1), roots)?;
        }
    }
    Ok(())
}

fn analyze(path: &Path, depth: usize) -> ToolResult<Evidence> {
    let mut evidence = Evidence {
        directories: 1,
        markdown_files: 0,
        max_depth: depth,
        missing_readmes: usize::from(!path.join("README.md").exists()),
        readmes_without_toc: usize::from(readme_without_toc(path)?),
    };
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        if child.is_dir() && !hidden(&child) {
            let child_evidence = analyze(&child, depth.saturating_add(1))?;
            evidence = merge(evidence, child_evidence);
        } else if child.extension().is_some_and(|extension| extension == "md") {
            evidence.markdown_files = evidence.markdown_files.saturating_add(1);
        }
    }
    Ok(evidence)
}

fn readme_without_toc(path: &Path) -> ToolResult<bool> {
    let readme = path.join("README.md");
    if !readme.exists() {
        return Ok(false);
    }
    Ok(!fs::read_to_string(readme)?.contains("## Table of Contents"))
}

fn merge(left: Evidence, right: Evidence) -> Evidence {
    Evidence {
        directories: left.directories.saturating_add(right.directories),
        markdown_files: left.markdown_files.saturating_add(right.markdown_files),
        max_depth: left.max_depth.max(right.max_depth),
        missing_readmes: left.missing_readmes.saturating_add(right.missing_readmes),
        readmes_without_toc: left
            .readmes_without_toc
            .saturating_add(right.readmes_without_toc),
    }
}

fn hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

fn report_failure(workspace: &Path, best: Option<(PathBuf, Evidence)>) -> String {
    let minimum = format!(
        "recursive structure incomplete: need README-indexed tree with at least {MIN_DIRECTORIES} directories, {MIN_MARKDOWN_FILES} markdown files, depth {MIN_DEPTH}, and no directory without README.md"
    );
    let Some((path, evidence)) = best else {
        return format!("{minimum}; no README.md candidate found");
    };
    let shown = path
        .strip_prefix(workspace)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .unwrap_or(path.as_path())
        .to_string_lossy();
    format!(
        "{minimum}; best={shown} directories={} markdown_files={} depth={} missing_readmes={} readmes_without_toc={}",
        evidence.directories,
        evidence.markdown_files,
        evidence.max_depth,
        evidence.missing_readmes,
        evidence.readmes_without_toc
    )
}
