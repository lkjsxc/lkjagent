use crate::count_profile_anchor::{raw_objective_anchors, truncate_anchor};
use crate::count_profile_anchor_cleanup::trim_count_mechanics;

pub(crate) fn content_anchors(objective: &str) -> Vec<String> {
    let anchors = raw_objective_anchors(objective);
    let filtered = anchors
        .iter()
        .filter_map(|anchor| content_anchor(anchor))
        .collect::<Vec<_>>();
    if !filtered.is_empty() {
        return filtered;
    }
    let fallback = anchors
        .iter()
        .filter_map(|anchor| fallback_anchor(anchor))
        .collect::<Vec<_>>();
    if fallback.is_empty() {
        anchors
            .iter()
            .map(|anchor| truncate_anchor(anchor))
            .collect()
    } else {
        fallback
    }
}

fn content_anchor(anchor: &str) -> Option<String> {
    let cleaned = trim_count_mechanics(trim_request_suffix(trim_scaffold_suffix(
        strip_scaffold_prefix(anchor),
    )))
    .trim();
    let lower = cleaned.to_ascii_lowercase();
    if operational_anchor(cleaned, &lower)
        || meta_constraint_anchor(&lower)
        || structural_count_anchor(cleaned, &lower)
        || cleaned.chars().count() < 4
    {
        None
    } else {
        Some(truncate_anchor(cleaned))
    }
}

fn fallback_anchor(anchor: &str) -> Option<String> {
    let cleaned = trim_count_mechanics(trim_request_suffix(trim_scaffold_suffix(
        strip_scaffold_prefix(anchor),
    )))
    .trim();
    let lower = cleaned.to_ascii_lowercase();
    if operational_anchor(cleaned, &lower)
        || meta_constraint_anchor(&lower)
        || cleaned.chars().count() < 4
    {
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
        " with documentation and manuscript files combined",
        " with documentation and manuscript combined",
    ] {
        if lower.ends_with(suffix) {
            let end = anchor.len().saturating_sub(suffix.len());
            return anchor[..end].trim();
        }
    }
    anchor
}

fn trim_request_suffix(anchor: &str) -> &str {
    for suffix in [
        "を作ってください",
        "を作成してください",
        "を生成してください",
        "してください",
        "して下さい",
    ] {
        if anchor.ends_with(suffix) {
            let end = anchor.len().saturating_sub(suffix.len());
            return anchor[..end].trim();
        }
    }
    anchor
}

fn operational_anchor(cleaned: &str, lower: &str) -> bool {
    lower.starts_with("use gpt")
        || lower.starts_with("use codex")
        || lower.contains("codex-spark thrift")
        || lower.contains("model thrift")
        || (lower.contains("codex") && (lower.contains("spark") || lower.contains("thrift")))
        || (lower.contains("codex") && (cleaned.contains('枠') || cleaned.contains("節約")))
}

fn meta_constraint_anchor(lower: &str) -> bool {
    lower.contains("not story-specific")
        || lower.contains("not story specific")
        || lower.starts_with("keep it generic")
        || lower.starts_with("keep this generic")
        || lower.starts_with("keep it reusable")
        || (lower.starts_with("keep ") && lower.contains("reusable"))
}

fn structural_count_anchor(cleaned: &str, lower: &str) -> bool {
    let has_file_unit = lower.contains("file")
        || lower.contains("document")
        || lower.contains("docs")
        || lower.contains("documentation")
        || lower.contains("markdown")
        || cleaned.contains("ファイル")
        || cleaned.contains("ドキュメント")
        || cleaned.contains("文書");
    let has_total_word = lower.contains("total")
        || lower.contains("combined")
        || lower.contains("in all")
        || cleaned.contains("合計")
        || cleaned.contains("総数")
        || cleaned.contains("合わせた");
    let has_content_word = lower.contains("story")
        || lower.contains("narrative")
        || lower.contains("guide")
        || lower.contains("manual")
        || lower.contains("report")
        || lower.contains("procedure")
        || cleaned.contains("物語")
        || cleaned.contains("本文")
        || cleaned.contains("本編")
        || cleaned.contains("成果物");
    let allocation_only = (lower.starts_with("with ") || lower.starts_with("including "))
        && has_file_unit
        && (lower.contains("remaining files")
            || lower.contains("ordered main")
            || lower.contains("planning note")
            || lower.contains("planning notes")
            || lower.contains("design memo")
            || lower.contains("design memos"));
    has_file_unit && (cleaned.contains("総数") || cleaned.contains("合わせた"))
        || has_file_unit && has_total_word && !has_content_word
        || allocation_only
        || cleaned.ends_with("ファイルぐらいで")
        || cleaned.ends_with("ファイル程度で")
        || cleaned.ends_with("ファイルくらいで")
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
