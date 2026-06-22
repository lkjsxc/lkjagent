#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldProfile {
    ProjectDocs,
    ArchitectureDocs,
    AgentManual,
    KnowledgeBase,
    ResearchReport,
    ImplementationPlan,
    UserGuide,
    OperationsRunbook,
    EvaluationSuite,
    NarrativeManuscript,
    Cookbook,
    BreadCookbook,
    GenericStructuredDocs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldMode {
    Exact,
    Approx,
}

impl ScaffoldMode {
    pub fn parse(value: &str) -> Self {
        if value.trim().eq_ignore_ascii_case("exact") {
            Self::Exact
        } else {
            Self::Approx
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approx => "approx",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldInput {
    pub root: String,
    pub kind: String,
    pub count: Option<usize>,
    pub mode: ScaffoldMode,
    pub title: String,
    pub sections: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedFile {
    pub path: String,
    pub title: String,
    pub role: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldPlan {
    pub root: String,
    pub profile: ScaffoldProfile,
    pub files: Vec<PlannedFile>,
}

impl ScaffoldPlan {
    pub fn markdown_count(&self) -> usize {
        self.files
            .iter()
            .filter(|file| file.path.ends_with(".md"))
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeGroup {
    pub dir: &'static str,
    pub title: &'static str,
    pub role: &'static str,
    pub leaves: &'static [&'static str],
}
