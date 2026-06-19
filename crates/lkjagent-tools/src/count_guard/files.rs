use std::fs;
use std::path::{Path, PathBuf};

use crate::count_guard::CountKind;
use crate::count_guard::{CountGuard, CountMode};
use crate::error::ToolResult;

pub struct CandidateCount {
    pub path: PathBuf,
    pub count: usize,
}

pub fn candidate_counts(workspace: &Path, kind: CountKind) -> ToolResult<Vec<CandidateCount>> {
    let mut roots = Vec::new();
    collect(workspace, 0, &mut roots)?;
    if !roots.is_empty() {
        return counts_for_roots(roots, kind);
    }
    plain_output_counts(workspace, kind)
}

fn counts_for_roots(roots: Vec<PathBuf>, kind: CountKind) -> ToolResult<Vec<CandidateCount>> {
    let mut counts = Vec::new();
    for path in roots {
        counts.push(CandidateCount {
            count: count_files(&path, kind)?,
            path,
        });
    }
    Ok(counts)
}

pub fn relative(path: &Path, workspace: &Path) -> String {
    path.strip_prefix(workspace)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

pub fn report(workspace: &Path, guard: CountGuard, best: Option<(PathBuf, usize)>) -> String {
    let label = match guard.kind {
        CountKind::File => "files",
        CountKind::Markdown => "markdown files",
    };
    let base = requirement(guard, label);
    match best {
        Some((path, count)) => format!(
            "{base}; next action should repair the count with one compact shell.run script; best={} {}={count}",
            relative(&path, workspace),
            label.replace(' ', "_")
        ),
        None => format!("{base}; no README.md candidate found; create a README-indexed root first"),
    }
}

fn requirement(guard: CountGuard, label: &str) -> String {
    let noun = label.trim_end_matches('s');
    match guard.mode {
        CountMode::Exact => format!(
            "{noun} count incomplete: need exactly {} {label}",
            guard.target
        ),
        CountMode::Approximate => {
            let (lower, upper) = guard.bounds();
            format!(
                "{noun} count incomplete: need about {} {label} ({lower}-{upper})",
                guard.target
            )
        }
    }
}

fn collect(path: &Path, depth: usize, roots: &mut Vec<PathBuf>) -> ToolResult<()> {
    if depth > 3 || hidden(path) {
        return Ok(());
    }
    if path.join("README.md").exists() {
        roots.push(path.to_path_buf());
    }
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            collect(&child, depth.saturating_add(1), roots)?;
        }
    }
    Ok(())
}

fn plain_output_counts(workspace: &Path, kind: CountKind) -> ToolResult<Vec<CandidateCount>> {
    let mut roots = Vec::new();
    let mut visible_files = 0_usize;
    for entry in fs::read_dir(workspace)? {
        let child = entry?.path();
        if hidden(&child) {
            continue;
        }
        if child.is_dir() {
            roots.push(child);
        } else {
            visible_files = visible_files.saturating_add(1);
        }
    }
    if roots.is_empty() || visible_files > 0 {
        return Ok(Vec::new());
    }
    if roots.len() == 1 {
        counts_for_roots(roots, kind)
    } else {
        let mut count = 0_usize;
        for root in roots {
            count = count.saturating_add(count_files(&root, kind)?);
        }
        Ok(vec![CandidateCount {
            path: workspace.to_path_buf(),
            count,
        }])
    }
}

fn count_files(path: &Path, kind: CountKind) -> ToolResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() && !hidden(&child) {
            count = count.saturating_add(count_files(&child, kind)?);
        } else if counted_file(&child, kind) {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}

fn counted_file(path: &Path, kind: CountKind) -> bool {
    match kind {
        CountKind::File => path.is_file(),
        CountKind::Markdown => path.extension().is_some_and(|extension| extension == "md"),
    }
}

fn hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}
