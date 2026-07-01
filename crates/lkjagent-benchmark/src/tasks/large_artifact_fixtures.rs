use crate::model::{FileSpec, Fixture};

pub const MANUSCRIPT_GOOD: [Fixture; 1] = [fixture(
    "complete",
    &[
        fw("stories/aurora/manuscript/chapter-01.md", 5100),
        fw("stories/aurora/manuscript/chapter-02.md", 5100),
    ],
)];
pub const MANUSCRIPT_BAD: [Fixture; 2] = [
    fixture(
        "short",
        &[fw("stories/aurora/manuscript/chapter-01.md", 200)],
    ),
    fixture("missing", &[]),
];
pub const REPORT_GOOD: [Fixture; 1] = [fixture("complete", &report_files(100))];
pub const REPORT_BAD: [Fixture; 2] = [fixture("thin", &report_files(10)), fixture("missing", &[])];
pub const STUDY_GOOD: [Fixture; 1] = [fixture("complete", &study_files(60))];
pub const STUDY_BAD: [Fixture; 2] = [fixture("thin", &study_files(5)), fixture("missing", &[])];
pub const DOCS_GOOD: [Fixture; 1] = [fixture("complete", &doc_files(60))];
pub const DOCS_BAD: [Fixture; 2] = [fixture("thin", &doc_files(5)), fixture("missing", &[])];
pub const GENERIC_GOOD: [Fixture; 1] = [fixture(
    "refused",
    &[file(
        "transcript.md",
        "generic_root_refused=true\nrequested_root=reports/market\n",
    )],
)];
pub const GENERIC_BAD: [Fixture; 2] = [
    fixture(
        "accepted",
        &[file("transcript.md", "generic_root_refused=false\n")],
    ),
    fixture(
        "created",
        &[
            file("structured-output/README.md", "bad"),
            file("transcript.md", "generic_root_refused=true\n"),
        ],
    ),
];
pub const RETRY_GOOD: [Fixture; 1] = [fixture(
    "retry",
    &[file("events.log", "atom_status=weak\natom_status=ready\n")],
)];
pub const RETRY_BAD: [Fixture; 2] = [
    fixture("weak", &[file("events.log", "atom_status=weak\n")]),
    fixture("empty", &[]),
];
pub const ASSEMBLY_GOOD: [Fixture; 1] = [fixture(
    "assembled",
    &[
        fw(
            "stories/assembly/manuscript/scenes/chapter-01/scene-01.md",
            220,
        ),
        fw("stories/assembly/manuscript/chapter-01.md", 240),
    ],
)];
pub const ASSEMBLY_BAD: [Fixture; 2] = [
    fixture(
        "source-only",
        &[fw(
            "stories/assembly/manuscript/scenes/chapter-01/scene-01.md",
            220,
        )],
    ),
    fixture(
        "short",
        &[
            fw(
                "stories/assembly/manuscript/scenes/chapter-01/scene-01.md",
                20,
            ),
            fw("stories/assembly/manuscript/chapter-01.md", 20),
        ],
    ),
];

const fn fixture(name: &'static str, files: &'static [FileSpec]) -> Fixture {
    Fixture { name, files }
}
const fn file(path: &'static str, content: &'static str) -> FileSpec {
    FileSpec { path, content }
}
const fn fw(path: &'static str, count: usize) -> FileSpec {
    FileSpec {
        path,
        content: word_marker(count),
    }
}
const fn word_marker(count: usize) -> &'static str {
    match count {
        5 => "@repeat_word word 5",
        10 => "@repeat_word word 10",
        20 => "@repeat_word word 20",
        60 => "@repeat_word word 60",
        100 => "@repeat_word word 100",
        200 => "@repeat_word word 200",
        220 => "@repeat_word word 220",
        240 => "@repeat_word word 240",
        5100 => "@repeat_word word 5100",
        _ => "@repeat_word word 1",
    }
}
const fn report_files(count: usize) -> [FileSpec; 5] {
    [
        fw("reports/market/executive-summary.md", count),
        fw("reports/market/evidence.md", count),
        fw("reports/market/analysis.md", count),
        fw("reports/market/recommendations.md", count),
        fw("reports/market/risks.md", count),
    ]
}
const fn study_files(count: usize) -> [FileSpec; 5] {
    [
        fw("study/rust-cert/objectives.md", count),
        fw("study/rust-cert/lessons/lesson-01.md", count),
        fw("study/rust-cert/flashcards.md", count),
        fw("study/rust-cert/drills.md", count),
        fw("study/rust-cert/quizzes.md", count),
    ]
}
const fn doc_files(count: usize) -> [FileSpec; 5] {
    [
        fw("docs/product-kit/overview.md", count),
        fw("docs/product-kit/usage.md", count),
        fw("docs/product-kit/architecture.md", count),
        fw("docs/product-kit/operations.md", count),
        fw("docs/product-kit/verification.md", count),
    ]
}
