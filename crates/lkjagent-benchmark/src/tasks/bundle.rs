use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Create a small README-indexed bundle under bundle/. It must contain exactly
four markdown files: bundle/README.md plus alpha.md, beta.md, and gamma.md.
The README must link all three topic files. Each topic must link back to
README.md and link to the next topic in the cycle alpha -> beta -> gamma -> alpha.
";

const GOOD_FILES: &[FileSpec] = &[
    FileSpec {
        path: "bundle/README.md",
        content: "# Bundle\n\n## Purpose\n\nIndex.\n\n## Table of Contents\n\n- [alpha.md](alpha.md)\n- [beta.md](beta.md)\n- [gamma.md](gamma.md)\n",
    },
    FileSpec {
        path: "bundle/alpha.md",
        content: "# Alpha\n\n## Purpose\n\nAlpha.\n\n- [Index](README.md)\n- [Next](beta.md)\n",
    },
    FileSpec {
        path: "bundle/beta.md",
        content: "# Beta\n\n## Purpose\n\nBeta.\n\n- [Index](README.md)\n- [Next](gamma.md)\n",
    },
    FileSpec {
        path: "bundle/gamma.md",
        content: "# Gamma\n\n## Purpose\n\nGamma.\n\n- [Index](README.md)\n- [Next](alpha.md)\n",
    },
];

const BAD_EDGE_FILES: &[FileSpec] = &[
    FileSpec {
        path: "bundle/README.md",
        content:
            "# Bundle\n\n## Purpose\n\nIndex.\n\n- [alpha.md](alpha.md)\n- [beta.md](beta.md)\n",
    },
    FileSpec {
        path: "bundle/alpha.md",
        content: "# Alpha\n\n- [Index](README.md)\n- [Next](beta.md)\n",
    },
];

const BAD_PUBLIC_FILES: &[FileSpec] = &[
    FileSpec {
        path: "bundle/README.md",
        content:
            "# Bundle\n\n- [alpha.md](alpha.md)\n- [beta.md](beta.md)\n- [gamma.md](gamma.md)\n",
    },
    FileSpec {
        path: "bundle/alpha.md",
        content: "# Alpha\n\n- [Next](beta.md)\n",
    },
    FileSpec {
        path: "bundle/beta.md",
        content: "# Beta\n\n- [Index](README.md)\n- [Next](gamma.md)\n",
    },
    FileSpec {
        path: "bundle/gamma.md",
        content: "# Gamma\n\n- [Index](README.md)\n- [Next](alpha.md)\n",
    },
];

const GOOD: &[Fixture] = &[Fixture {
    name: "complete-cycle",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "missing-files",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "missing-backlink",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "bundle-readme-cycle-001",
    suite: "tiny",
    family: TaskFamily::MultiFile,
    difficulty: Difficulty::Tiny,
    tags: &["structure", "links", "multi-file"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::ReadmeBundle,
    seed: 6001,
    points: 1,
    timeout_seconds: 120,
};
