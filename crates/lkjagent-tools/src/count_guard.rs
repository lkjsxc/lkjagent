use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountKind {
    File,
    Markdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountGuard {
    pub kind: CountKind,
    pub target: usize,
}

impl CountGuard {
    pub fn as_state_value(self) -> String {
        match self.kind {
            CountKind::File => format!("file-count:{}", self.target),
            CountKind::Markdown => format!("markdown-count:{}", self.target),
        }
    }

    pub fn from_state_value(value: &str) -> Option<Self> {
        let (kind, raw) = if let Some(raw) = value.strip_prefix("file-count:") {
            (CountKind::File, raw)
        } else if let Some(raw) = value.strip_prefix("markdown-count:") {
            (CountKind::Markdown, raw)
        } else {
            return None;
        };
        raw.parse::<usize>()
            .ok()
            .map(|target| Self { kind, target })
    }

    pub fn markdown_target(self) -> Option<usize> {
        (self.kind == CountKind::Markdown).then_some(self.target)
    }
}

pub fn count_target(lower: &str, content: &str) -> Option<CountGuard> {
    if !file_signal(lower, content) {
        return None;
    }
    let target = numbers(lower).into_iter().max()?;
    let kind = if markdown_signal(lower, content) {
        CountKind::Markdown
    } else {
        CountKind::File
    };
    Some(CountGuard { kind, target })
}

pub fn verify_count(workspace: &Path, guard: CountGuard) -> ToolResult<()> {
    let mut best: Option<(PathBuf, usize)> = None;
    for candidate in candidates(workspace)? {
        let count = count_files(&candidate, guard.kind)?;
        if count == guard.target {
            return Ok(());
        }
        if best.as_ref().is_none_or(|(_, current)| {
            distance(count, guard.target) < distance(*current, guard.target)
        }) {
            best = Some((candidate, count));
        }
    }
    Err(ToolError::invalid(report(workspace, guard, best)))
}

fn file_signal(lower: &str, content: &str) -> bool {
    lower.contains("file")
        || lower.contains(".md")
        || content.contains("ファイル")
        || content.contains("文書")
        || content.contains("ドキュメント")
}

fn markdown_signal(lower: &str, content: &str) -> bool {
    lower.contains("markdown")
        || lower.contains(".md")
        || content.contains("マークダウン")
        || content.contains("ドキュメント")
}

fn numbers(text: &str) -> Vec<usize> {
    let mut values = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else {
            save_number(&mut values, &mut current);
        }
    }
    save_number(&mut values, &mut current);
    values
}

fn save_number(values: &mut Vec<usize>, current: &mut String) {
    if current.is_empty() {
        return;
    }
    if let Ok(value) = current.parse() {
        values.push(value);
    }
    current.clear();
}

fn candidates(workspace: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut roots = Vec::new();
    collect(workspace, 0, &mut roots)?;
    Ok(roots)
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

fn distance(count: usize, target: usize) -> usize {
    count.max(target).saturating_sub(count.min(target))
}

fn report(workspace: &Path, guard: CountGuard, best: Option<(PathBuf, usize)>) -> String {
    let label = match guard.kind {
        CountKind::File => "files",
        CountKind::Markdown => "markdown files",
    };
    let base = format!(
        "{} count incomplete: need exactly {} {label}",
        label.trim_end_matches('s'),
        guard.target
    );
    match best {
        Some((path, count)) => format!(
            "{base}; next action should repair the count with one compact shell.run script; best={} {}={count}",
            relative(&path, workspace),
            label.replace(' ', "_")
        ),
        None => format!("{base}; no README.md candidate found; create a README-indexed root first"),
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
