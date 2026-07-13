pub(crate) fn short_slug(path: &str) -> Option<&str> {
    path.strip_prefix("artifacts/reports/")
        .and_then(|value| value.strip_suffix(".md"))
        .filter(|value| short_semantic_slug(value))
}

pub(crate) fn map_path(path: &str) -> Option<&str> {
    let tail = path.strip_prefix("artifacts/documents/")?;
    let (slug, rest) = tail.split_once('/')?;
    (rest == "README.md" && semantic_part(slug)).then_some(slug)
}

pub(crate) fn child_path(path: &str) -> Option<(&str, &str)> {
    let tail = path.strip_prefix("artifacts/documents/")?;
    let (slug, rest) = tail.split_once('/')?;
    let unit = rest.strip_suffix(".md")?;
    (semantic_part(slug) && semantic_part(unit) && unit != "index" && unit != "README")
        .then_some((slug, unit))
}

pub(crate) fn member_path(slug: &str, unit: &str) -> String {
    format!("artifacts/documents/{slug}/{unit}.md")
}

pub(crate) fn semantic_part(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && value.bytes().any(|byte| byte.is_ascii_lowercase())
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value
            .strip_prefix("part-")
            .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
}

fn short_semantic_slug(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
}
