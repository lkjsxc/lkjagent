use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Find a shortest directed path from A to F in the graph below. Write the
certificate to path.txt as vertex names in order, separated by whitespace.

Edges with weights:
A B 2
A C 5
B C 1
B D 4
C D 1
C E 7
D E 3
D F 6
E F 1
";

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "path.txt",
    content: "A B C D E F\n",
}];

const BAD_EDGE_FILES: &[FileSpec] = &[FileSpec {
    path: "path.txt",
    content: "A C E F\n",
}];

const BAD_PUBLIC_FILES: &[FileSpec] = &[FileSpec {
    path: "path.txt",
    content: "A C D F\n",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "optimal-path",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "valid-but-too-long",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "plausible-public-path",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "graph-shortest-001",
    suite: "tiny",
    family: TaskFamily::Graph,
    difficulty: Difficulty::Tiny,
    tags: &["shortest-path", "certificate", "graph"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::ShortestPath,
    seed: 2001,
    points: 1,
    timeout_seconds: 120,
};
