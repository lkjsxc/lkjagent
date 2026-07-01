#[path = "large_artifact_fixtures.rs"]
mod fixtures;

use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};
use fixtures::*;

const TAGS: &[&str] = &["owner", "artifact", "large-artifact"];
const EMPTY: &[FileSpec] = &[];

pub const MANUSCRIPT: BenchmarkTask = task(
    "long-manuscript-completion",
    "Prove a long manuscript from real chapter files.",
    &MANUSCRIPT_GOOD,
    &MANUSCRIPT_BAD,
    JudgeKind::LargeArtifactManuscript,
    601,
);
pub const REPORT: BenchmarkTask = task(
    "large-report-completion",
    "Prove report atom readiness from real report files.",
    &REPORT_GOOD,
    &REPORT_BAD,
    JudgeKind::LargeArtifactReport,
    602,
);
pub const STUDY: BenchmarkTask = task(
    "study-set-completion",
    "Prove lessons, cards, drills, and quizzes from files.",
    &STUDY_GOOD,
    &STUDY_BAD,
    JudgeKind::LargeArtifactStudySet,
    603,
);
pub const DOCS: BenchmarkTask = task(
    "documentation-tree-completion",
    "Prove documentation tree atoms from real files.",
    &DOCS_GOOD,
    &DOCS_BAD,
    JudgeKind::LargeArtifactDocumentation,
    604,
);
pub const GENERIC_ROOT: BenchmarkTask = task(
    "generic-root-refusal",
    "Prove named roots are not stolen by generic roots.",
    &GENERIC_GOOD,
    &GENERIC_BAD,
    JudgeKind::LargeArtifactGenericRoot,
    605,
);
pub const ATOM_RETRY: BenchmarkTask = task(
    "atom-retry-recovery",
    "Prove weak atom retry reaches readiness.",
    &RETRY_GOOD,
    &RETRY_BAD,
    JudgeKind::LargeArtifactAtomRetry,
    606,
);
pub const ASSEMBLY: BenchmarkTask = task(
    "assembly-replay",
    "Prove source atoms assemble to final targets.",
    &ASSEMBLY_GOOD,
    &ASSEMBLY_BAD,
    JudgeKind::LargeArtifactAssembly,
    607,
);

const fn task(
    id: &'static str,
    prompt: &'static str,
    good: &'static [Fixture],
    bad: &'static [Fixture],
    judge: JudgeKind,
    seed: u64,
) -> BenchmarkTask {
    BenchmarkTask {
        id,
        suite: "tiny",
        family: TaskFamily::OwnerReliability,
        difficulty: Difficulty::Small,
        tags: TAGS,
        prompt,
        follow_up: None,
        starter_files: EMPTY,
        good,
        bad,
        judge,
        seed,
        points: 1,
        timeout_seconds: 30,
    }
}
