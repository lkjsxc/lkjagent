use crate::artifact_story_manuscript::ManuscriptFile;
use crate::artifact_story_text::{path_char, prose_words};

pub(crate) fn planned_paths(
    root: &str,
    files: &[ManuscriptFile<'_>],
    text: &str,
    chapter_count: Option<usize>,
    target_words: Option<usize>,
) -> Vec<String> {
    let mut paths = requested_paths(root, text);
    if paths.is_empty() {
        let count = chapter_count
            .or_else(|| target_words.map(|words| words.div_ceil(1_000).clamp(1, 20)))
            .unwrap_or(1);
        paths.extend((1..=count).map(|index| format!("manuscript/chapter-{index:02}.md")));
    }
    paths.extend(existing_final_paths(files));
    paths.sort();
    paths.dedup();
    paths
}

pub(crate) fn requested_paths(root: &str, text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for token in text.split(|ch: char| !path_char(ch)) {
        let trimmed = token.trim_matches('.');
        if let Some(relative) = trimmed.strip_prefix(&format!("{}/", root.trim_end_matches('/'))) {
            push_manuscript_path(&mut out, relative);
        } else {
            push_manuscript_path(&mut out, trimmed);
        }
    }
    out
}

pub(crate) fn manuscript_words(files: &[ManuscriptFile<'_>]) -> usize {
    files
        .iter()
        .filter(|file| final_manuscript_path(file.relative))
        .map(|file| prose_words(file.text))
        .sum()
}

pub(crate) fn manuscript_path_words(files: &[ManuscriptFile<'_>], path: &str) -> usize {
    files
        .iter()
        .find(|file| file.relative == path)
        .map(|file| prose_words(file.text))
        .unwrap_or(0)
}

pub(crate) fn next_scene_path(
    files: &[ManuscriptFile<'_>],
    chapter_path: &str,
    floor: usize,
) -> Option<String> {
    let chapter = chapter_name(chapter_path)?;
    let scenes = scene_files(files, chapter);
    if let Some((path, _)) = scenes.iter().find(|(_, file)| weak_scene_text(file)) {
        return Some(path.clone());
    }
    let scene_words = scenes
        .iter()
        .map(|(_, file)| prose_words(file.text))
        .sum::<usize>();
    if scene_words >= floor {
        return None;
    }
    Some(format!(
        "manuscript/scenes/{chapter}/scene-{:02}.md",
        scenes.len().saturating_add(1)
    ))
}

pub(crate) fn scene_atoms_for_missing(
    files: &[ManuscriptFile<'_>],
    missing: &[String],
) -> Vec<String> {
    let mut paths = Vec::new();
    for chapter in missing.iter().filter_map(|path| chapter_name(path)) {
        paths.extend(
            scene_files(files, chapter)
                .into_iter()
                .map(|(path, _)| path),
        );
    }
    paths.sort();
    paths.dedup();
    paths
}

pub(crate) fn chapter_floor(total_floor: usize, count: usize) -> usize {
    if count == 0 {
        total_floor
    } else {
        total_floor.div_ceil(count)
    }
}

fn push_manuscript_path(out: &mut Vec<String>, path: &str) {
    if final_manuscript_path(path) && !out.iter().any(|item| item == path) {
        out.push(path.to_string());
    }
}

fn existing_final_paths(files: &[ManuscriptFile<'_>]) -> Vec<String> {
    files
        .iter()
        .filter(|file| final_manuscript_path(file.relative))
        .map(|file| file.relative.to_string())
        .collect()
}

fn final_manuscript_path(path: &str) -> bool {
    path.starts_with("manuscript/")
        && !path.starts_with("manuscript/scenes/")
        && path.ends_with(".md")
}

fn scene_files<'a, 'b>(
    files: &'a [ManuscriptFile<'b>],
    chapter: &str,
) -> Vec<(String, &'a ManuscriptFile<'b>)> {
    let prefix = format!("manuscript/scenes/{chapter}/");
    let mut out = files
        .iter()
        .filter(|file| file.relative.starts_with(&prefix) && file.relative.ends_with(".md"))
        .map(|file| (file.relative.to_string(), file))
        .collect::<Vec<_>>();
    out.sort_by(|left, right| left.0.cmp(&right.0));
    out
}

fn chapter_name(path: &str) -> Option<&str> {
    path.strip_prefix("manuscript/")?.strip_suffix(".md")
}

fn weak_scene_text(file: &ManuscriptFile<'_>) -> bool {
    prose_words(file.text) < 25
        || !file.text.contains("##")
        || file.lower.contains("content_state=structure-only")
        || file.lower.contains("placeholder")
        || file.lower.contains("todo")
}
