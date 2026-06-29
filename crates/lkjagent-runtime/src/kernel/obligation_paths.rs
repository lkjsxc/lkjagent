pub(crate) fn contract_paths(text: &str, root: &str) -> Vec<String> {
    let Some((label, rest)) = path_block(text) else {
        return Vec::new();
    };
    rest.lines()
        .map(str::trim)
        .take_while(|line| line.starts_with("- "))
        .map(|line| line.trim_start_matches("- "))
        .map(|path| path_for_label(label, root, path))
        .collect()
}

pub(crate) fn full_path(root: &str, path: &str) -> String {
    if path.starts_with(root) {
        path.to_string()
    } else {
        format!(
            "{}/{}",
            root.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

pub(crate) fn identity_paths(root: &str, kind: &str) -> Vec<String> {
    identity_leaves(kind)
        .into_iter()
        .map(|path| full_path(root, path))
        .collect()
}

pub(crate) fn required_sections(kind: &str) -> Vec<String> {
    let values = match kind.to_ascii_lowercase().as_str() {
        "cookbook" => vec![
            "title",
            "purpose",
            "ingredients or concept",
            "method or procedure",
            "timing, signals, and fixes",
            "verification notes",
        ],
        "story" => vec![
            "title",
            "purpose",
            "scene content or reference detail",
            "continuity notes",
            "verification notes",
        ],
        _ => vec!["title", "purpose", "concrete content", "verification notes"],
    };
    values.into_iter().map(str::to_string).collect()
}

pub(crate) fn weak_phrase_classes() -> Vec<String> {
    [
        "scaffold-only",
        "placeholder",
        "owner-terms-only",
        "generic-example",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn identity_leaves(kind: &str) -> Vec<&'static str> {
    if kind.eq_ignore_ascii_case("story") {
        vec![
            "catalog.toml",
            "README.md",
            "objective.md",
            "setting-overview.md",
            "cast.md",
        ]
    } else {
        vec![
            "catalog.toml",
            "README.md",
            "objective.md",
            "overview.md",
            "verification-notes.md",
        ]
    }
}

fn path_block(text: &str) -> Option<(&'static str, &str)> {
    for label in ["paths:", "next_paths:"] {
        if let Some((_, rest)) = text.split_once(label) {
            return Some((label, rest.trim_start_matches('\n')));
        }
    }
    None
}

fn path_for_label(label: &str, root: &str, path: &str) -> String {
    if label == "next_paths:" {
        full_path(root, path)
    } else {
        path.to_string()
    }
}
