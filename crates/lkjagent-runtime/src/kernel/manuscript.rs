use crate::kernel::obligation_parse::line_value;
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ManuscriptFacts {
    pub active: bool,
    pub target_words: Option<usize>,
    pub target_word_floor: usize,
    pub chapter_count: Option<usize>,
    pub requested_paths: Vec<String>,
    pub missing_paths: Vec<String>,
    pub next_path: Option<String>,
    pub words_written: usize,
    pub anomaly_shrink_level: u32,
    pub exact_path_required: bool,
    pub forbidden_roots: Vec<String>,
}

pub(crate) fn facts_from_snapshot(snapshot: &RuntimeSnapshot) -> Option<ManuscriptFacts> {
    let root = snapshot.artifact.root.as_deref()?;
    let text = source_text(snapshot);
    let requested_paths = requested_paths(root, &text);
    let target_words =
        observed_number(snapshot, "manuscript_target_words").or_else(|| target_words(&text));
    let chapter_count = chapter_count(&text)
        .or_else(|| (!requested_paths.is_empty()).then_some(requested_paths.len()));
    let active = manuscript_active(root, &text, snapshot, target_words, chapter_count);
    if !active {
        return None;
    }
    let mut missing_paths = observed_paths(snapshot, "missing_manuscript_paths");
    if missing_paths.is_empty() {
        missing_paths = requested_paths.clone();
    }
    let next_path = observed_value(snapshot, "next_manuscript_path")
        .filter(|value| value != "none")
        .or_else(|| missing_paths.first().cloned())
        .or_else(|| default_path(root, chapter_count));
    if missing_paths.is_empty() {
        if let Some(path) = next_path.clone() {
            missing_paths.push(path);
        }
    }
    let floor = target_words
        .map(|words| words.saturating_mul(85) / 100)
        .unwrap_or(600);
    let words_written = observed_number(snapshot, "manuscript_word_count").unwrap_or(0);
    Some(ManuscriptFacts {
        active,
        target_words,
        target_word_floor: floor,
        chapter_count,
        requested_paths,
        missing_paths,
        next_path,
        words_written,
        anomaly_shrink_level: anomaly_level(snapshot),
        exact_path_required: text.contains("stories/") && text.contains("/manuscript/"),
        forbidden_roots: forbidden_roots(&text),
    })
}

fn manuscript_active(
    root: &str,
    text: &str,
    snapshot: &RuntimeSnapshot,
    target_words: Option<usize>,
    chapter_count: Option<usize>,
) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
        && (text.contains("/manuscript/")
            || text.contains("manuscript")
            || target_words.is_some()
            || chapter_count.is_some()
            || snapshot.artifact.kind.as_deref() == Some("manuscript"))
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
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn requested_paths(root: &str, text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for token in text.split(|ch: char| !path_char(ch)) {
        let path = token.trim_matches('.');
        if path.starts_with(root) && path.contains("/manuscript/") && path.ends_with(".md") {
            push_unique(&mut out, path.to_string());
        }
    }
    out
}

fn default_path(root: &str, chapter_count: Option<usize>) -> Option<String> {
    let index = chapter_count.filter(|count| *count == 1).map_or(1, |_| 1);
    Some(format!(
        "{}/manuscript/chapter-{index:02}.md",
        root.trim_end_matches('/')
    ))
}

fn target_words(text: &str) -> Option<usize> {
    numbers_before(text, "word").into_iter().max()
}

fn chapter_count(text: &str) -> Option<usize> {
    numbers_before(text, "chapter").into_iter().max()
}

fn numbers_before(text: &str, unit: &str) -> Vec<usize> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != ',')
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|pair| {
            (pair[1].starts_with(unit))
                .then(|| parse_number(pair[0]))
                .flatten()
        })
        .collect()
}

fn parse_number(value: &str) -> Option<usize> {
    match value.trim_matches(',') {
        "one" => Some(1),
        "ten" => Some(10),
        raw => raw.replace(',', "").parse().ok(),
    }
}

fn forbidden_roots(text: &str) -> Vec<String> {
    if text.contains("structured-output") {
        vec!["structured-output".to_string()]
    } else {
        Vec::new()
    }
}

fn anomaly_level(snapshot: &RuntimeSnapshot) -> u32 {
    snapshot.retry_count.max(
        snapshot
            .provider
            .retry_count
            .saturating_add(u32::from(snapshot.provider.anomaly_class.is_some())),
    )
}

fn push_unique(out: &mut Vec<String>, value: String) {
    if !out.iter().any(|item| item == &value) {
        out.push(value);
    }
}

fn path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '/' | '-' | '_' | '.')
}
