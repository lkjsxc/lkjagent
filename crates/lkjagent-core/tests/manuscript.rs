use lkjagent_core::classify::instantiate;
use lkjagent_core::manuscript::{chapter_plan, extract};
use lkjagent_core::model::{CheckSpec, StepKind, StepState, TemplateId};

#[test]
fn manuscript_field_extraction_covers_real_objectives() {
    let aurora = extract(
        "Write the Aurora Ledger manuscript at stories/aurora-ledger as 10 chapters totaling 10000 words.",
    );
    assert_eq!(aurora.root, "stories/aurora-ledger");
    assert_eq!(aurora.chapter_count, 10);
    assert_eq!(aurora.total_words, 10000);

    let iwanna = extract("Make an iwanna manuscript with 5 chapters and 2000 words.");
    assert_eq!(iwanna.root, "stories/iwanna");
    assert_eq!(iwanna.chapter_count, 5);
    assert_eq!(iwanna.total_words, 2000);

    let japanese = extract("オーロラ manuscript を 十章 で 一万語 書く");
    assert_eq!(japanese.chapter_count, 10);
    assert_eq!(japanese.total_words, 10000);

    let fallback = extract("Write a manuscript without a count.");
    assert_eq!(fallback.chapter_count, 10);
    assert_eq!(fallback.total_words, 10000);
    assert!(fallback.note.is_some());
}

#[test]
fn aurora_manuscript_snapshot_has_plan_settings_verify_and_checks() {
    let snapshot = instantiate(
        7,
        "Write the Aurora Ledger manuscript at stories/aurora-ledger as 10 chapters totaling 10000 words.",
    );
    assert_eq!(snapshot.task.template, TemplateId::Manuscript);
    assert_eq!(snapshot.steps[0].kind, StepKind::Plan);
    assert_eq!(snapshot.steps[0].state, StepState::Done);
    assert_eq!(
        snapshot.steps[1].output_path.as_deref(),
        Some("stories/aurora-ledger/settings.md")
    );
    assert_eq!(snapshot.steps[2].kind, StepKind::Write);
    assert_eq!(
        snapshot.steps[2].output_path.as_deref(),
        Some("stories/aurora-ledger/manuscript/chapter-01.md")
    );
    assert!(snapshot.steps[2]
        .instruction
        .contains("bounded 350-word unit"));
    assert_eq!(snapshot.steps[12].kind, StepKind::Verify);
    assert_eq!(snapshot.steps[13].kind, StepKind::Respond);
    assert!(snapshot.task.checks.contains(&CheckSpec::FileCount {
        glob: "stories/aurora-ledger/manuscript/*.md".to_string(),
        min: 10,
        max: Some(10)
    }));
    assert!(snapshot.task.checks.contains(&CheckSpec::MinWordsTotal {
        glob: "stories/aurora-ledger/manuscript/*.md".to_string(),
        n: 10000
    }));
    assert_eq!(chapter_plan(&extract(&snapshot.task.objective)).len(), 10);
}
