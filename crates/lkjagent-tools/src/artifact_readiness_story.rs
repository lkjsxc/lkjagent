use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_readiness_story_req::{Requirement, STORY_REQUIREMENTS};
use crate::error::ToolResult;

pub(crate) fn story_report(root: &str, full: &Path, report: String) -> ToolResult<String> {
    let files = markdown_files(full)?;
    let missing = story_missing(&files);
    if missing.is_empty() {
        return Ok(crate::artifact_readiness::content_bearing(report).replace(
            "readiness=content-bearing",
            "readiness=story-semantic-content",
        ));
    }
    Ok(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-semantic-content\nfailed=1\nfailures:\n- story_semantic_missing: {}\nnext_decision_required=true\ncandidate_action=artifact.next",
        missing.join(",")
    ))
}

fn story_missing(files: &[StoryFile]) -> Vec<&'static str> {
    STORY_REQUIREMENTS
        .iter()
        .filter(|requirement| !role_present(files, requirement))
        .map(|requirement| requirement.label)
        .collect()
}

fn role_present(files: &[StoryFile], requirement: &Requirement) -> bool {
    files.iter().any(|file| {
        role_candidate(file, requirement)
            && word_count(&file.text) >= 25
            && signal_count(&file.lower, requirement.signals) >= 2
    })
}

fn role_candidate(file: &StoryFile, requirement: &Requirement) -> bool {
    requirement
        .paths
        .iter()
        .any(|path| file.relative.contains(path))
        || heading_mentions(&file.lower, requirement.label)
}

fn heading_mentions(text: &str, label: &str) -> bool {
    let normalized = label.replace('-', " ");
    text.lines()
        .filter(|line| line.trim_start().starts_with('#'))
        .any(|line| line.contains(&normalized))
}

fn signal_count(text: &str, signals: &[&'static str]) -> usize {
    signals
        .iter()
        .filter(|signal| text.contains(**signal))
        .count()
}

fn markdown_files(root: &Path) -> ToolResult<Vec<StoryFile>> {
    let mut paths = Vec::new();
    collect_markdown(root, root, &mut paths)?;
    let mut files = Vec::new();
    for (relative, path) in paths {
        let text = fs::read_to_string(path)?;
        files.push(StoryFile {
            relative,
            lower: text.to_ascii_lowercase(),
            text,
        });
    }
    Ok(files)
}

fn collect_markdown(
    root: &Path,
    current: &Path,
    files: &mut Vec<(String, PathBuf)>,
) -> ToolResult<()> {
    if current.is_dir() {
        for entry in fs::read_dir(current)? {
            collect_markdown(root, &entry?.path(), files)?;
        }
    } else if current.extension().is_some_and(|ext| ext == "md") && !is_readme(current) {
        files.push((relative_path(root, current), current.to_path_buf()));
    }
    Ok(())
}

fn is_readme(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "README.md")
}

fn relative_path(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => path,
    }
    .to_string_lossy()
    .to_ascii_lowercase()
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}

struct StoryFile {
    relative: String,
    lower: String,
    text: String,
}
