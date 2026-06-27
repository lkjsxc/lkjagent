use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const TAGS: &[&str] = &["owner", "artifact", "long-novel", "recovery"];
const EMPTY: &[FileSpec] = &[];
const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: GOOD_TRANSCRIPT,
}];
const BAD_SCHEMA_FILES: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: BAD_SCHEMA_TRANSCRIPT,
}];
const BAD_STATUS_FILES: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: BAD_STATUS_TRANSCRIPT,
}];
const GOOD: &[Fixture] = &[Fixture {
    name: "long-novel-authority",
    files: GOOD_FILES,
}];
const BAD: &[Fixture] = &[
    Fixture {
        name: "schema-repeat",
        files: BAD_SCHEMA_FILES,
    },
    Fixture {
        name: "status-stale",
        files: BAD_STATUS_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-long-novel-run",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Small,
    tags: TAGS,
    prompt: "Replay the long-novel active run and preserve kernel-owned recovery evidence.",
    follow_up: None,
    starter_files: EMPTY,
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::LongNovelFailure,
    seed: 43,
    points: 1,
    timeout_seconds: 30,
};

const GOOD_TRANSCRIPT: &str = r#"
fixture=long-novel-active-run
root=stories/long-novel-with-detailed-settings
profile=NarrativeManuscript
doc.audit content_readiness=failed
weak_paths=28
child_file_tags=refused
schema_fault=unsupported child tags
second_same_shape=artifact.next
next_decision_required=true
provider_anomaly=reasoning_only_response
touched_paths=artifact-ledger-derived
maintenance_noop=cooldown
"#;

const BAD_SCHEMA_TRANSCRIPT: &str = r#"
fixture=long-novel-active-run
root=stories/long-novel-with-detailed-settings
profile=NarrativeManuscript
doc.audit content_readiness=failed
weak_paths=28
child_file_tags=refused
schema_fault=unsupported child tags
repeat child_file_tags
provider_anomaly=reasoning_only_response
touched_paths=artifact-ledger-derived
maintenance_noop=cooldown
"#;

const BAD_STATUS_TRANSCRIPT: &str = r#"
fixture=long-novel-active-run
root=stories/long-novel-with-detailed-settings
profile=NarrativeManuscript
doc.audit content_readiness=failed
weak_paths=28
child_file_tags=refused
schema_fault=unsupported child tags
second_same_shape=artifact.next
next_decision_required=true
provider_anomaly=reasoning_only_response
touched_paths=none
maintenance_noop=endpoint_churn
"#;
