use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Solve this exact integer problem. Write only the smallest nonnegative integer
answer to answer.txt.

Find x such that:
- x mod 7 = 3
- x mod 11 = 5
- x mod 13 = 8
";

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "346\n",
}];

const BAD_EDGE_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "1347\n",
}];

const BAD_SAMPLE_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "3\n",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "canonical",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "congruent-but-not-minimal",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "first-public-residue-only",
        files: BAD_SAMPLE_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "crt-exact-001",
    suite: "tiny",
    family: TaskFamily::Arithmetic,
    difficulty: Difficulty::Tiny,
    tags: &["exact", "modular-arithmetic", "certificate"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::Crt,
    seed: 1001,
    points: 1,
    timeout_seconds: 120,
};
