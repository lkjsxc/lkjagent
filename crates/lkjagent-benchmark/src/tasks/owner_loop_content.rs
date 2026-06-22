use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const POLICY_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "active_mode=Maintenance\ngraph_policy=disabled\nallowed_tools=memory.find,memory.save,queue.list,agent.done\nactive_mode=Compaction\nhard_compaction=runtime-owned\ngraph_policy=disabled\nmissing=plan\nallowed_tools=graph.plan,fs.read\npreferred_next_action=graph.plan\nvalid_example=<tool>graph.plan</tool>\n",
}];

const POLICY_BAD_MAINT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "maintenance only allows memory.find memory.save queue.list agent.done\nactive graph node=complete\ngraph policy refused memory.save\n",
}];

const POLICY_BAD_COMPACT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "compaction only allows memory.save actions\nactive graph node=recover-params\ngraph policy refused memory.save\n",
}];

const POLICY_BAD_PLAN: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "missing=plan\npreferred_next_action=graph.plan\nblocked_tools=graph.plan\ngraph policy refused graph.plan\n",
}];

pub const POLICY_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-policy-contradiction-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "active-mode", "policy"],
    prompt: "Render one active mode policy without graph/maintenance contradictions.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "compaction-graph-policy-contradiction-fixed",
        files: POLICY_GOOD,
    }],
    bad: &[
        Fixture {
            name: "maintenance-graph-policy-contradiction",
            files: POLICY_BAD_MAINT,
        },
        Fixture {
            name: "compaction-graph-policy-contradiction",
            files: POLICY_BAD_COMPACT,
        },
        Fixture {
            name: "graph-plan-required-but-refused",
            files: POLICY_BAD_PLAN,
        },
    ],
    judge: JudgeKind::PolicyContradiction,
    seed: 8114,
    points: 1,
    timeout_seconds: 120,
};

const NOTE_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "graph.note kind=decision normalized_from=planning\nsummary=Recovery will use document construction.\ngraph.note kind=success normalized_from=progress\n",
}];

const NOTE_BAD_KIND: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "graph.note kind=planning\ngraph.note kind=progress\ngraph.note kind=note\ngraph.note kind=evidence\ngraph.note kind=recovery\n",
}];

const NOTE_BAD_EVIDENCE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "graph.evidence kind=decision\nunknown graph evidence requirement: decision\ngraph.next\ngraph.next\n",
}];

pub const NOTE_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-graph-note-kind-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "graph-note", "evidence"],
    prompt: "Recover invalid graph.note and graph.evidence kinds deterministically.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "graph-note-invalid-kind-loop-fixed",
        files: NOTE_GOOD,
    }],
    bad: &[
        Fixture {
            name: "graph-note-invalid-kind-loop",
            files: NOTE_BAD_KIND,
        },
        Fixture {
            name: "graph-evidence-note-kind-loop",
            files: NOTE_BAD_EVIDENCE,
        },
    ],
    judge: JudgeKind::GraphNoteKindRecovery,
    seed: 8115,
    points: 1,
    timeout_seconds: 120,
};

const BREAD_GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=Create a big bread cookbook.\nsubroute=content-artifact\nroot=cookbooks/bread-cookbook\nprofile=BreadCookbook\nartifact.next next_action=fs.batch_write\nsemantic_paths=cookbooks/README.md,cookbooks/foundations/flour-water-salt-yeast.md,cookbooks/recipes/sourdough-country-loaf.md\ncontent-bearing files verified\nartifact.audit passed\nagent.done complete after artifact audit\n",
}];

const BREAD_BAD_GENERIC: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "root=docs/bread\nsemantic_paths=docs/bread/README.md,docs/bread/architecture.md,docs/bread/operations.md\nagent.done all evidence requirements met\n",
}];

const BREAD_BAD_SCAFFOLD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "profile=GenericProjectDocs\ndoc.scaffold root=docs/bread\ndocument=root=structured-output audit=Missing\nagent.done scaffold only\n",
}];

const BREAD_BAD_EMPTY_ROOT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "root=cookbooks/cookbook-about-bread\nempty artifact root after many turns\nrecover-by-smaller-scope blocked doc.scaffold\nagent.done claimed progress\n",
}];

const BREAD_BAD_BLOCKED_NEXT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "subroute=content-artifact\nroot=cookbooks/bread-cookbook\nartifact.next blocked by graph policy\nnext_action=graph.next\nagent.done claimed progress\n",
}];

pub const BREAD_TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-loop-bread-cookbook-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "cookbook", "content-artifact"],
    prompt: "Create a large bread cookbook as a semantic content artifact.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "bread-cookbook-content-artifact",
        files: BREAD_GOOD,
    }],
    bad: &[
        Fixture {
            name: "bread-cookbook-generic-scaffold-false-complete",
            files: BREAD_BAD_GENERIC,
        },
        Fixture {
            name: "bread-cookbook-scaffold-only-complete",
            files: BREAD_BAD_SCAFFOLD,
        },
        Fixture {
            name: "recovery-big-bread-cookbook-empty-root",
            files: BREAD_BAD_EMPTY_ROOT,
        },
        Fixture {
            name: "bread-cookbook-artifact-next-blocked",
            files: BREAD_BAD_BLOCKED_NEXT,
        },
    ],
    judge: JudgeKind::BreadCookbookArtifact,
    seed: 8116,
    points: 1,
    timeout_seconds: 120,
};
