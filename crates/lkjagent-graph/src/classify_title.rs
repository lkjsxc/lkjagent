pub(crate) fn owner_title(objective: &str) -> Option<String> {
    quoted_title_after_marker(objective).or_else(|| unquoted_title_after_marker(objective))
}

pub(crate) fn owner_title_alias(objective: &str) -> Option<String> {
    let title = owner_title(objective)?;
    slug(&title)
}

fn quoted_title_after_marker(objective: &str) -> Option<String> {
    let lower = objective.to_ascii_lowercase();
    for marker in TITLE_MARKERS {
        let Some(index) = lower.find(marker) else {
            continue;
        };
        let start = index + marker.len();
        let rest = objective.get(start..)?.trim_start_matches(marker_trim);
        let mut chars = rest.chars();
        let quote = chars.next()?;
        if !matches!(quote, '"' | '\'') {
            continue;
        }
        let title = chars.take_while(|ch| *ch != quote).collect::<String>();
        if !title.trim().is_empty() {
            return Some(title);
        }
    }
    None
}

fn unquoted_title_after_marker(objective: &str) -> Option<String> {
    let lower = objective.to_ascii_lowercase();
    for marker in TITLE_MARKERS {
        let Some(index) = lower.find(marker) else {
            continue;
        };
        let start = index + marker.len();
        let rest = objective.get(start..)?.trim_start_matches(marker_trim);
        let title = rest.split([',', '.', ';', ':']).next().unwrap_or("").trim();
        if !title.is_empty() {
            return Some(title.to_string());
        }
    }
    None
}

fn marker_trim(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, ':' | '-' | '=')
}

fn slug(title: &str) -> Option<String> {
    let words = title
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|word| !word.is_empty() && !TITLE_STOP_WORDS.contains(&word.as_str()))
        .take(5)
        .collect::<Vec<_>>();
    if !words.is_empty() {
        return Some(words.join("-"));
    }
    let trimmed = title.trim();
    (!trimmed.is_empty()).then(|| format!("title-{:x}", stable_hash(trimmed)))
}

fn stable_hash(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325, |hash, byte| {
            hash.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
        })
}

const TITLE_MARKERS: &[&str] = &[" named", " titled", " called", " title"];

const TITLE_STOP_WORDS: &[&str] = &[
    "a",
    "an",
    "the",
    "novel",
    "story",
    "book",
    "manuscript",
    "artifact",
];
