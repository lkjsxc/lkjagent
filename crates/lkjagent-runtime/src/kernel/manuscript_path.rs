pub(crate) fn requested_final_paths(root: &str, text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for token in text.split(|ch: char| !path_char(ch)) {
        let path = token.trim_matches('.');
        if path.starts_with(root)
            && path.contains("/manuscript/")
            && !path.contains("/manuscript/scenes/")
            && path.ends_with(".md")
        {
            push_unique(&mut out, path.to_string());
        }
    }
    out
}

pub(crate) fn first_manuscript_path(root: &str, text: &str) -> Option<String> {
    text.split(|ch: char| !path_char(ch))
        .map(|token| token.trim_matches('.'))
        .find_map(|path| normalize_manuscript_path(root, path))
}

pub(crate) fn default_final_path(root: &str, chapter_count: Option<usize>) -> String {
    let index = chapter_count.filter(|count| *count == 1).map_or(1, |_| 1);
    format!(
        "{}/manuscript/chapter-{index:02}.md",
        root.trim_end_matches('/')
    )
}

pub(crate) fn write_path_for(path: &str) -> String {
    if path.contains("/manuscript/scenes/") {
        path.to_string()
    } else {
        scene_path_for(path).unwrap_or_else(|| path.to_string())
    }
}

pub(crate) fn target_words(text: &str) -> Option<usize> {
    numbers_before(text, "word").into_iter().max()
}

pub(crate) fn chapter_count(text: &str) -> Option<usize> {
    numbers_before(text, "chapter").into_iter().max()
}

pub(crate) fn numbers_before(text: &str, unit: &str) -> Vec<usize> {
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

fn normalize_manuscript_path(root: &str, path: &str) -> Option<String> {
    if path.starts_with(root) && path.contains("/manuscript/") && path.ends_with(".md") {
        return Some(path.to_string());
    }
    if path.starts_with("manuscript/") && path.ends_with(".md") {
        return Some(format!("{}/{}", root.trim_end_matches('/'), path));
    }
    None
}

fn scene_path_for(path: &str) -> Option<String> {
    let (root, chapter) = path.rsplit_once("/manuscript/")?;
    let chapter = chapter.strip_suffix(".md")?;
    Some(format!("{root}/manuscript/scenes/{chapter}/scene-01.md"))
}

fn parse_number(value: &str) -> Option<usize> {
    match value.trim_matches(',') {
        "one" => Some(1),
        "ten" => Some(10),
        raw => raw.replace(',', "").parse().ok(),
    }
}

fn push_unique(out: &mut Vec<String>, value: String) {
    if !out.iter().any(|item| item == &value) {
        out.push(value);
    }
}

fn path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '/' | '-' | '_' | '.')
}
