pub(crate) fn trim_count_mechanics(anchor: &str) -> &str {
    let without_create = strip_content_creation_prefix(anchor).trim();
    let without_hyphen = trim_english_count_prefix(without_create).trim();
    let without_english = trim_english_count_suffix(without_hyphen).trim();
    trim_japanese_count_prefix(without_english).trim()
}

fn strip_content_creation_prefix(anchor: &str) -> &str {
    let lower = anchor.to_ascii_lowercase();
    for prefix in [
        "build ",
        "create ",
        "generate ",
        "make ",
        "produce ",
        "write ",
    ] {
        if lower.starts_with(prefix) {
            let rest = &anchor[prefix.len()..];
            if !count_leading(&rest.to_ascii_lowercase()) {
                return rest;
            }
        }
    }
    anchor
}

fn trim_english_count_suffix(anchor: &str) -> &str {
    let lower = anchor.to_ascii_lowercase();
    for marker in [
        " in about ",
        " in around ",
        " in approximately ",
        " in approx ",
        " across about ",
        " across around ",
        " over about ",
        " over around ",
        " as about ",
        " as around ",
        " as approximately ",
        " as approx ",
        " in ",
        " across ",
        " over ",
        " as ",
    ] {
        if let Some(index) = lower.find(marker) {
            let suffix = &lower[index.saturating_add(marker.len())..];
            if counted_file_suffix(suffix) {
                return anchor[..index].trim();
            }
        }
    }
    anchor
}

fn trim_english_count_prefix(anchor: &str) -> &str {
    for prefix in [
        "a one-hundred-file ",
        "a one hundred file ",
        "a hundred-file ",
        "a hundred file ",
        "a 100-file ",
        "a 100 file ",
        "one-hundred-file ",
        "one hundred file ",
        "hundred-file ",
        "hundred file ",
        "100-file ",
        "100 file ",
    ] {
        if anchor.to_ascii_lowercase().starts_with(prefix) {
            return anchor[prefix.len()..].trim();
        }
    }
    anchor
}

fn trim_japanese_count_prefix(anchor: &str) -> &str {
    for marker in ["ぐらいの", "くらいの", "程度の", "ほどの"] {
        let Some(index) = anchor.find(marker) else {
            continue;
        };
        let prefix = &anchor[..index];
        if prefix.contains("ファイル") || prefix.contains("ドキュメント") || prefix.contains("文書")
        {
            return anchor[index.saturating_add(marker.len())..].trim();
        }
    }
    anchor
}

fn count_leading(lower: &str) -> bool {
    [
        "about ",
        "around ",
        "approx ",
        "approximately ",
        "one hundred",
        "hundred",
        "100",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
}

fn counted_file_suffix(lower: &str) -> bool {
    let has_count = lower.chars().any(|ch| ch.is_ascii_digit())
        || lower.contains("hundred")
        || lower.contains("dozen");
    let has_file_unit = lower.contains("file")
        || lower.contains("document")
        || lower.contains("docs")
        || lower.contains("manuscript");
    has_count && has_file_unit
}
