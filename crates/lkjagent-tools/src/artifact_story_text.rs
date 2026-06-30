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

pub(crate) fn prose_words(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}

pub(crate) fn full_path(root: &str, path: &str) -> String {
    format!(
        "{}/{}",
        root.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

pub(crate) fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

pub(crate) fn path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '/' | '-' | '_' | '.')
}

fn parse_number(value: &str) -> Option<usize> {
    match value.trim_matches(',') {
        "one" => Some(1),
        "ten" => Some(10),
        raw => raw.replace(',', "").parse().ok(),
    }
}
