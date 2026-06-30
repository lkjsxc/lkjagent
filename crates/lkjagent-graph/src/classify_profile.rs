use crate::classify_signals::{
    contains_any, counted_story_request, large_content_signal, manuscript_request,
    operational_compaction_request, priority_counted_content_request,
    priority_long_content_request, unquoted_lower,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentProfile {
    pub content_creation: bool,
    pub content_artifact: bool,
    pub operational_compaction: bool,
    pub artifact_kind: Option<String>,
    pub requested_scale: Option<String>,
}

pub(crate) fn intent_facts(lower: &str, content: &str) -> IntentProfile {
    let content_artifact = priority_long_content_request(lower, content);
    let content_creation = priority_counted_content_request(lower, content) || content_artifact;
    let unquoted = unquoted_lower(content);
    IntentProfile {
        content_creation,
        content_artifact,
        operational_compaction: operational_compaction_request(&unquoted),
        artifact_kind: content_artifact.then(|| artifact_kind(lower, content).to_string()),
        requested_scale: requested_scale(lower).map(str::to_string),
    }
}

fn artifact_kind(lower: &str, content: &str) -> &'static str {
    if contains_any(lower, &["dictionary", "glossary", "lexicon"]) {
        "dictionary"
    } else if contains_any(lower, &["cookbook", "recipe", "bread"]) {
        "cookbook"
    } else if contains_any(
        lower,
        &[
            "story",
            "novel",
            "manuscript",
            "narrative",
            "chapter",
            "scene",
        ],
    ) || contains_any(content, &["小説", "物語", "章"])
    {
        "story"
    } else {
        "artifact"
    }
}

fn requested_scale(lower: &str) -> Option<&'static str> {
    if contains_any(
        lower,
        &["full draft", "complete draft", "manuscript", "20 chapters"],
    ) || manuscript_request(lower) && contains_any(lower, &["word", "chapter", "scene"])
    {
        Some("full-draft")
    } else if large_content_signal(lower, lower) || counted_story_request(lower) {
        Some("large-story")
    } else {
        None
    }
}
