use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Write dfa.txt for a total deterministic finite automaton over alphabet 0,1.
It must accept exactly the binary strings with an even number of 1 bits.

Use this format:
start: STATE
accept: STATE [STATE...]
STATE SYMBOL NEXT_STATE

Every reachable state must have one 0 transition and one 1 transition.
";

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "dfa.txt",
    content: "\
start: even
accept: even
even 0 even
even 1 odd
odd 0 odd
odd 1 even
",
}];

const BAD_EDGE_FILES: &[FileSpec] = &[FileSpec {
    path: "dfa.txt",
    content: "\
start: even
accept: even
even 0 even
even 1 odd
odd 1 even
",
}];

const BAD_PUBLIC_FILES: &[FileSpec] = &[FileSpec {
    path: "dfa.txt",
    content: "\
start: even
accept: odd
even 0 even
even 1 odd
odd 0 odd
odd 1 even
",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "two-state-even-ones",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "missing-transition",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "accepts-odd-ones",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "dfa-even-ones-001",
    suite: "tiny",
    family: TaskFamily::Automata,
    difficulty: Difficulty::Small,
    tags: &["dfa", "formal-language", "equivalence"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::EvenOnesDfa,
    seed: 3001,
    points: 1,
    timeout_seconds: 120,
};
