use super::model::{ScaffoldInput, ScaffoldProfile};
pub use super::shape_profiles::shape;

pub fn select_profile(input: &ScaffoldInput) -> ScaffoldProfile {
    let text = format!(
        "{} {} {}",
        input.kind,
        input.title,
        input.sections.join(" ")
    )
    .to_ascii_lowercase();
    if lkjagent_seed_subject(&text) {
        ScaffoldProfile::LkjagentSemanticSeed
    } else if bread_subject(&text) {
        ScaffoldProfile::BreadCookbook
    } else if text.contains("cookbook") || text.contains("recipe") {
        ScaffoldProfile::Cookbook
    } else if creative_subject(&text, &input.kind) {
        ScaffoldProfile::NarrativeManuscript
    } else if text.contains("knowledge") || text.contains("encyclopedia") {
        ScaffoldProfile::KnowledgeBase
    } else if text.contains("plan") || text.contains("implementation") {
        ScaffoldProfile::ImplementationPlan
    } else if text.contains("report") || text.contains("research") {
        ScaffoldProfile::ResearchReport
    } else if text.contains("guide") || text.contains("manual") {
        ScaffoldProfile::UserGuide
    } else if text.contains("runbook") || text.contains("operations") {
        ScaffoldProfile::OperationsRunbook
    } else if text.contains("architecture") {
        ScaffoldProfile::ArchitectureDocs
    } else if input.count.is_none() && relation_first_subject(&text) {
        ScaffoldProfile::GenericStructuredDocs
    } else {
        ScaffoldProfile::ProjectDocs
    }
}

fn lkjagent_seed_subject(text: &str) -> bool {
    text.contains("lkjagent")
        && (text.contains("model")
            || text.contains("endpoint")
            || text.contains("qwen")
            || text.contains("rust")
            || text.contains("asia")
            || text.contains("minecraft")
            || text.contains("factorio"))
}

fn bread_subject(text: &str) -> bool {
    ["bread", "sourdough", "ciabatta", "focaccia", "rye loaf"]
        .iter()
        .any(|needle| text.contains(needle))
}

fn creative_subject(text: &str, kind: &str) -> bool {
    let kind = kind.to_ascii_lowercase();
    let explicit_kind = [
        "story",
        "novel",
        "manuscript",
        "fiction",
        "character-profile",
        "setting",
        "outline",
        "scene",
    ]
    .iter()
    .any(|needle| kind.contains(needle));
    let title_signal = [
        "novel",
        "sf ",
        "sci-fi",
        "science fiction",
        "character",
        "setting",
        "manuscript",
        "fiction",
    ]
    .iter()
    .any(|needle| text.contains(needle));
    explicit_kind || (title_signal && !technical_kind(&kind))
}

fn technical_kind(kind: &str) -> bool {
    matches!(kind, "architecture" | "runbook" | "guide" | "operations")
}

fn relation_first_subject(text: &str) -> bool {
    text.contains(',') || topic_word_count(text) > 4
}

fn topic_word_count(text: &str) -> usize {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .filter(|word| !matches!(*word, "a" | "an" | "and" | "the" | "to" | "of" | "for"))
        .count()
}
