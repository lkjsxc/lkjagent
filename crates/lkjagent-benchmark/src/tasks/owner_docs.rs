use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const DOC: &str = "# Topic\n\n## Purpose\n\nSemantic topic.\n";
const GRAPH: &str = "# Document Graph\n\n## Nodes\n\n| id | path | role | status |\n| --- | --- | --- | --- |\n| root | README.md | root index | scaffolded |\n\n## Edges\n\n| from | to | kind | reason |\n| --- | --- | --- | --- |\n| root | overview | indexes | local map |\n\n## Coverage\n\n| owner requirement | covered by | status |\n| --- | --- | --- |\n| semantic docs | docs | satisfied |\n";

const PROJECT_PROMPT: &str = "\
Create documentation for this project under docs/. Use semantic recursive
docs, README indexes, and a graph manifest. Do not create part files.
";

const PROJECT_GOOD_FILES: &[FileSpec] = &[
    FileSpec { path: "docs/README.md", content: "# Docs\n\n## Purpose\n\nRoot.\n\n## Local Map\n\n- [overview](overview/README.md): overview.\n- [architecture](architecture/README.md): architecture.\n- [guides](guides/README.md): guides.\n- [operations](operations/README.md): operations.\n- [reference](reference/README.md): reference.\n" },
    FileSpec { path: "docs/.lkj-doc-graph.md", content: GRAPH },
    FileSpec { path: "docs/overview/README.md", content: "# Overview\n\n## Purpose\n\nOverview.\n\n- [purpose.md](purpose.md)\n" },
    FileSpec { path: "docs/overview/purpose.md", content: DOC },
    FileSpec { path: "docs/architecture/README.md", content: "# Architecture\n\n## Purpose\n\nArchitecture.\n\n- [runtime.md](runtime.md)\n" },
    FileSpec { path: "docs/architecture/runtime.md", content: DOC },
    FileSpec { path: "docs/guides/README.md", content: "# Guides\n\n## Purpose\n\nGuides.\n\n- [setup.md](setup.md)\n" },
    FileSpec { path: "docs/guides/setup.md", content: DOC },
    FileSpec { path: "docs/operations/README.md", content: "# Operations\n\n## Purpose\n\nOperations.\n\n- [running.md](running.md)\n" },
    FileSpec { path: "docs/operations/running.md", content: DOC },
    FileSpec { path: "docs/reference/README.md", content: "# Reference\n\n## Purpose\n\nReference.\n\n- [glossary.md](glossary.md)\n" },
    FileSpec { path: "docs/reference/glossary.md", content: DOC },
];

const PROJECT_BAD_SERIAL: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: "# Docs\n\n## Purpose\n\nRoot.\n\n- [part](part-001.md)\n",
    },
    FileSpec {
        path: "docs/.lkj-doc-graph.md",
        content: GRAPH,
    },
    FileSpec {
        path: "docs/part-001.md",
        content: DOC,
    },
];

const PROJECT_BAD_FLAT: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: "# Docs\n\n## Purpose\n\nFlat.\n",
    },
    FileSpec {
        path: "docs/overview.md",
        content: DOC,
    },
];

const PROJECT_GOOD: &[Fixture] = &[Fixture {
    name: "semantic-project",
    files: PROJECT_GOOD_FILES,
}];

const PROJECT_BAD: &[Fixture] = &[
    Fixture {
        name: "serial-parts",
        files: PROJECT_BAD_SERIAL,
    },
    Fixture {
        name: "flat-no-graph",
        files: PROJECT_BAD_FLAT,
    },
];

pub const PROJECT_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-docs-project-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "docs", "semantic-tree"],
    prompt: PROJECT_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: PROJECT_GOOD,
    bad: PROJECT_BAD,
    judge: JudgeKind::SemanticProjectDocs,
    seed: 8001,
    points: 1,
    timeout_seconds: 120,
};

const RECURSIVE_PROMPT: &str = "\
Create recursive documentation under docs/. Every directory needs README.md,
local README links, and a compact .lkj-doc-graph.md manifest.
";

const RECURSIVE_GOOD_FILES: &[FileSpec] = &[
    FileSpec { path: "docs/README.md", content: "# Docs\n\n## Purpose\n\nRoot.\n\n- [architecture](architecture/README.md)\n" },
    FileSpec { path: "docs/.lkj-doc-graph.md", content: GRAPH },
    FileSpec { path: "docs/architecture/README.md", content: "# Architecture\n\n## Purpose\n\nArchitecture.\n\n- [runtime](runtime/README.md)\n- [decisions.md](decisions.md)\n" },
    FileSpec { path: "docs/architecture/decisions.md", content: DOC },
    FileSpec { path: "docs/architecture/runtime/README.md", content: "# Runtime\n\n## Purpose\n\nRuntime.\n\n- [state.md](state.md)\n" },
    FileSpec { path: "docs/architecture/runtime/state.md", content: DOC },
];

const RECURSIVE_BAD_FLAT: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: "# Docs\n\n## Purpose\n\nRoot.\n\n- [architecture.md](architecture.md)\n",
    },
    FileSpec {
        path: "docs/.lkj-doc-graph.md",
        content: GRAPH,
    },
    FileSpec {
        path: "docs/architecture.md",
        content: DOC,
    },
];

const RECURSIVE_BAD_LINK: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: "# Docs\n\n## Purpose\n\nRoot.\n\n- [architecture](architecture/README.md)\n",
    },
    FileSpec {
        path: "docs/.lkj-doc-graph.md",
        content: GRAPH,
    },
    FileSpec {
        path: "docs/architecture/README.md",
        content: "# Architecture\n\n## Purpose\n\nArchitecture.\n",
    },
    FileSpec {
        path: "docs/architecture/runtime/README.md",
        content: "# Runtime\n\n## Purpose\n\nRuntime.\n\n- [state.md](state.md)\n",
    },
    FileSpec {
        path: "docs/architecture/runtime/state.md",
        content: DOC,
    },
];

const RECURSIVE_GOOD: &[Fixture] = &[Fixture {
    name: "recursive-indexed",
    files: RECURSIVE_GOOD_FILES,
}];

const RECURSIVE_BAD: &[Fixture] = &[
    Fixture {
        name: "flat",
        files: RECURSIVE_BAD_FLAT,
    },
    Fixture {
        name: "missing-local-link",
        files: RECURSIVE_BAD_LINK,
    },
];

pub const RECURSIVE_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-docs-recursive-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "docs", "recursive"],
    prompt: RECURSIVE_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: RECURSIVE_GOOD,
    bad: RECURSIVE_BAD,
    judge: JudgeKind::RecursiveDocTree,
    seed: 8002,
    points: 1,
    timeout_seconds: 120,
};
