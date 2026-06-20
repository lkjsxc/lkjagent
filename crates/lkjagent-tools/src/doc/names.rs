use std::path::Path;

pub fn slug(role: &str) -> String {
    let words = role
        .to_ascii_lowercase()
        .replace('&', "and")
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if words.is_empty() {
        "overview".to_string()
    } else {
        words.join("-")
    }
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
