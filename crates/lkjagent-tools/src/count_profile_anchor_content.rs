use crate::count_profile_anchor::{raw_objective_anchors, truncate_anchor};

pub(crate) fn content_anchors(objective: &str) -> Vec<String> {
    let anchors = raw_objective_anchors(objective);
    let filtered = anchors
        .iter()
        .filter_map(|anchor| content_anchor(anchor))
        .collect::<Vec<_>>();
    if filtered.is_empty() {
        anchors
            .iter()
            .map(|anchor| truncate_anchor(anchor))
            .collect()
    } else {
        filtered
    }
}

fn content_anchor(anchor: &str) -> Option<String> {
    let cleaned = trim_scaffold_suffix(strip_scaffold_prefix(anchor)).trim();
    let lower = cleaned.to_ascii_lowercase();
    if operational_anchor(&lower) || cleaned.chars().count() < 4 {
        None
    } else {
        Some(truncate_anchor(cleaned))
    }
}

fn strip_scaffold_prefix(anchor: &str) -> &str {
    let lower = anchor.to_ascii_lowercase();
    if !starts_with_creation(&lower) {
        return anchor;
    }
    if let Some(index) = lower.find(" for ") {
        let before = &lower[..index];
        if before.contains("file")
            || before.contains("document")
            || before.contains("markdown")
            || before.contains("total")
        {
            return &anchor[index.saturating_add(5)..];
        }
    }
    anchor
}

fn trim_scaffold_suffix(anchor: &str) -> &str {
    let lower = anchor.to_ascii_lowercase();
    for suffix in [
        " with docs and main content",
        " with documentation and main content",
        " with docs and ordered main files",
        " with ordered main files",
        " including docs and main content",
        " including documentation and main content",
    ] {
        if lower.ends_with(suffix) {
            let end = anchor.len().saturating_sub(suffix.len());
            return anchor[..end].trim();
        }
    }
    anchor
}

fn operational_anchor(lower: &str) -> bool {
    lower.starts_with("use gpt")
        || lower.starts_with("use codex")
        || lower.contains("codex-spark thrift")
        || lower.contains("model thrift")
}

fn starts_with_creation(lower: &str) -> bool {
    [
        "build ",
        "create ",
        "generate ",
        "make ",
        "produce ",
        "write ",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
}
