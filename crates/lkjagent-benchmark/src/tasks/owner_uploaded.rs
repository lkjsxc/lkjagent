use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=recover-repeat-parameter-fault\nschema_repair=one canonical example\nnext_action=graph.recover\ncompletion=refused\nfixture=bread-dictionary-shallow-content\nartifact_kind=Dictionary\ncontent_readiness=failed\nrepair_admitted=artifact.next,fs.batch_write\nfixture=large-write-payload-risk\npayload_too_large=blocked raw fs.write\nnext_action=fs.batch_write\nfixture=completion-with-blocked-mutation\ncompletion=refused\nmission=Repair\nmutation_tools=admitted\nfixture=maintenance-during-owner-work\nmaintenance=yielded\nmemory_loop=absent\nfixture=cookbook-scaffold-false-ready\nstructure_audit=passed\ncontent_readiness=failed\nagent.done=refused\n",
}];

const BAD_RECOVER_REPEAT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=recover-repeat-parameter-fault\ngraph.state\ngraph.state\ngraph.state\nagent.done complete\n",
}];

const BAD_DICTIONARY: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "owner_input=more more detailed dictionary please\ntouched_path=dictionary/bread-terms.txt\n32 bread terms listed\ncontent_readiness=passed\nagent.done complete\n",
}];

const BAD_PAYLOAD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "fixture=large-write-payload-risk\n<tool>fs.write</tool>\nmax_tokens\nretry raw fs.write\n",
}];

const BAD_BLOCKED_COMPLETION: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=completion-with-blocked-mutation\nactive_node=complete\nmutation_tools=blocked\nagent.done complete\n",
}];

const BAD_MAINTENANCE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=maintenance-during-owner-work\nactive_owner_case=true\nmaintenance memory.save\nempty maintenance cycle\n",
}];

const BAD_COOKBOOK: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=cookbook-scaffold-false-ready\n100 files\nstructure_audit=passed\ncontent_readiness=passed\nagent.done complete\n",
}];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-uploaded-run-fixtures-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "uploaded-run", "runtime-authority"],
    prompt: "Reject uploaded run-log failure patterns and show productive next actions.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "uploaded-run-fixture-matrix-fixed",
        files: GOOD,
    }],
    bad: &[
        Fixture {
            name: "recover-repeat-parameter-fault",
            files: BAD_RECOVER_REPEAT,
        },
        Fixture {
            name: "bread-dictionary-shallow-content",
            files: BAD_DICTIONARY,
        },
        Fixture {
            name: "large-write-payload-risk",
            files: BAD_PAYLOAD,
        },
        Fixture {
            name: "completion-with-blocked-mutation",
            files: BAD_BLOCKED_COMPLETION,
        },
        Fixture {
            name: "maintenance-during-owner-work",
            files: BAD_MAINTENANCE,
        },
        Fixture {
            name: "cookbook-scaffold-false-ready",
            files: BAD_COOKBOOK,
        },
    ],
    judge: JudgeKind::UploadedRunFixtures,
    seed: 8117,
    points: 1,
    timeout_seconds: 120,
};
