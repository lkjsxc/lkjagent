#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFamily {
    Arithmetic,
    Graph,
    Automata,
    ProgramSynthesis,
    ProgramRepair,
    MultiFile,
    StatefulCorrection,
    OwnerReliability,
}

impl TaskFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Arithmetic => "arithmetic",
            Self::Graph => "graph",
            Self::Automata => "automata",
            Self::ProgramSynthesis => "program-synthesis",
            Self::ProgramRepair => "program-repair",
            Self::MultiFile => "multi-file",
            Self::StatefulCorrection => "stateful-correction",
            Self::OwnerReliability => "owner-reliability",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Tiny,
    Small,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Small => "small",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JudgeKind {
    Crt,
    ShortestPath,
    EvenOnesDfa,
    FibonacciShell,
    RepairRankShell,
    ReadmeBundle,
    CorrectedComposites,
    SemanticProjectDocs,
    RecursiveDocTree,
    ThirtySemanticDocs,
    MultiTopicDocumentation,
    GraphStateParamRecovery,
    DocScaffoldParamRecovery,
    RecoveryLoopLongStory,
    GraphPlanExample,
    GraphTransitionTarget,
    MemoryFtsQuery,
    MaintenanceMemoryDuplicate,
    PolicyContradiction,
    GraphNoteKindRecovery,
    BreadCookbookArtifact,
    UploadedRunFixtures,
    StatusAccounting,
    ModelHandoffLog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileSpec {
    pub path: &'static str,
    pub content: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fixture {
    pub name: &'static str,
    pub files: &'static [FileSpec],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkTask {
    pub id: &'static str,
    pub suite: &'static str,
    pub family: TaskFamily,
    pub difficulty: Difficulty,
    pub tags: &'static [&'static str],
    pub prompt: &'static str,
    pub follow_up: Option<&'static str>,
    pub starter_files: &'static [FileSpec],
    pub good: &'static [Fixture],
    pub bad: &'static [Fixture],
    pub judge: JudgeKind,
    pub seed: u64,
    pub points: u16,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgeOutcome {
    pub passed: bool,
    pub points_earned: u16,
    pub points_possible: u16,
    pub reason: String,
}

impl JudgeOutcome {
    pub fn pass(points: u16) -> Self {
        Self {
            passed: true,
            points_earned: points,
            points_possible: points,
            reason: "ok".to_string(),
        }
    }

    pub fn fail(points: u16, reason: impl Into<String>) -> Self {
        Self {
            passed: false,
            points_earned: 0,
            points_possible: points,
            reason: reason.into(),
        }
    }
}
