use crate::kernel::decision::ContentWriteContract;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn content_contract_for(snapshot: &RuntimeSnapshot) -> Option<ContentWriteContract> {
    let root = snapshot.artifact.root.clone()?;
    let paths = contract_paths(snapshot, &root);
    Some(ContentWriteContract {
        root,
        paths,
        max_files: 20,
        max_file_bytes: 1_800,
        max_batch_bytes: 6_000,
        required_sections: required_sections(snapshot.artifact.kind.as_deref()),
        forbidden_weak_phrase_classes: vec![
            "scaffold-only".to_string(),
            "placeholder".to_string(),
            "owner-terms-only".to_string(),
            "generic-example".to_string(),
        ],
    })
}

fn contract_paths(snapshot: &RuntimeSnapshot, root: &str) -> Vec<String> {
    let parsed = snapshot
        .observation
        .latest
        .as_deref()
        .map(paths_from_observation);
    if let Some(paths) = parsed.filter(|paths| !paths.is_empty()) {
        return paths;
    }
    snapshot
        .artifact
        .weak_paths
        .iter()
        .map(|path| full_path(root, path))
        .collect()
}

fn paths_from_observation(text: &str) -> Vec<String> {
    let mut in_paths = false;
    let mut paths = Vec::new();
    for line in text.lines() {
        if line == "next_paths:" || line == "paths:" {
            in_paths = true;
            continue;
        }
        if in_paths && !line.starts_with("- ") {
            break;
        }
        if in_paths {
            paths.push(line.trim_start_matches("- ").to_string());
        }
    }
    paths
}

fn full_path(root: &str, path: &str) -> String {
    format!(
        "{}/{}",
        root.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

fn required_sections(kind: Option<&str>) -> Vec<String> {
    let values = match kind.unwrap_or("artifact").to_ascii_lowercase().as_str() {
        "story" => vec![
            "title",
            "purpose",
            "scene content or reference detail",
            "continuity notes",
            "verification notes",
        ],
        "cookbook" => vec![
            "title",
            "purpose",
            "ingredients or concept",
            "method or procedure",
            "timing, signals, and fixes",
            "verification notes",
        ],
        _ => vec!["title", "purpose", "concrete content", "verification notes"],
    };
    values.into_iter().map(str::to_string).collect()
}
