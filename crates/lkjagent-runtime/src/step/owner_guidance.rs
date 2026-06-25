use lkjagent_graph::case_document::DocumentState;

use crate::task::RuntimeState;

pub(super) fn apply_owner_guidance(state: &mut RuntimeState, content: &str) {
    let root =
        labeled_value(content, &["Root directory:", "Root:"]).filter(|root| valid_root(root));
    let kind = labeled_value(content, &["Artifact kind:"]);
    let Some(graph) = state.graph.as_mut() else {
        return;
    };
    if root.is_none() && kind.is_none() {
        return;
    }
    match graph.document.as_mut() {
        Some(document) => {
            if let Some(root) = root {
                document.root = root;
            }
            if let Some(kind) = kind {
                document.kind = kind;
            }
        }
        None => {
            if let Some(root) = root {
                graph.document = Some(DocumentState::planned(
                    root,
                    kind.unwrap_or_else(|| "documentation".to_string()),
                ));
            }
        }
    }
}

fn labeled_value(content: &str, labels: &[&str]) -> Option<String> {
    content.lines().find_map(|line| {
        let trimmed = line.trim().trim_matches('`');
        labels
            .iter()
            .find_map(|label| trimmed.strip_prefix(label).map(clean_value))
    })
}

fn clean_value(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_end_matches('.')
        .trim()
        .to_string()
}

fn valid_root(root: &str) -> bool {
    !root.is_empty() && !root.ends_with(".md") && !root.contains("..") && !root.starts_with('/')
}
