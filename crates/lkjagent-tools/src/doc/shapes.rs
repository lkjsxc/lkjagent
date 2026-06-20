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
    if text.contains("cookbook") || text.contains("recipe") || text.contains("bread") {
        ScaffoldProfile::Cookbook
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
    } else if text.contains("manuscript") || text.contains("story") {
        ScaffoldProfile::NarrativeManuscript
    } else if text.contains("architecture") {
        ScaffoldProfile::ArchitectureDocs
    } else {
        ScaffoldProfile::ProjectDocs
    }
}
