use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn admitted(values: &BTreeMap<String, String>) -> bool {
    let get = |name: &str| values.get(name).map(String::as_str);
    let options = (
        get("slug"),
        get("unit"),
        get("children"),
        get("minimum_words"),
    );
    match (get("family"), options) {
        (Some("journal"), (None, None, None, None)) => true,
        (Some("memory"), (None, None, None, None)) => title_slug(values),
        (Some("report"), (None, None, None, None)) => title_slug(values),
        (Some("report"), (Some(slug), Some("index"), Some(children), Some(words))) => {
            canonical(slug) && child_set(children) && bounded_words(words)
        }
        (Some("report"), (Some(slug), Some(unit), None, None)) => {
            canonical(slug) && unit != "index" && canonical(unit)
        }
        _ => false,
    }
}

fn title_slug(values: &BTreeMap<String, String>) -> bool {
    values
        .get("title")
        .is_some_and(|title| title.bytes().any(|byte| byte.is_ascii_alphanumeric()))
}

fn child_set(value: &str) -> bool {
    let children = value.split(',').map(str::trim).collect::<Vec<_>>();
    let unique = children.iter().copied().collect::<BTreeSet<_>>();
    children.len() >= 2
        && unique.len() == children.len()
        && children
            .iter()
            .all(|child| *child != "index" && canonical(child))
}

fn bounded_words(value: &str) -> bool {
    value
        .parse::<u32>()
        .is_ok_and(|words| (1..=20_000).contains(&words))
}

fn canonical(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && value.bytes().any(|byte| byte.is_ascii_lowercase())
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !anonymous(value)
}

fn anonymous(value: &str) -> bool {
    ["part-", "section-"].iter().any(|prefix| {
        value
            .strip_prefix(prefix)
            .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit()))
    })
}
