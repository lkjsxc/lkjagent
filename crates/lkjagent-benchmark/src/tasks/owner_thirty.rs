use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const DOC: &str = "# Topic\n\n## Purpose\n\nSemantic topic.\n";
const CATALOG: &str = "profile = \"thirty-docs\"\nroot = \"docs\"\n";

macro_rules! doc {
    ($path:literal) => {
        FileSpec {
            path: $path,
            content: DOC,
        }
    };
}

const PROMPT: &str = "\
Create 30 Markdown documentation files under docs/. Use semantic topical
groups and README indexes, not part files or numbered placeholder files.
";

const GOOD_FILES: &[FileSpec] = &[
    doc!("docs/README.md"),
    FileSpec {
        path: "docs/catalog.toml",
        content: CATALOG,
    },
    doc!("docs/overview/README.md"),
    doc!("docs/overview/purpose.md"),
    doc!("docs/overview/system-map.md"),
    doc!("docs/overview/audience.md"),
    doc!("docs/overview/constraints.md"),
    doc!("docs/overview/glossary-path.md"),
    doc!("docs/architecture/README.md"),
    doc!("docs/architecture/runtime.md"),
    doc!("docs/architecture/data-model.md"),
    doc!("docs/architecture/decisions.md"),
    doc!("docs/architecture/recovery.md"),
    doc!("docs/architecture/context.md"),
    doc!("docs/guides/README.md"),
    doc!("docs/guides/setup.md"),
    doc!("docs/guides/workflow.md"),
    doc!("docs/guides/troubleshooting.md"),
    doc!("docs/guides/documentation.md"),
    doc!("docs/guides/recovery.md"),
    doc!("docs/operations/README.md"),
    doc!("docs/operations/running.md"),
    doc!("docs/operations/verification.md"),
    doc!("docs/operations/recovery.md"),
    doc!("docs/operations/status.md"),
    doc!("docs/operations/logging.md"),
    doc!("docs/reference/README.md"),
    doc!("docs/reference/commands.md"),
    doc!("docs/reference/configuration.md"),
    doc!("docs/reference/token-ledger.md"),
    doc!("docs/reference/state-tracks.md"),
];

const BAD_SERIAL_FILES: &[FileSpec] = &[
    doc!("docs/README.md"),
    FileSpec {
        path: "docs/catalog.toml",
        content: CATALOG,
    },
    doc!("docs/part-001.md"),
    doc!("docs/part-002.md"),
];

const BAD_COUNT_FILES: &[FileSpec] = &[
    doc!("docs/README.md"),
    FileSpec {
        path: "docs/catalog.toml",
        content: CATALOG,
    },
    doc!("docs/overview/README.md"),
    doc!("docs/overview/purpose.md"),
];

const GOOD: &[Fixture] = &[Fixture {
    name: "thirty-semantic-docs",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "serial-numbered",
        files: BAD_SERIAL_FILES,
    },
    Fixture {
        name: "wrong-count",
        files: BAD_COUNT_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-docs-thirty-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "docs", "counted-structure"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::ThirtySemanticDocs,
    seed: 8003,
    points: 1,
    timeout_seconds: 120,
};
