use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GRAPH_PROMPT: &str = "\
Recover from a model output that calls graph.state with an accidental path
parameter. The next visible notice must show the valid graph.state action.
";

const GRAPH_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params refused\ntool=graph.state\nexpected=no parameters\nreceived=missing []; unknown [path]\nvalid_example:\n<act>\n<tool>graph.state</tool>\n</act>\nnext_node=recover-params\n",
}];

const GRAPH_BAD_OLD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "parse fault: missing params []; unknown params [path]\n",
}];

const GRAPH_BAD_EXAMPLE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params refused\ntool=graph.state\nvalid_example:\n<act>\n<tool>graph.state</tool>\n<path>.</path>\n</act>\nnext_node=recover-params\n",
}];

const GRAPH_GOOD_FIXTURES: &[Fixture] = &[Fixture {
    name: "valid-example",
    files: GRAPH_GOOD,
}];

const GRAPH_BAD_FIXTURES: &[Fixture] = &[
    Fixture {
        name: "old-unknown-param",
        files: GRAPH_BAD_OLD,
    },
    Fixture {
        name: "invalid-example",
        files: GRAPH_BAD_EXAMPLE,
    },
];

pub const GRAPH_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-param-graph-state-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "params", "graph-state"],
    prompt: GRAPH_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GRAPH_GOOD_FIXTURES,
    bad: GRAPH_BAD_FIXTURES,
    judge: JudgeKind::GraphStateParamRecovery,
    seed: 8101,
    points: 1,
    timeout_seconds: 120,
};

const SCAFFOLD_PROMPT: &str = "\
Recover from a model output that calls doc.scaffold with path instead of
root. Normalize it or show a valid root-based action example.
";

const SCAFFOLD_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params normalized\ntool=doc.scaffold\nrenamed=path->root\nreason=doc.scaffold uses root, not path\n",
}];

const SCAFFOLD_BAD_OLD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "parse fault: missing params [root]; unknown params [path]\n",
}];

const SCAFFOLD_BAD_PATH: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params refused\ntool=doc.scaffold\nvalid_example:\n<act>\n<tool>doc.scaffold</tool>\n<path>docs</path>\n</act>\n",
}];

const SCAFFOLD_GOOD_FIXTURES: &[Fixture] = &[Fixture {
    name: "path-renamed",
    files: SCAFFOLD_GOOD,
}];

const SCAFFOLD_BAD_FIXTURES: &[Fixture] = &[
    Fixture {
        name: "old-unknown-param",
        files: SCAFFOLD_BAD_OLD,
    },
    Fixture {
        name: "path-example",
        files: SCAFFOLD_BAD_PATH,
    },
];

pub const SCAFFOLD_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-param-doc-scaffold-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "params", "doc-scaffold"],
    prompt: SCAFFOLD_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: SCAFFOLD_GOOD_FIXTURES,
    bad: SCAFFOLD_BAD_FIXTURES,
    judge: JudgeKind::DocScaffoldParamRecovery,
    seed: 8102,
    points: 1,
    timeout_seconds: 120,
};
