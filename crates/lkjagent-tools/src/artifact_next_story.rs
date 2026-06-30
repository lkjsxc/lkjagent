use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_readiness_story_req::{Requirement, STORY_REQUIREMENTS};
use crate::error::ToolResult;

pub(crate) struct StoryNextContract {
    pub selected: Vec<String>,
    pub valid_example: String,
    pub response: String,
}

pub(crate) fn story_contract_if_missing(
    root: &str,
    kind: &str,
    scale: &str,
    full: &Path,
) -> ToolResult<Option<StoryNextContract>> {
    if !story_kind(kind) && !story_root(root) {
        return Ok(None);
    }
    let files = markdown_files(full)?;
    let manuscript_files = files
        .iter()
        .map(|file| crate::artifact_story_manuscript::ManuscriptFile {
            relative: &file.relative,
            lower: &file.lower,
            text: &file.text,
        })
        .collect::<Vec<_>>();
    let manuscript = crate::artifact_story_manuscript::facts(root, scale, &manuscript_files);
    let selected = if manuscript.active && !manuscript.missing_paths.is_empty() {
        manuscript.missing_paths.into_iter().take(1).collect()
    } else {
        selected_paths(&files)
    };
    if selected.is_empty() {
        return Ok(None);
    }
    let valid_example = if selected.iter().any(|path| path.starts_with("manuscript/")) {
        crate::artifact_next_example::manuscript_batch_write_contract(root, &selected)
    } else {
        crate::artifact_next_example::batch_write_contract(root, "story", &selected)
    };
    let response =
        crate::artifact_next_response::batch_response(root, "story", &selected, &valid_example);
    Ok(Some(StoryNextContract {
        selected,
        valid_example,
        response,
    }))
}

fn selected_paths(files: &[StoryFile]) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for requirement in STORY_REQUIREMENTS {
        if !role_present(files, requirement) {
            paths.insert(label_path(requirement.label));
        }
    }
    if files.len() < STORY_REQUIREMENTS.len() {
        for requirement in STORY_REQUIREMENTS {
            let path = label_path(requirement.label);
            if !path_present(files, &path) {
                paths.insert(path);
            }
        }
    }
    if total_words(files) < 400 && paths.is_empty() {
        paths.insert("checks/scale-readiness.md".to_string());
    }
    paths.into_iter().take(5).collect()
}

fn role_present(files: &[StoryFile], requirement: &Requirement) -> bool {
    files.iter().any(|file| {
        role_candidate(file, requirement)
            && word_count(&file.text) >= 25
            && (signal_count(&file.lower, requirement.signals) >= 2
                || file.relative.contains(requirement.label))
    })
}

fn role_candidate(file: &StoryFile, requirement: &Requirement) -> bool {
    requirement
        .paths
        .iter()
        .any(|path| file.relative.contains(path))
        || file.relative.contains(requirement.label)
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

fn path_present(files: &[StoryFile], path: &str) -> bool {
    files.iter().any(|file| file.relative == path)
}

fn label_path(label: &str) -> String {
    format!("{label}.md")
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

fn total_words(files: &[StoryFile]) -> usize {
    files.iter().map(|file| word_count(&file.text)).sum()
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}

fn story_kind(kind: &str) -> bool {
    matches!(
        kind.to_ascii_lowercase().as_str(),
        "story" | "novel" | "manuscript"
    )
}

fn story_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

struct StoryFile {
    relative: String,
    lower: String,
    text: String,
}
