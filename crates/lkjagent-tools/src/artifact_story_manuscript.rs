use crate::artifact_story_manuscript_paths::{
    chapter_floor, manuscript_path_words, manuscript_words, next_scene_path, planned_paths,
    requested_paths, scene_atoms_for_missing,
};
use crate::artifact_story_text::{contains_any, full_path, numbers_before};

pub(crate) struct ManuscriptFile<'a> {
    pub relative: &'a str,
    pub lower: &'a str,
    pub text: &'a str,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ManuscriptFacts {
    pub active: bool,
    pub target_words: Option<usize>,
    pub word_floor: usize,
    pub chapter_count: Option<usize>,
    pub required_paths: Vec<String>,
    pub missing_paths: Vec<String>,
    pub next_path: Option<String>,
    pub total_words: usize,
    pub scene_atoms_unassembled: Vec<String>,
}

pub(crate) fn facts(root: &str, scale: &str, files: &[ManuscriptFile<'_>]) -> ManuscriptFacts {
    let joined = joined_text(scale, files);
    let active = active_manuscript(root, scale, files, &joined);
    if !active {
        return ManuscriptFacts::default();
    }
    let target_words = target_words(&joined);
    let requested_count = requested_paths(root, &joined).len();
    let chapter_count = chapter_count(&joined).or((requested_count > 0).then_some(requested_count));
    let word_floor = target_words
        .map(|words| words.saturating_mul(85) / 100)
        .unwrap_or(600);
    let required_paths = planned_paths(root, files, &joined, chapter_count, target_words);
    let total_words = manuscript_words(files);
    let per_chapter = chapter_floor(word_floor, required_paths.len());
    let missing_paths = missing_paths(files, &required_paths, per_chapter);
    let scene_atoms_unassembled = scene_atoms_for_missing(files, &missing_paths);
    let next_path = missing_paths
        .first()
        .and_then(|path| next_scene_path(files, path, per_chapter));
    ManuscriptFacts {
        active,
        target_words,
        word_floor,
        chapter_count,
        required_paths,
        missing_paths,
        next_path,
        total_words,
        scene_atoms_unassembled,
    }
}

pub(crate) fn readiness_failure(root: &str, facts: &ManuscriptFacts) -> Option<String> {
    if !facts.active {
        return None;
    }
    if facts.missing_paths.is_empty() && facts.total_words >= facts.word_floor {
        return None;
    }
    let missing = full_paths(root, &facts.missing_paths);
    let required = full_paths(root, &facts.required_paths);
    let scenes = full_paths(root, &facts.scene_atoms_unassembled);
    let next = facts
        .next_path
        .as_deref()
        .map(|path| full_path(root, path))
        .unwrap_or_else(|| "none".to_string());
    Some(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-manuscript-content\n\
         failed=1\nfailures:\n- manuscript_missing_paths: {missing}\n\
         - manuscript_word_count: {}\n- manuscript_target_words: {}\n\
         - required_manuscript_paths: {required}\n\
         - scene_atoms_unassembled: {scenes}\n- next_manuscript_path: {next}\n\
         next_decision_required=true\ncandidate_action=artifact.next",
        facts.total_words,
        facts.target_words.unwrap_or(facts.word_floor),
    ))
}

fn active_manuscript(root: &str, scale: &str, files: &[ManuscriptFile<'_>], joined: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
        && (scale.contains("full-draft")
            || scale.contains("manuscript")
            || joined.contains("/manuscript/")
            || joined.contains("manuscript/")
            || prose_request(joined)
            || files
                .iter()
                .any(|file| file.relative.starts_with("manuscript/")))
}

fn prose_request(joined: &str) -> bool {
    contains_any(joined, &["novel", "story", "chapter", "scene"])
        && contains_any(joined, &[" word", " words", "-word"])
}

fn missing_paths(
    files: &[ManuscriptFile<'_>],
    required_paths: &[String],
    per_chapter: usize,
) -> Vec<String> {
    required_paths
        .iter()
        .filter(|path| manuscript_path_words(files, path) < per_chapter)
        .cloned()
        .collect()
}

fn target_words(text: &str) -> Option<usize> {
    numbers_before(text, "word").into_iter().max()
}

fn chapter_count(text: &str) -> Option<usize> {
    numbers_before(text, "chapter").into_iter().max()
}

fn joined_text(scale: &str, files: &[ManuscriptFile<'_>]) -> String {
    let mut text = scale.to_ascii_lowercase();
    for file in files {
        text.push('\n');
        text.push_str(file.lower);
    }
    text
}

fn full_paths(root: &str, paths: &[String]) -> String {
    if paths.is_empty() {
        "none".to_string()
    } else {
        paths
            .iter()
            .map(|path| full_path(root, path))
            .collect::<Vec<_>>()
            .join(",")
    }
}
