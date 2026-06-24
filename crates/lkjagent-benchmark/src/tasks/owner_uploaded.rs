use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=recover-repeat-parameter-fault\nschema_repair=one canonical example\nnext_action=graph.recover\ncompletion=refused\nfixture=bread-dictionary-shallow-content\nartifact_kind=Dictionary\ncontent_readiness=failed\nrepair_admitted=artifact.next,fs.batch_write\nfixture=large-write-payload-risk\npayload_too_large=blocked raw fs.write\nnext_action=fs.batch_write\nfixture=completion-with-blocked-mutation\ncompletion=refused\nmission=Repair\nmutation_tools=admitted\nfixture=maintenance-during-owner-work\nmaintenance=yielded\nmemory_loop=absent\nfixture=cookbook-scaffold-false-ready\nstructure_audit=passed\ncontent_readiness=failed\nagent.done=refused\nfixture=artifact-readiness-graph-evidence-bypass\ngraph.evidence artifact-readiness=refused\nnext_action=artifact.audit\nfixture=japanese-cookbook-drift\nowner_input=Create a very big cookbook about japanese foods.\nartifact_kind=Cookbook\nsubject=Japanese food\nbread_profile=rejected\nforbidden_bread_paths=absent\nfixture=document-structure-graph-evidence-bypass\ngraph.evidence document-structure=refused\nnext_action=doc.audit\nfixture=batch-write-payload-schema-fault\nfs.batch_write json_payload=refused\ncanonical_grammar=line-block\npartial_write=absent\nfixture=shell-parameter-missing-command\nshell.run missing_command=refused\nschema_repair=command required\ninvalid_timeout_retry=absent\nfixture=queue-story-interrupt\ncase1 objective=cookbook\ncase2 objective=japanese story\ncross_case_contamination=absent\nfixture=context-compaction-resume\ndurable_snapshot=created\npost_compaction_check=passed\nmissing_evidence=preserved\nlast_refused_action=preserved\nfixture=missing-act-block\nparse_fault=MissingActBlock\ndispatch=absent\nrecovery_route=RenderSingleExactActionExample\nfixture=empty-content-interrupted\nfault=InterruptedGeneration\nresume=last_durable_observation\nfixture=unclosed-act-from-stop\nclosure_mode=StopSequenceClosed\nparse_repair=logged\nfixture=contradictory-authority\nallowed_tools_none_with_tool_action=impossible\nauthority_recomputed=without_model\nfixture=provider-exchange-logging\nrequest_json=written\nresponse_json=written\nadmission_before_dispatch=required\nfixture=repeated-recovery-action\nrepeated_action_signature=blocked\nnext_action=different_action_class\n",
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

const BAD_ARTIFACT_EVIDENCE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=artifact-readiness-graph-evidence-bypass\ngraph.evidence kind=artifact-readiness\ncompletion=ready\nagent.done complete\n",
}];

const BAD_JAPANESE_COOKBOOK_DRIFT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=japanese-cookbook-drift\nartifact_kind=Cookbook\nsubject=Japanese food\nbread_profile=selected\npath=ciabatta.md\nagent.done complete\n",
}];

const BAD_DOCUMENT_STRUCTURE_EVIDENCE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=document-structure-graph-evidence-bypass\ngraph.evidence kind=document-structure\ncompletion=ready\nagent.done complete\n",
}];

const BAD_BATCH_WRITE_JSON: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=batch-write-payload-schema-fault\nfs.batch_write files=[{\"path\":\"x.md\",\"content\":\"body\"}]\npartial_write=present\n",
}];

const BAD_SHELL_PARAMETER: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=shell-parameter-missing-command\nshell.run timeout=30\ninvalid_timeout_retry=present\n",
}];

const BAD_QUEUE_INTERRUPT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=queue-story-interrupt\ncase1 objective=cookbook\ncase2 objective=cookbook\nactive_objective=overwritten\ncross_case_contamination=present\n",
}];

const BAD_COMPACTION_RESUME: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "fixture=context-compaction-resume\npost_compaction_check=skipped\nmissing_evidence=lost\n",
}];

const BAD_MISSING_ACT: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=missing-act-block\nempty assistant content\ndispatched graph.state\n",
}];

const BAD_UNCLOSED_STOP: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "fixture=unclosed-act-from-stop\n<action>\n<tool>graph.recover</tool>\nparse_repair=silent\n",
}];

const BAD_CONTRADICTORY_AUTHORITY: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content:
        "fixture=contradictory-authority\nallowed_tools=none\npreferred_next_action=graph.state\n",
}];

const BAD_PROVIDER_LOGGING: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=provider-exchange-logging\nrequest_json=missing\nresponse_json=missing\n",
}];

const BAD_REPEATED_RECOVERY: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=repeated-recovery-action\nrepeat identical graph.recover\n",
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
        Fixture {
            name: "artifact-readiness-graph-evidence-bypass",
            files: BAD_ARTIFACT_EVIDENCE,
        },
        Fixture {
            name: "japanese-cookbook-drift",
            files: BAD_JAPANESE_COOKBOOK_DRIFT,
        },
        Fixture {
            name: "document-structure-graph-evidence-bypass",
            files: BAD_DOCUMENT_STRUCTURE_EVIDENCE,
        },
        Fixture {
            name: "batch-write-payload-schema-fault",
            files: BAD_BATCH_WRITE_JSON,
        },
        Fixture {
            name: "shell-parameter-missing-command",
            files: BAD_SHELL_PARAMETER,
        },
        Fixture {
            name: "queue-story-interrupt",
            files: BAD_QUEUE_INTERRUPT,
        },
        Fixture {
            name: "context-compaction-resume",
            files: BAD_COMPACTION_RESUME,
        },
        Fixture {
            name: "missing-act-block",
            files: BAD_MISSING_ACT,
        },
        Fixture {
            name: "unclosed-act-from-stop",
            files: BAD_UNCLOSED_STOP,
        },
        Fixture {
            name: "contradictory-authority",
            files: BAD_CONTRADICTORY_AUTHORITY,
        },
        Fixture {
            name: "provider-exchange-logging",
            files: BAD_PROVIDER_LOGGING,
        },
        Fixture {
            name: "repeated-recovery-action",
            files: BAD_REPEATED_RECOVERY,
        },
    ],
    judge: JudgeKind::UploadedRunFixtures,
    seed: 8117,
    points: 1,
    timeout_seconds: 120,
};
