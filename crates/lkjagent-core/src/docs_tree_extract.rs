#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsFields {
    pub root: String,
    pub topic: String,
    pub page_count: Option<usize>,
    pub exact: bool,
}

pub fn extract(objective: &str) -> DocsFields {
    let lower = objective.to_ascii_lowercase();
    let root = lower
        .split_whitespace()
        .find(|word| word.contains("docs/") || word.starts_with("documentation/"))
        .map(clean)
        .unwrap_or_else(|| "docs/guide".to_string());
    let page_count = number_before(&lower, "page").or_else(|| kanji_count(objective));
    let exact = page_count.is_some()
        && !has_any(&lower, &["about", "roughly", "approx", "around"])
        && !objective.contains('約');
    DocsFields {
        root,
        topic: topic(&lower),
        page_count,
        exact,
    }
}

pub(crate) fn parse_inputs(inputs: &str) -> DocsFields {
    let root = inputs
        .split_whitespace()
        .find_map(|part| part.strip_prefix("root="))
        .unwrap_or("docs/guide");
    let exact = inputs.contains("exact=true");
    let page_count = inputs
        .split("pages=Some(")
        .nth(1)
        .and_then(|tail| tail.split(')').next())
        .and_then(|n| n.parse().ok());
    DocsFields {
        root: root.to_string(),
        topic: "docs".to_string(),
        page_count,
        exact,
    }
}

fn number_before(lower: &str, marker: &str) -> Option<usize> {
    let words = lower.split_whitespace().collect::<Vec<_>>();
    words
        .windows(2)
        .find_map(|pair| {
            if pair[1].starts_with(marker) {
                pair[0].parse().ok()
            } else {
                None
            }
        })
        .or_else(|| {
            if lower.contains(marker) {
                words.iter().find_map(|word| word.parse().ok())
            } else {
                None
            }
        })
}

fn kanji_count(text: &str) -> Option<usize> {
    if text.contains("三") {
        Some(3)
    } else if text.contains("二") {
        Some(2)
    } else {
        None
    }
}

fn topic(lower: &str) -> String {
    if lower.contains("daemon") {
        "daemon".to_string()
    } else {
        "docs".to_string()
    }
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn clean(word: &str) -> String {
    word.trim_matches(|ch: char| ch == ',' || ch == '.' || ch == ':' || ch == ';')
        .to_string()
}
