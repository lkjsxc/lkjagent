pub(crate) fn requested_total(text: &str) -> usize {
    if let Some(count) = first_number(text) {
        if text.to_ascii_lowercase().contains("thousand") {
            return count.saturating_mul(1000);
        }
        return count;
    }
    if text.contains("万") {
        10_000
    } else {
        0
    }
}

pub(crate) fn section_count(text: &str) -> usize {
    let lower = text.to_ascii_lowercase();
    if lower.contains("chapter") || lower.contains("section") || lower.contains("lesson") {
        first_number(text).unwrap_or(0)
    } else {
        0
    }
}

pub(crate) fn first_number(text: &str) -> Option<usize> {
    let mut digits = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if !digits.is_empty() {
            break;
        }
    }
    digits.parse::<usize>().ok()
}

pub(crate) fn requested_paths(text: &str) -> Vec<String> {
    text.split(|ch: char| !path_char(ch))
        .map(|token| token.trim_matches('.'))
        .filter(|token| token.contains('/'))
        .filter(|token| !generic_root(token))
        .map(str::to_string)
        .collect()
}

pub(crate) fn root_from_path(path: &str) -> String {
    if path.ends_with(".md") {
        path.rsplit_once('/')
            .map_or(path, |(root, _)| root)
            .to_string()
    } else {
        path.trim_end_matches('/').to_string()
    }
}

pub(crate) fn clean_root(root: &str) -> Option<String> {
    let trimmed = root.trim().trim_start_matches("./").trim_end_matches('/');
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub(crate) fn generic_root(root: &str) -> bool {
    matches!(
        root.trim_start_matches("./"),
        "structured-output" | "output" | "artifact" | "work-product"
    )
}

pub(crate) fn title_from_text(text: &str) -> String {
    let quoted = text.split('"').nth(1).or_else(|| text.split('“').nth(1));
    quoted.map_or_else(|| normalize_title(text), normalize_title)
}

pub(crate) fn normalize_title(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.chars().take(80).collect()
    }
}

pub(crate) fn slug(value: &str) -> String {
    let slug = value
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    slug.split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub(crate) fn audience(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    if lower.contains("learner") {
        "learner"
    } else if lower.contains("operator") {
        "operator"
    } else {
        "unspecified"
    }
    .to_string()
}

pub(crate) fn language_hint(text: &str) -> String {
    if text.chars().any(|ch| {
        ('\u{3040}'..='\u{30ff}').contains(&ch) || ('\u{4e00}'..='\u{9fff}').contains(&ch)
    }) {
        "ja".to_string()
    } else {
        "unspecified".to_string()
    }
}

pub(crate) fn evidence_requirements(kind: &str) -> Vec<String> {
    [
        "plan",
        "observation",
        "document-structure",
        "artifact-readiness",
    ]
    .into_iter()
    .chain((kind == "manuscript").then_some("manuscript-word-count"))
    .map(str::to_string)
    .collect()
}

fn path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '/' | '-' | '_' | '.')
}
