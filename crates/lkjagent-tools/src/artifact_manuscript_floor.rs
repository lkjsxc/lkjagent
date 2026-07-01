use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_story_text::{numbers_before, path_char};
use crate::error::ToolResult;

pub(crate) fn chapter_floor(root: &Path) -> ToolResult<usize> {
    let text = markdown_text(root)?.to_ascii_lowercase();
    let target = numbers_before(&text, "word").into_iter().max();
    let total_floor = target
        .map(|words| words.saturating_mul(85) / 100)
        .unwrap_or(600);
    let count = numbers_before(&text, "chapter")
        .into_iter()
        .max()
        .or_else(|| manuscript_path_count(&text))
        .unwrap_or(1);
    Ok(total_floor.div_ceil(count.max(1)))
}

fn markdown_text(root: &Path) -> ToolResult<String> {
    let mut text = String::new();
    for path in markdown_files(root)? {
        text.push('\n');
        text.push_str(&fs::read_to_string(path)?);
    }
    Ok(text)
}

fn markdown_files(root: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_markdown(root, &mut out)?;
    Ok(out)
}

fn collect_markdown(current: &Path, out: &mut Vec<PathBuf>) -> ToolResult<()> {
    if current.is_dir() {
        for entry in fs::read_dir(current)? {
            collect_markdown(&entry?.path(), out)?;
        }
    } else if current.extension().is_some_and(|ext| ext == "md") && !is_readme(current) {
        out.push(current.to_path_buf());
    }
    Ok(())
}

fn manuscript_path_count(text: &str) -> Option<usize> {
    let mut paths = text
        .split(|ch: char| !path_char(ch))
        .filter(|token| token.contains("manuscript/"))
        .filter(|token| !token.contains("manuscript/scenes/"))
        .filter(|token| token.trim_matches('.').ends_with(".md"))
        .map(|token| token.trim_matches('.').to_string())
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    (!paths.is_empty()).then_some(paths.len())
}

fn is_readme(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some("README.md")
}
