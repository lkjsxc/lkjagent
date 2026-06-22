use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ToolResult;

use super::mock_content::mock_sibling_checks;

pub fn content_checks(root: &Path, failures: &mut Vec<String>) -> ToolResult<()> {
    let Some(kind) = content_kind(root)? else {
        return Ok(());
    };
    let files = markdown_leaves(root)?;
    for file in &files {
        let text = fs::read_to_string(file)?;
        if let Some(failure) = content_failure(&kind, root, file, &text) {
            failures.push(failure);
        }
    }
    if matches!(kind, ContentKind::Documentation) {
        mock_sibling_checks(root, &files, failures)?;
    }
    Ok(())
}

pub fn weak_content_paths(root: &Path) -> ToolResult<Vec<String>> {
    let Some(kind) = content_kind(root)? else {
        return Ok(Vec::new());
    };
    let mut paths = Vec::new();
    for file in markdown_leaves(root)? {
        let text = fs::read_to_string(&file)?;
        if content_failure(&kind, root, &file, &text).is_some() {
            paths.push(rel(root, &file));
        }
    }
    Ok(paths)
}

fn content_kind(root: &Path) -> ToolResult<Option<ContentKind>> {
    let catalog = root.join("catalog.toml");
    if !catalog.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(catalog)?;
    if text.contains("Cookbook") || text.contains("kind = \"cookbook\"") {
        return Ok(Some(ContentKind::Cookbook));
    }
    if text.contains("NarrativeManuscript") || text.contains("kind = \"story\"") {
        return Ok(Some(ContentKind::Story));
    }
    Ok(Some(ContentKind::Documentation))
}

fn markdown_leaves(root: &Path) -> ToolResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_markdown(root, &mut files)?;
    files.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name != "README.md")
    });
    files.sort();
    Ok(files)
}

fn collect_markdown(dir: &Path, files: &mut Vec<PathBuf>) -> ToolResult<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    Ok(())
}

fn scaffold_only(text: &str) -> bool {
    text.contains("This file records the")
        && text.contains("generated documentation tree")
        && text.contains("\nscaffolded\n")
}

fn content_failure(kind: &ContentKind, root: &Path, file: &Path, text: &str) -> Option<String> {
    let relative = rel(root, file);
    if scaffold_only(text) {
        return Some(format!("scaffold_only_content: {relative}"));
    }
    if let Some(phrase) = crate::placeholder::detect(text) {
        return Some(format!("placeholder_content: {relative} phrase={phrase}"));
    }
    match kind {
        ContentKind::Cookbook => cookbook_failure(&relative, text),
        ContentKind::Story => story_failure(&relative, text),
        ContentKind::Documentation => None,
    }
}

fn cookbook_failure(relative: &str, text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let mut missing = Vec::new();
    if word_count(text) < 80 {
        missing.push("word-count");
    }
    if relative.starts_with("recipes/") {
        require_any(
            &lower,
            &["ingredient", "flour", "water", "salt"],
            "ingredients",
            &mut missing,
        );
        require_any(
            &lower,
            &["method", "steps", "mix", "bake"],
            "method",
            &mut missing,
        );
        require_any(
            &lower,
            &["timing", "time", "yield"],
            "timing-or-yield",
            &mut missing,
        );
        require_any(
            &lower,
            &["notes", "troubleshooting", "avoid"],
            "notes",
            &mut missing,
        );
    } else if relative.starts_with("reference/") {
        require_any(
            &lower,
            &["lookup", "table", "range"],
            "lookup-content",
            &mut missing,
        );
    } else {
        require_any(&lower, &["procedure", "steps"], "procedure", &mut missing);
        require_any(&lower, &["signals", "look for"], "signals", &mut missing);
        require_any(&lower, &["mistake", "avoid"], "mistakes", &mut missing);
        require_any(
            &lower,
            &["correct", "fix"],
            "corrective-action",
            &mut missing,
        );
    }
    if missing.is_empty() {
        None
    } else {
        Some(format!(
            "weak_cookbook_content: {relative} missing={}",
            missing.join(",")
        ))
    }
}

fn story_failure(relative: &str, text: &str) -> Option<String> {
    if word_count(text) >= 40 && text.contains("##") {
        return None;
    }
    Some(format!(
        "weak_story_content: {relative} missing=scene-or-reference-content"
    ))
}

fn require_any(text: &str, needles: &[&str], label: &'static str, missing: &mut Vec<&'static str>) {
    if !needles.iter().any(|needle| text.contains(needle)) {
        missing.push(label);
    }
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}

enum ContentKind {
    Cookbook,
    Story,
    Documentation,
}

fn rel(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => path,
    }
    .to_string_lossy()
    .to_string()
}
