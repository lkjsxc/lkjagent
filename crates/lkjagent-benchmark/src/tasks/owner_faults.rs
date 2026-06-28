use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GRAPH_PROMPT: &str = "\
Recover from a model output that calls graph.state with an accidental path
parameter. The next visible notice must show the valid graph.state action.
";

const GRAPH_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params refused\ntool=graph.state\nexpected=no parameters\nreceived=missing []; unknown [path]\nvalid_example:\n<action>\n<tool>graph.state</tool>\n</action>\nnext_node=recover-params\n",
}];

const GRAPH_BAD_OLD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "parse fault: missing params []; unknown params [path]\n",
}];

const GRAPH_BAD_EXAMPLE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "action params refused\ntool=graph.state\nvalid_example:\n<action>\n<tool>graph.state</tool>\n<path>.</path>\n</action>\nnext_node=recover-params\n",
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

const BATCH_PROMPT: &str = "\
Recover from a model output that sends object JSON to fs.batch_write.
The next visible notice must require line protocol with path/content blocks.
";

const BATCH_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "schema fault: object-literal fs.batch_write files are not live output\ntool=fs.batch_write\ncanonical_grammar=line-protocol\npath: docs/README.md\ncontent:\n# Docs\n",
}];

const BATCH_BAD_JSON: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "<tool>fs.batch_write</tool>\n<files>[{\"path\":\"docs/README.md\",\"content\":\"# Docs\"}]</files>\n",
}];

const BATCH_BAD_CHILD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "<tool>fs.batch_write</tool>\n<files><file><path>docs/README.md</path></file></files>\n",
}];

const BATCH_GOOD_FIXTURES: &[Fixture] = &[Fixture {
    name: "line-protocol-required",
    files: BATCH_GOOD,
}];

const BATCH_BAD_FIXTURES: &[Fixture] = &[
    Fixture {
        name: "json-files-payload",
        files: BATCH_BAD_JSON,
    },
    Fixture {
        name: "child-file-tags",
        files: BATCH_BAD_CHILD,
    },
];

pub const BATCH_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-param-batch-write-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "params", "batch-write"],
    prompt: BATCH_PROMPT,
    follow_up: None,
    starter_files: &[],
    good: BATCH_GOOD_FIXTURES,
    bad: BATCH_BAD_FIXTURES,
    judge: JudgeKind::BatchWriteProtocolRecovery,
    seed: 8102,
    points: 1,
    timeout_seconds: 120,
};

const STORY_PROMPT: &str = "\
Create long SF story.
";

const STORY_FOLLOW_UP: &str = "\
not allowed big file. structured please.
";

const STORY_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=Create long SF story.\nfollowup_input=not allowed big file. structured please.\nfamily=documentation\nsubroute=content-artifact\nroot=stories/long-sf-story\npayload_risk=raw fs.write retry is blocked\nnext_action=artifact.plan\n<tool>artifact.plan</tool>\nprofile=Story\nsemantic_paths=stories/README.md,stories/premise.md,stories/chapters/waking-pod.md\ngraph.note kind=decision\nartifact.audit passed\nfinal_status=evidence-backed partial handoff\n",
}];

const STORY_BAD_NOTE_LOOP: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=Create long SF story.\nparse fault: unclosed tag content\ngraph.note kind=planning\ngraph.note kind=progress\n<tool>graph.next</tool>\n<tool>graph.next</tool>\nagent.ask how should I proceed?\n",
}];

const STORY_BAD_COMPACTION: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=Create long SF story.\n<tool>fs.write</tool>\n<path>story.md</path>\ncompletion hit max tokens\ncompaction only allows memory.save actions\ngraph policy refused memory.save\n",
}];

const STORY_BAD_FALSE_COMPLETE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=Create a very long story.\ngraph.plan recorded\nagent.done planning phase only\nartifact root missing\naudit=Missing\n",
}];

const STORY_GOOD_FIXTURES: &[Fixture] = &[Fixture {
    name: "recovery-loop-very-long-story-structured-followup",
    files: STORY_GOOD,
}];

const STORY_BAD_FIXTURES: &[Fixture] = &[
    Fixture {
        name: "recovery-loop-long-sf-story",
        files: STORY_BAD_NOTE_LOOP,
    },
    Fixture {
        name: "compaction-graph-policy-contradiction",
        files: STORY_BAD_COMPACTION,
    },
    Fixture {
        name: "recovery-very-long-story-false-complete",
        files: STORY_BAD_FALSE_COMPLETE,
    },
];

pub const STORY_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-recovery-loop-long-story-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "recovery-loop", "content-artifact"],
    prompt: STORY_PROMPT,
    follow_up: Some(STORY_FOLLOW_UP),
    starter_files: &[],
    good: STORY_GOOD_FIXTURES,
    bad: STORY_BAD_FIXTURES,
    judge: JudgeKind::RecoveryLoopLongStory,
    seed: 8103,
    points: 1,
    timeout_seconds: 120,
};
