use std::path::Path;

const MAX_STEM_LEN: usize = 40;
const MAX_SEGMENT_LEN: usize = 48;
const MAX_REL_PATH_LEN: usize = 120;

pub fn slug(role: &str) -> String {
    let mut words = role
        .to_ascii_lowercase()
        .replace('&', " and ")
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .filter(|word| !stopword(word))
        .map(str::to_string)
        .collect::<Vec<_>>();
    if words.is_empty() {
        words.push("overview".to_string());
    }
    bounded_stem(&words)
}

pub fn join_root(root: &str, child: &str) -> String {
    if root == "." {
        child.to_string()
    } else {
        format!("{}/{}", root.trim_end_matches('/'), child)
    }
}

pub fn link_target(child: &str, is_dir: bool) -> String {
    if is_dir {
        format!("{child}/README.md")
    } else {
        child.to_string()
    }
}

pub fn title_from_path(path: &str) -> String {
    let stem = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Document");
    stem.split('-')
        .filter(|word| !word.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn forbidden_serial_name(path: &str) -> bool {
    let Some(name) = Path::new(path).file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    let stem = lower.trim_end_matches(".md");
    ["part", "section", "chapter", "file", "doc"]
        .iter()
        .any(|prefix| ordinal_suffix(stem, prefix))
}

pub fn path_hygiene_failures(relative: &str) -> Vec<String> {
    let mut failures = Vec::new();
    if relative.len() > MAX_REL_PATH_LEN {
        failures.push(format!("path_too_long: {relative}"));
    }
    for segment in relative.split('/') {
        if segment.len() > MAX_SEGMENT_LEN {
            failures.push(format!("path_segment_too_long: {relative}"));
            break;
        }
    }
    if let Some(stem) = markdown_stem(relative) {
        if stem.len() > MAX_STEM_LEN {
            failures.push(format!("markdown_stem_too_long: {relative}"));
        }
        if multi_topic_stem(stem) {
            failures.push(format!("multi_topic_slug: {relative}"));
        }
    }
    failures
}

pub fn banned_release_wording(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    for word in lower.split(|character: char| !character.is_ascii_alphanumeric()) {
        if release_tag(word) {
            return Some(word.to_string());
        }
    }
    if lower.contains(&format!("{}ed api", concat!("ver", "sion"))) {
        return Some("release-shaped api wording".to_string());
    }
    None
}

fn bounded_stem(words: &[String]) -> String {
    let mut selected = Vec::new();
    for word in words.iter().take(5) {
        let next = if selected.is_empty() {
            word.clone()
        } else {
            format!("{}-{word}", selected.join("-"))
        };
        if next.len() > MAX_STEM_LEN {
            break;
        }
        selected.push(word.clone());
    }
    if selected.is_empty() {
        return format!("{}-{:04x}", trim_word(&words[0]), hash16(words));
    }
    selected.join("-")
}

fn trim_word(word: &str) -> String {
    word.chars().take(24).collect()
}

fn hash16(words: &[String]) -> u16 {
    let mut hash: u32 = 0x811c9dc5;
    for byte in words.join("-").bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    (hash & 0xffff) as u16
}

fn stopword(word: &str) -> bool {
    matches!(
        word,
        "a" | "an" | "and" | "the" | "to" | "of" | "for" | "with" | "about" | "into"
    )
}

fn markdown_stem(relative: &str) -> Option<&str> {
    Path::new(relative)
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".md"))
}

fn multi_topic_stem(stem: &str) -> bool {
    stem.split('-').filter(|part| !stopword(part)).count() > 5
}

fn ordinal_suffix(stem: &str, prefix: &str) -> bool {
    let Some(rest) = stem.strip_prefix(prefix) else {
        return false;
    };
    let rest = rest.trim_start_matches(['-', '_']);
    !rest.is_empty() && rest.chars().all(|character| character.is_ascii_digit())
}

fn release_tag(word: &str) -> bool {
    let mut chars = word.chars();
    matches!(chars.next(), Some('v'))
        && chars.clone().next().is_some()
        && chars.all(|character| character.is_ascii_digit())
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}
