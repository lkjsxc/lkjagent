use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GRAPH_PLAN_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "valid_example:\n<action>\n<tool>graph.plan</tool>\n<objective>Repair loop policy.</objective>\n<steps>Inspect active mode. Render one policy.</steps>\n<checks>policy renders without contradiction</checks>\n<reason>Plan must name a check.</reason>\n</action>\n",
}];

const GRAPH_PLAN_BAD_OMITS_CHECKS: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "valid_example:\n<action>\n<tool>graph.plan</tool>\n<objective>Repair loop policy.</objective>\n<steps>Inspect active mode.</steps>\n<reason>Plan first.</reason>\n</action>\ngraph.plan needs checks or paths\n",
}];

const GRAPH_PLAN_BAD_LOOP: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "graph.plan needs checks or paths after generated valid example\n<tool>graph.next</tool>\n<tool>graph.next</tool>\n",
}];

pub const GRAPH_PLAN_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-graph-plan-example-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "graph-plan", "example"],
    prompt: "Render an actually dispatchable graph.plan valid example.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "graph-plan-valid-example-with-checks",
        files: GRAPH_PLAN_GOOD,
    }],
    bad: &[
        Fixture {
            name: "graph-plan-valid-example-missing-checks",
            files: GRAPH_PLAN_BAD_OMITS_CHECKS,
        },
        Fixture {
            name: "graph-plan-invalid-loop",
            files: GRAPH_PLAN_BAD_LOOP,
        },
    ],
    judge: JudgeKind::GraphPlanExample,
    seed: 8111,
    points: 1,
    timeout_seconds: 120,
};

const TRANSITION_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "legal_transitions=document-audit,document-repair\nvalid_example:\n<action>\n<tool>graph.transition</tool>\n<target>document-audit</target>\n<reason>audit document artifact before completion</reason>\n</action>\n",
}];

const TRANSITION_BAD_LABEL: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "valid_example:\n<action>\n<tool>graph.transition</tool>\n<target>plan:admitted</target>\n</action>\n",
}];

const TRANSITION_BAD_ALIAS: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "graph policy refused graph.transition\nvalid_example target=audit\nlegal target is document-audit\n",
}];

pub const TRANSITION_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-graph-transition-target-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "graph-transition", "example"],
    prompt: "Render graph.transition examples with legal node ids only.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "graph-transition-legal-target-example",
        files: TRANSITION_GOOD,
    }],
    bad: &[
        Fixture {
            name: "graph-transition-illegal-admitted-label",
            files: TRANSITION_BAD_LABEL,
        },
        Fixture {
            name: "graph-transition-illegal-target-example",
            files: TRANSITION_BAD_ALIAS,
        },
    ],
    judge: JudgeKind::GraphTransitionTarget,
    seed: 8117,
    points: 1,
    timeout_seconds: 120,
};

const FTS_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "memory.find query=graph.note query_normalized=graph note\nmemory.find query=parameter-fault query_normalized=parameter fault\n",
}];

const FTS_BAD_DOT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "memory.find query=graph.note\nstore error: sqlite error: fts5: syntax error near \".\"\n",
}];

const FTS_BAD_HYPHEN: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "memory.find query=parameter-fault\nstore error: sqlite error: fts5: syntax error near \"-\"\n",
}];

pub const FTS_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-fts-punctuation-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "memory-find", "fts"],
    prompt: "Search memory for punctuated graph and fault terms without FTS errors.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "fts5-punctuation-query-loop-fixed",
        files: FTS_GOOD,
    }],
    bad: &[
        Fixture {
            name: "fts5-punctuation-query-loop-dot",
            files: FTS_BAD_DOT,
        },
        Fixture {
            name: "fts5-punctuation-query-loop-hyphen",
            files: FTS_BAD_HYPHEN,
        },
    ],
    judge: JudgeKind::MemoryFtsQuery,
    seed: 8112,
    points: 1,
    timeout_seconds: 120,
};

const MEMORY_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "memory.save kind=lesson title=Graph note recovery\nMemoryWriteDecision=SkipDuplicate existing_id=42\nmaintenance no-op closed cooldown_set=true\n",
}];

const MEMORY_BAD_REPEAT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "memory.save kind=lesson title=Graph note recovery memory_id=10\nmemory.save kind=lesson title=Graph note recovery memory_id=11\nduplicate memory rows for same lesson\n",
}];

const MEMORY_BAD_ASK: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "maintenance asks owner: What stale memory rows need pruning?\nagent.ask What recent transcript spans should be distilled?\n",
}];

pub const MEMORY_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-memory-duplicate-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "maintenance", "memory"],
    prompt: "End maintenance without duplicate memory rows or owner questions.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "maintenance-memory-duplicate-loop-fixed",
        files: MEMORY_GOOD,
    }],
    bad: &[
        Fixture {
            name: "maintenance-memory-duplicate-loop",
            files: MEMORY_BAD_REPEAT,
        },
        Fixture {
            name: "maintenance-internal-owner-question",
            files: MEMORY_BAD_ASK,
        },
    ],
    judge: JudgeKind::MaintenanceMemoryDuplicate,
    seed: 8113,
    points: 1,
    timeout_seconds: 120,
};
