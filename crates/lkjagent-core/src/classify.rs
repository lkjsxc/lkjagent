use crate::model::TemplateId;

pub fn classify(objective: &str) -> TemplateId {
    let lower = objective.to_ascii_lowercase();
    if has_any(&lower, &["docs tree", "documentation", "docs/"]) {
        TemplateId::DocsTree
    } else if has_any(&lower, &["journal", "schedule", "todo", "task-list"]) {
        TemplateId::Journal
    } else if question(&lower) {
        TemplateId::Question
    } else if file_work(&lower) {
        TemplateId::FileWork
    } else {
        TemplateId::Generic
    }
}

pub fn instantiate(id: u64, objective: &str) -> crate::model::TaskSnapshot {
    crate::templates::instantiate(id, objective)
}

fn question(lower: &str) -> bool {
    lower.ends_with('?') || has_any(lower, &["what ", "why ", "how ", "which ", "where "])
}

fn file_work(lower: &str) -> bool {
    has_any(
        lower,
        &[
            "create ", "write ", "edit ", "revise ", "append ", "rewrite ",
        ],
    ) || !crate::templates::concrete_paths(lower).is_empty()
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
