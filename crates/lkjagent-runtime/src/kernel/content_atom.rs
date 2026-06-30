use crate::kernel::obligation_parse::line_value;
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ContentAtomFacts {
    pub active: bool,
    pub status: Option<String>,
    pub missing_count: usize,
    pub next_atom: Option<String>,
    pub required_atoms: Vec<String>,
}

pub(crate) fn facts_from_snapshot(snapshot: &RuntimeSnapshot) -> ContentAtomFacts {
    let Some(text) = latest_text(snapshot) else {
        return ContentAtomFacts::default();
    };
    let missing_count = line_value(text, "atom_missing_count")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let required_atoms = line_value(text, "required_atoms")
        .map(|value| split_list(&value))
        .unwrap_or_default();
    let status = line_value(text, "atom_status");
    let active = status.is_some() || !required_atoms.is_empty();
    ContentAtomFacts {
        active,
        status,
        missing_count,
        next_atom: line_value(text, "next_atom").filter(|value| value != "none"),
        required_atoms,
    }
}

pub(crate) fn generic_root_conflict(snapshot: &RuntimeSnapshot) -> Option<String> {
    let root = snapshot.artifact.root.as_deref()?;
    if !generic_root(root) {
        return None;
    }
    let text = snapshot
        .case
        .owner_objective
        .as_deref()
        .or(snapshot.case.normalized_objective.as_deref())?;
    target_path(text).map(|target| format!("selected_root={root} requested_target={target}"))
}

fn latest_text(snapshot: &RuntimeSnapshot) -> Option<&str> {
    snapshot
        .observation
        .latest
        .as_deref()
        .or(snapshot.observation.latest_successful.as_deref())
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty() && *item != "none")
        .map(str::to_string)
        .collect()
}

fn generic_root(root: &str) -> bool {
    let lower = root.trim_start_matches("./").to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "structured-output" | "output" | "artifact" | "work-product"
    ) || lower.starts_with("structured-output/")
        || lower.starts_with("output/")
}

fn target_path(text: &str) -> Option<String> {
    text.split(|ch: char| !path_char(ch))
        .map(|token| token.trim_matches('.'))
        .find(|token| target_root(token))
        .map(str::to_string)
}

fn target_root(token: &str) -> bool {
    [
        "stories/",
        "reports/",
        "guides/",
        "cookbooks/",
        "dictionaries/",
        "knowledge/",
    ]
    .iter()
    .any(|prefix| token.starts_with(prefix))
        || token.starts_with("docs/") && token.ends_with(".md")
}

fn path_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '/' | '-' | '_' | '.')
}
