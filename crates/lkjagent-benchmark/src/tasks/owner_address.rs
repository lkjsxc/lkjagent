use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const GOOD: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=sf-novel-file-root-audit-loop\nartifact.next root=stories/sf-novel-with-structured-settings/characters/protagonist.md\naddress_status=root_is_file\nnormalized_root=stories/sf-novel-with-structured-settings\nweak_path=characters/protagonist.md\nnext_action=fs.batch_write\nfixture=artifact-next-file-root-missing-zero-false\nfile_root_audit_example=absent\nmissing=not-zero\nfixture=markdown-suffix-directory-created-by-artifact-apply\nartifact.apply root=stories/sf-novel-with-structured-settings/02-characters.md\naddress_status=root_ends_with_markdown_suffix\ndirectory_created=false\nfixture=batch-write-json-in-files\nfs.batch_write json_payload=refused\nfiles_written=0\ncanonical_grammar=line-protocol\nfixture=oversized-fs-write-after-recovery\nfs.write payload_too_large=blocked\nnext_action=fs.batch_write\nsplit_semantic_files=required\n",
}];

const BAD_FILE_AUDIT_LOOP: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=sf-novel-file-root-audit-loop\nnext_action=artifact.audit\nroot=stories/sf-novel-with-structured-settings/characters/protagonist.md\nio error: Not a directory\n",
}];

const BAD_MISSING_ZERO: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=artifact-next-file-root-missing-zero-false\nroot=stories/sf-novel-with-structured-settings/characters/protagonist.md\nmissing=0\nnext_action=artifact.audit\n",
}];

const BAD_MD_DIRECTORY: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=markdown-suffix-directory-created-by-artifact-apply\nartifact.apply root=stories/sf-novel-with-structured-settings/02-characters.md\ndirectory_created=true\nmarkdown_suffix_directory\n",
}];

const BAD_JSON_BATCH: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=batch-write-json-in-files\nfs.batch_write input_format=json-array\npartial_write=present\n",
}];

const BAD_OVERSIZED_WRITE: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: "fixture=oversized-fs-write-after-recovery\nfs.write payload_too_large\nretry raw fs.write\nsplit_semantic_files=absent\n",
}];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-artifact-address-controller-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "artifact-address", "sf-novel"],
    prompt: "Reject SF-novel file-root audit loops and recover with valid address actions.",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "artifact-address-controller-fixed",
        files: GOOD,
    }],
    bad: &[
        Fixture {
            name: "sf-novel-file-root-audit-loop",
            files: BAD_FILE_AUDIT_LOOP,
        },
        Fixture {
            name: "artifact-next-file-root-missing-zero-false",
            files: BAD_MISSING_ZERO,
        },
        Fixture {
            name: "markdown-suffix-directory-created-by-artifact-apply",
            files: BAD_MD_DIRECTORY,
        },
        Fixture {
            name: "batch-write-json-in-files",
            files: BAD_JSON_BATCH,
        },
        Fixture {
            name: "oversized-fs-write-after-recovery",
            files: BAD_OVERSIZED_WRITE,
        },
    ],
    judge: JudgeKind::ArtifactAddressController,
    seed: 8123,
    points: 1,
    timeout_seconds: 120,
};
