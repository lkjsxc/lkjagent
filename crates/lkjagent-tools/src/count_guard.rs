use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

mod files;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountKind {
    File,
    Markdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountMode {
    Exact,
    Approximate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountGuard {
    pub kind: CountKind,
    pub target: usize,
    pub mode: CountMode,
}

impl CountGuard {
    pub fn as_state_value(self) -> String {
        match (self.kind, self.mode) {
            (CountKind::File, CountMode::Exact) => format!("file-count:{}", self.target),
            (CountKind::File, CountMode::Approximate) => {
                format!("file-count-about:{}", self.target)
            }
            (CountKind::Markdown, CountMode::Exact) => format!("markdown-count:{}", self.target),
            (CountKind::Markdown, CountMode::Approximate) => {
                format!("markdown-count-about:{}", self.target)
            }
        }
    }

    pub fn from_state_value(value: &str) -> Option<Self> {
        let (kind, mode, raw) = if let Some(raw) = value.strip_prefix("file-count-about:") {
            (CountKind::File, CountMode::Approximate, raw)
        } else if let Some(raw) = value.strip_prefix("markdown-count-about:") {
            (CountKind::Markdown, CountMode::Approximate, raw)
        } else if let Some(raw) = value.strip_prefix("file-count:") {
            (CountKind::File, CountMode::Exact, raw)
        } else if let Some(raw) = value.strip_prefix("markdown-count:") {
            (CountKind::Markdown, CountMode::Exact, raw)
        } else {
            return None;
        };
        raw.parse::<usize>()
            .ok()
            .map(|target| Self { kind, target, mode })
    }

    pub fn markdown_target(self) -> Option<usize> {
        (self.kind == CountKind::Markdown).then_some(self.target)
    }

    fn accepts(self, count: usize) -> bool {
        let (lower, upper) = self.bounds();
        (lower..=upper).contains(&count)
    }

    fn bounds(self) -> (usize, usize) {
        match self.mode {
            CountMode::Exact => (self.target, self.target),
            CountMode::Approximate => {
                let tolerance = self.target.div_ceil(10).max(1);
                (
                    self.target.saturating_sub(tolerance).max(1),
                    self.target.saturating_add(tolerance),
                )
            }
        }
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
    let mode = if approximate_signal(lower, content) && !exact_signal(lower, content) {
        CountMode::Approximate
    } else {
        CountMode::Exact
    };
    Some(CountGuard { kind, target, mode })
}

pub fn verify_count(workspace: &Path, guard: CountGuard) -> ToolResult<()> {
    let mut best: Option<(PathBuf, usize)> = None;
    for candidate in files::candidate_counts(workspace, guard.kind)? {
        if guard.accepts(candidate.count) {
            return Ok(());
        }
        if best.as_ref().is_none_or(|(_, current)| {
            distance(candidate.count, guard.target) < distance(*current, guard.target)
        }) {
            best = Some((candidate.path, candidate.count));
        }
    }
    Err(ToolError::invalid(files::report(workspace, guard, best)))
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

fn approximate_signal(lower: &str, content: &str) -> bool {
    lower.contains("about")
        || lower.contains("around")
        || lower.contains("roughly")
        || lower.contains("approximately")
        || lower.contains("approx ")
        || content.contains("ぐらい")
        || content.contains("くらい")
        || content.contains("程度")
        || content.contains("約")
}

fn exact_signal(lower: &str, content: &str) -> bool {
    lower.contains("exact")
        || lower.contains("exactly")
        || lower.contains("precisely")
        || content.contains("ちょうど")
        || content.contains("ぴったり")
        || content.contains("正確")
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

fn distance(count: usize, target: usize) -> usize {
    count.max(target).saturating_sub(count.min(target))
}
