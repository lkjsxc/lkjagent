use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Write answer.txt with the first five prime numbers greater than 20,
comma-separated on one line.
";

const FOLLOW_UP: &str = "\
Correction: answer.txt must instead contain the first seven composite
numbers greater than 20, comma-separated on one line. The latest instruction
replaces the prime-number request.
";

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "21,22,24,25,26,27,28\n",
}];

const BAD_EDGE_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "23,29,31,37,41\n",
}];

const BAD_PUBLIC_FILES: &[FileSpec] = &[FileSpec {
    path: "answer.txt",
    content: "21,22,24,25,26\n",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "latest-owner-guidance",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "solved-first-prompt",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "partial-correction",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "interrupted-composites-001",
    suite: "tiny",
    family: TaskFamily::StatefulCorrection,
    difficulty: Difficulty::Tiny,
    tags: &["interruption", "latest-instruction", "exact-answer"],
    prompt: PROMPT,
    follow_up: Some(FOLLOW_UP),
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::CorrectedComposites,
    seed: 7001,
    points: 1,
    timeout_seconds: 120,
};
