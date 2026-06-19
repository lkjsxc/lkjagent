use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Repair rank.sh. The script reads signed integers, one per line, from stdin.
It must print the distinct integers sorted numerically ascending, one per
line. Keep the file named rank.sh. The judge runs hidden shell cases.
";

const STARTER: &[FileSpec] = &[FileSpec {
    path: "rank.sh",
    content: "#!/bin/sh\nsort\n",
}];

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "rank.sh",
    content: "#!/bin/sh\nsort -n | uniq\n",
}];

const BAD_EDGE_FILES: &[FileSpec] = STARTER;

const BAD_PUBLIC_FILES: &[FileSpec] = &[FileSpec {
    path: "rank.sh",
    content: "#!/bin/sh\nprintf '1\\n2\\n3\\n'\n",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "numeric-distinct-sort",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "original-lexicographic-sort",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "hard-coded-small-output",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "repair-rank-001",
    suite: "tiny",
    family: TaskFamily::ProgramRepair,
    difficulty: Difficulty::Small,
    tags: &["repair", "shell", "hidden-cases"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: STARTER,
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::RepairRankShell,
    seed: 5001,
    points: 1,
    timeout_seconds: 180,
};
