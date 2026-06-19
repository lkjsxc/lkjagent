use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

mod files;
mod mode;
mod parse;

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
    parse::count_target(lower, content)
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

fn distance(count: usize, target: usize) -> usize {
    count.max(target).saturating_sub(count.min(target))
}
