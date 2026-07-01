use crate::kernel::manuscript_path::{
    chapter_count, default_final_path, first_manuscript_path, requested_final_paths, target_words,
    write_path_for,
};
use crate::kernel::obligation_parse::line_value;
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManuscriptTaskKind {
    StoryBible,
    Manuscript,
    StoryBibleThenManuscript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManuscriptFacts {
    pub active: bool,
    pub task_kind: ManuscriptTaskKind,
    pub allowed_root: String,
    pub target_words: Option<usize>,
    pub target_word_floor: usize,
    pub chapter_count: Option<usize>,
    pub requested_paths: Vec<String>,
    pub missing_paths: Vec<String>,
    pub scene_atoms_unassembled: Vec<String>,
    pub next_path: Option<String>,
    pub words_written: usize,
    pub anomaly_shrink_level: u32,
    pub exact_path_required: bool,
    pub forbidden_roots: Vec<String>,
    pub max_file_bytes: usize,
    pub output_token_budget: usize,
}

pub(crate) fn facts_from_snapshot(snapshot: &RuntimeSnapshot) -> Option<ManuscriptFacts> {
    let root = snapshot.artifact.root.as_deref()?;
    let text = source_text(snapshot);
    let requested_paths = requested_final_paths(root, &text);
    let target_words =
        observed_number(snapshot, "manuscript_target_words").or_else(|| target_words(&text));
    let chapter_count = chapter_count(&text)
        .or_else(|| (!requested_paths.is_empty()).then_some(requested_paths.len()));
    let task_kind = task_kind(root, &text, snapshot, target_words, chapter_count);
    if matches!(task_kind, ManuscriptTaskKind::StoryBible) {
        return None;
    }
    let mut missing_paths = observed_paths(snapshot, "manuscript_missing_paths");
    if missing_paths.is_empty() {
        missing_paths = requested_paths.clone();
    }
    let final_path = missing_paths
        .first()
        .cloned()
        .or_else(|| requested_paths.first().cloned())
        .unwrap_or_else(|| default_final_path(root, chapter_count));
    let observed_next = observed_value(snapshot, "next_manuscript_path")
        .filter(|value| value != "none")
        .or_else(|| observed_write_path(root, snapshot));
    let next_path = Some(
        observed_next
            .map(|path| write_path_for(&path))
            .unwrap_or_else(|| write_path_for(&final_path)),
    );
    if missing_paths.is_empty() {
        missing_paths.push(final_path);
    }
    let floor = target_words
        .map(|words| words.saturating_mul(85) / 100)
        .unwrap_or(600);
    let shrink = anomaly_level(snapshot);
    let max_file_bytes = max_file_bytes(shrink);
    Some(ManuscriptFacts {
        active: true,
        task_kind,
        allowed_root: root.to_string(),
        target_words,
        target_word_floor: floor,
        chapter_count,
        requested_paths,
        missing_paths,
        scene_atoms_unassembled: observed_paths(snapshot, "scene_atoms_unassembled"),
        next_path,
        words_written: observed_number(snapshot, "manuscript_word_count").unwrap_or(0),
        anomaly_shrink_level: shrink,
        exact_path_required: text.contains("stories/") && text.contains("/manuscript/"),
        forbidden_roots: forbidden_roots(&text),
        max_file_bytes,
        output_token_budget: max_file_bytes / 4,
    })
}

fn task_kind(
    root: &str,
    text: &str,
    snapshot: &RuntimeSnapshot,
    target_words: Option<usize>,
    chapter_count: Option<usize>,
) -> ManuscriptTaskKind {
    let manuscript = root.trim_start_matches("./").starts_with("stories/")
        && (text.contains("/manuscript/")
            || text.contains("manuscript")
            || target_words.is_some()
            || chapter_count.is_some()
            || snapshot.artifact.kind.as_deref() == Some("manuscript"));
    let bible = text.contains("story bible") || text.contains("reference");
    match (bible, manuscript) {
        (true, true) => ManuscriptTaskKind::StoryBibleThenManuscript,
        (false, true) => ManuscriptTaskKind::Manuscript,
        _ => ManuscriptTaskKind::StoryBible,
    }
}

fn source_text(snapshot: &RuntimeSnapshot) -> String {
    [
        snapshot.case.owner_objective.as_deref(),
        snapshot.observation.latest.as_deref(),
        snapshot.observation.latest_successful.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n")
    .to_ascii_lowercase()
}

fn observed_value(snapshot: &RuntimeSnapshot, key: &str) -> Option<String> {
    [
        snapshot.observation.latest.as_deref(),
        snapshot.observation.latest_successful.as_deref(),
    ]
    .into_iter()
    .flatten()
    .find_map(|text| line_value(text, key))
}

fn observed_number(snapshot: &RuntimeSnapshot, key: &str) -> Option<usize> {
    observed_value(snapshot, key).and_then(|value| value.parse().ok())
}

fn observed_paths(snapshot: &RuntimeSnapshot, key: &str) -> Vec<String> {
    observed_value(snapshot, key)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty() && *item != "none")
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn observed_write_path(root: &str, snapshot: &RuntimeSnapshot) -> Option<String> {
    [
        snapshot.observation.latest.as_deref(),
        snapshot.observation.latest_successful.as_deref(),
    ]
    .into_iter()
    .flatten()
    .find_map(|text| first_manuscript_path(root, text))
}

fn forbidden_roots(text: &str) -> Vec<String> {
    ["structured-output", "output", "artifact", "work-product"]
        .into_iter()
        .filter(|root| text.contains(root))
        .map(str::to_string)
        .collect()
}

fn anomaly_level(snapshot: &RuntimeSnapshot) -> u32 {
    snapshot.retry_count.max(
        snapshot
            .provider
            .retry_count
            .saturating_add(u32::from(snapshot.provider.anomaly_class.is_some())),
    )
}

fn max_file_bytes(shrink: u32) -> usize {
    match shrink {
        0 => 1_800,
        1 => 1_200,
        _ => 800,
    }
}
