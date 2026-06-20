use super::model::{ScaffoldInput, ScaffoldProfile, ShapeGroup};

pub const PROJECT: &[ShapeGroup] = &[
    group(
        "overview",
        "Overview",
        "overview index",
        &["purpose", "system map"],
    ),
    group(
        "architecture",
        "Architecture",
        "architecture index",
        &["runtime", "data model", "decisions"],
    ),
    group(
        "guides",
        "Guides",
        "guide index",
        &["setup", "workflow", "troubleshooting"],
    ),
    group(
        "operations",
        "Operations",
        "operations index",
        &["running", "verification", "recovery"],
    ),
    group(
        "reference",
        "Reference",
        "reference index",
        &["glossary", "commands", "configuration"],
    ),
];
pub const KNOWLEDGE: &[ShapeGroup] = &[
    group(
        "concepts",
        "Concepts",
        "concept index",
        &["core model", "terminology"],
    ),
    group(
        "maps",
        "Maps",
        "map index",
        &["topic map", "dependency map", "evidence map"],
    ),
    group(
        "notes",
        "Notes",
        "notes index",
        &["findings", "unresolved questions"],
    ),
    group(
        "synthesis",
        "Synthesis",
        "synthesis index",
        &["principles", "implications"],
    ),
    group(
        "reference",
        "Reference",
        "reference index",
        &["sources", "glossary"],
    ),
];
pub const PLAN: &[ShapeGroup] = &[
    group(
        "diagnosis",
        "Diagnosis",
        "diagnosis index",
        &["current state", "failure modes"],
    ),
    group(
        "design",
        "Design",
        "design index",
        &["target architecture", "data model", "state transitions"],
    ),
    group(
        "tasks",
        "Tasks",
        "task index",
        &["immediate", "implementation", "verification"],
    ),
    group(
        "acceptance",
        "Acceptance",
        "acceptance index",
        &["gates", "test cases", "handoff"],
    ),
];
pub const REPORT: &[ShapeGroup] = &[
    group(
        "summary",
        "Summary",
        "summary index",
        &["executive summary", "key findings"],
    ),
    group(
        "analysis",
        "Analysis",
        "analysis index",
        &["evidence", "interpretation", "risks"],
    ),
    group(
        "recommendations",
        "Recommendations",
        "recommendations index",
        &["immediate actions", "long term actions"],
    ),
    group(
        "appendices",
        "Appendices",
        "appendix index",
        &["assumptions", "glossary"],
    ),
];
pub const MANUSCRIPT: &[ShapeGroup] = &[
    group(
        "planning",
        "Planning",
        "planning index",
        &["premise", "outline", "continuity"],
    ),
    group(
        "manuscript",
        "Manuscript",
        "manuscript index",
        &[
            "chapter arc setup",
            "chapter arc conflict",
            "chapter arc resolution",
        ],
    ),
    group(
        "revision",
        "Revision",
        "revision index",
        &["style guide", "continuity checks"],
    ),
];

pub fn select_profile(input: &ScaffoldInput) -> ScaffoldProfile {
    let text = format!("{} {}", input.kind, input.sections.join(" ")).to_ascii_lowercase();
    if text.contains("knowledge") || text.contains("encyclopedia") {
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

pub fn shape(profile: ScaffoldProfile) -> &'static [ShapeGroup] {
    match profile {
        ScaffoldProfile::KnowledgeBase => KNOWLEDGE,
        ScaffoldProfile::ImplementationPlan => PLAN,
        ScaffoldProfile::ResearchReport => REPORT,
        ScaffoldProfile::NarrativeManuscript => MANUSCRIPT,
        _ => PROJECT,
    }
}

const fn group(
    dir: &'static str,
    title: &'static str,
    role: &'static str,
    leaves: &'static [&'static str],
) -> ShapeGroup {
    ShapeGroup {
        dir,
        title,
        role,
        leaves,
    }
}
