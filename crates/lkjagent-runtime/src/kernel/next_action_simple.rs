use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn artifact_work_required(snapshot: &RuntimeSnapshot) -> bool {
    if snapshot.artifact.root.is_some() {
        return true;
    }
    let family = snapshot.case.task_family.as_deref().unwrap_or_default();
    matches!(family, "documentation" | "knowledge-base") || objective_names_artifact(snapshot)
}

pub(crate) fn simple_write_path(snapshot: &RuntimeSnapshot) -> Option<String> {
    let objective = snapshot.case.owner_objective.as_deref()?;
    objective
        .split_whitespace()
        .map(clean_path_token)
        .find(|token| looks_like_file_path(token))
}

pub(crate) fn simple_write_body(path: &str, snapshot: &RuntimeSnapshot) -> String {
    let content = if objective_contains(snapshot, "hello") {
        "Hello."
    } else {
        "Done."
    };
    format!(
        "<action>\n<tool>fs.write</tool>\n<path>{path}</path>\n<content>{content}</content>\n</action>"
    )
}

fn objective_names_artifact(snapshot: &RuntimeSnapshot) -> bool {
    [
        "novel",
        "story",
        "cookbook",
        "dictionary",
        "guide",
        "documentation corpus",
        "knowledge",
        "encyclopedia",
    ]
    .iter()
    .any(|needle| objective_contains(snapshot, needle))
}

fn objective_contains(snapshot: &RuntimeSnapshot, needle: &str) -> bool {
    snapshot
        .case
        .owner_objective
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .contains(needle)
}

fn clean_path_token(token: &str) -> String {
    token
        .trim_matches(|ch: char| matches!(ch, '`' | '\'' | '"' | ',' | ';' | ':' | '.' | ')' | '('))
        .to_string()
}

fn looks_like_file_path(token: &str) -> bool {
    [
        ".md", ".txt", ".sh", ".rs", ".toml", ".json", ".yaml", ".yml",
    ]
    .iter()
    .any(|suffix| token.ends_with(suffix))
}
