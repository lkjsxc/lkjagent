use crate::artifact_story_text::{contains_any, full_path, numbers_before, path_char, prose_words};

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
    pub missing_paths: Vec<String>,
    pub next_path: Option<String>,
    pub total_words: usize,
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
    let paths = planned_paths(root, files, &joined, chapter_count, target_words);
    let total_words = manuscript_words(files);
    let missing_paths = paths
        .iter()
        .filter(|path| manuscript_path_words(files, path) < chapter_floor(word_floor, paths.len()))
        .cloned()
        .collect::<Vec<_>>();
    let next_path = missing_paths.first().cloned();
    ManuscriptFacts {
        active,
        target_words,
        word_floor,
        chapter_count,
        missing_paths,
        next_path,
        total_words,
    }
}
pub(crate) fn readiness_failure(root: &str, facts: &ManuscriptFacts) -> Option<String> {
    if !facts.active {
        return None;
    }
    if facts.missing_paths.is_empty() && facts.total_words >= facts.word_floor {
        return None;
    }
    let missing = if facts.missing_paths.is_empty() {
        "none".to_string()
    } else {
        facts
            .missing_paths
            .iter()
            .map(|path| full_path(root, path))
            .collect::<Vec<_>>()
            .join(",")
    };
    Some(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-manuscript-content\nfailed=1\nfailures:\n- manuscript_missing_paths: {missing}\n- manuscript_word_count: {}\n- manuscript_target_words: {}\n- next_manuscript_path: {}\nnext_decision_required=true\ncandidate_action=artifact.next",
        facts.total_words,
        facts.target_words.unwrap_or(facts.word_floor),
        facts.next_path.as_deref().map(|path| full_path(root, path)).unwrap_or_else(|| "none".to_string())
    ))
}

fn active_manuscript(root: &str, scale: &str, files: &[ManuscriptFile<'_>], joined: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
        && (scale.contains("full-draft")
            || scale.contains("manuscript")
            || joined.contains("/manuscript/")
            || joined.contains("manuscript/")
            || (contains_any(joined, &["novel", "story", "chapter", "scene"])
                && contains_any(joined, &[" word", " words", "-word"]))
            || files
                .iter()
                .any(|file| file.relative.starts_with("manuscript/")))
}

fn planned_paths(
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
    for file in files {
        if file.relative.starts_with("manuscript/")
            && !paths.iter().any(|path| path == file.relative)
        {
            paths.push(file.relative.to_string());
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn requested_paths(root: &str, text: &str) -> Vec<String> {
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

fn push_manuscript_path(out: &mut Vec<String>, path: &str) {
    if path.starts_with("manuscript/")
        && path.ends_with(".md")
        && !out.iter().any(|item| item == path)
    {
        out.push(path.to_string());
    }
}

fn manuscript_words(files: &[ManuscriptFile<'_>]) -> usize {
    files
        .iter()
        .filter(|file| file.relative.starts_with("manuscript/"))
        .map(|file| prose_words(file.text))
        .sum()
}

fn manuscript_path_words(files: &[ManuscriptFile<'_>], path: &str) -> usize {
    files
        .iter()
        .find(|file| file.relative == path)
        .map(|file| prose_words(file.text))
        .unwrap_or(0)
}

fn chapter_floor(total_floor: usize, count: usize) -> usize {
    if count == 0 {
        total_floor
    } else {
        total_floor.div_ceil(count).min(700)
    }
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
