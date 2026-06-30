use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_readiness_story_req::{Requirement, STORY_REQUIREMENTS};
use crate::error::ToolResult;

pub(crate) fn story_report(
    root: &str,
    full: &Path,
    scale: &str,
    report: String,
) -> ToolResult<String> {
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
    if let Some(failure) = crate::artifact_story_manuscript::readiness_failure(root, &manuscript) {
        return Ok(failure);
    }
    let missing = story_missing(&files);
    let scale_missing = story_scale_missing(&files);
    if missing.is_empty() && scale_missing.is_empty() {
        let passed = crate::artifact_readiness::content_bearing(report).replace(
            "readiness=content-bearing",
            "readiness=story-semantic-content",
        );
        let atoms =
            crate::artifact_content_atom::status_lines_for_profile("story", "ready", 0, None);
        return Ok(format!("{passed}\nartifact_atom_profile=story\n{atoms}"));
    }
    let next_atom = missing.first().map(|label| format!("{label}.md"));
    let atom_count = missing.len() + scale_missing.len();
    let atoms = crate::artifact_content_atom::status_lines_for_profile(
        "story",
        "missing",
        atom_count,
        next_atom.as_ref(),
    );
    Ok(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-semantic-content\nartifact_atom_profile=story\n{atoms}\nfailed=1\nfailures:\n- story_semantic_missing: {}\n- story_scale_missing: {}\nnext_decision_required=true\ncandidate_action=artifact.next",
        join_or_none(&missing),
        join_or_none(&scale_missing)
    ))
}

fn story_missing(files: &[StoryFile]) -> Vec<&'static str> {
    STORY_REQUIREMENTS
        .iter()
        .filter(|requirement| !role_present(files, requirement))
        .map(|requirement| requirement.label)
        .collect()
}

fn story_scale_missing(files: &[StoryFile]) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if files.len() < STORY_REQUIREMENTS.len() {
        missing.push("profile-scale-content-groups");
    }
    if files
        .iter()
        .map(|file| word_count(&file.text))
        .sum::<usize>()
        < 400
    {
        missing.push("profile-scale-word-count");
    }
    missing
}

fn join_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
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
