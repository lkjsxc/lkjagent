use super::model::{ScaffoldProfile, ShapeGroup};

#[rustfmt::skip]
pub const PROJECT: &[ShapeGroup] = &[
    group("overview", "Overview", "overview index", &["purpose", "system map"]),
    group("architecture", "Architecture", "architecture index", &["runtime", "data model", "decisions"]),
    group("guides", "Guides", "guide index", &["setup", "workflow", "troubleshooting"]),
    group("operations", "Operations", "operations index", &["running", "verification", "recovery"]),
    group("reference", "Reference", "reference index", &["glossary", "commands", "configuration"]),
];

#[rustfmt::skip]
pub const KNOWLEDGE: &[ShapeGroup] = &[
    group("concepts", "Concepts", "concept index", &["core model", "terminology"]),
    group("maps", "Maps", "map index", &["topic map", "dependency map", "evidence map"]),
    group("notes", "Notes", "notes index", &["findings", "unresolved questions"]),
    group("synthesis", "Synthesis", "synthesis index", &["principles", "implications"]),
    group("reference", "Reference", "reference index", &["sources", "glossary"]),
];

#[rustfmt::skip]
pub const PLAN: &[ShapeGroup] = &[
    group("diagnosis", "Diagnosis", "diagnosis index", &["current state", "failure modes"]),
    group("design", "Design", "design index", &["target architecture", "data model", "state transitions"]),
    group("tasks", "Tasks", "task index", &["immediate", "implementation", "verification"]),
    group("acceptance", "Acceptance", "acceptance index", &["gates", "test cases", "handoff"]),
];

#[rustfmt::skip]
pub const REPORT: &[ShapeGroup] = &[
    group("summary", "Summary", "summary index", &["executive summary", "key findings"]),
    group("analysis", "Analysis", "analysis index", &["evidence", "interpretation", "risks"]),
    group("recommendations", "Recommendations", "recommendations index", &["immediate actions", "long term actions"]),
    group("appendices", "Appendices", "appendix index", &["assumptions", "glossary"]),
];

#[rustfmt::skip]
pub const MANUSCRIPT: &[ShapeGroup] = &[
    group("planning", "Planning", "planning index", &["premise", "cast", "world", "outline"]),
    group("chapters", "Chapters", "chapter index", &[
        "waking pod", "cylinder pulse", "laboratory of light", "memory market",
        "first theft", "mirror in the vial", "final choice", "world after memory",
    ]),
    group("revision", "Revision", "revision index", &["style guide", "continuity checks"]),
];

#[rustfmt::skip]
pub const COOKBOOK: &[ShapeGroup] = &[
    group("foundations", "Foundations", "foundation index", &[
        "flour water salt yeast", "kneading", "fermentation", "shaping", "baking",
    ]),
    group("recipes", "Recipes", "recipe index", &[
        "sourdough country loaf", "ciabatta", "focaccia", "rye bread", "milk bread",
    ]),
    group("reference", "Reference", "reference index", &["troubleshooting", "equipment", "timelines"]),
];

pub fn shape(profile: ScaffoldProfile) -> &'static [ShapeGroup] {
    match profile {
        ScaffoldProfile::KnowledgeBase => KNOWLEDGE,
        ScaffoldProfile::ImplementationPlan => PLAN,
        ScaffoldProfile::ResearchReport => REPORT,
        ScaffoldProfile::NarrativeManuscript => MANUSCRIPT,
        ScaffoldProfile::Cookbook => COOKBOOK,
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
