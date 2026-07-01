use lkjagent_runtime::kernel::{
    build_snapshot, RuntimeDecisionKind, RuntimeEvent, RuntimeFault, SnapshotAdapterInput,
};

#[test]
fn completion_gate_reports_stable_blockers() -> Result<(), String> {
    let rows = vec![
        ("no objective", without_objective(), "objective"),
        ("missing evidence", with_missing_evidence(), "final-audit"),
        ("latest fault", with_latest_fault(), "recovery-fault"),
        (
            "artifact not ready",
            with_artifact_not_ready(),
            "artifact-readiness",
        ),
        (
            "content atom missing",
            with_missing_content_atom(),
            "content-atoms:1:atom-2",
        ),
        (
            "manuscript path",
            with_missing_manuscript_path(),
            "manuscript-paths",
        ),
        (
            "manuscript floor",
            with_short_manuscript(),
            "manuscript-word-count",
        ),
    ];
    for (name, input, expected) in rows {
        let snapshot = build_snapshot(input).map_err(format_error)?;
        let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::CaseClosed)
            .map_err(format_error)?;
        assert!(
            decision
                .completion_blockers
                .iter()
                .any(|blocker| blocker.contains(expected)),
            "{name}: {:?}",
            decision.completion_blockers
        );
    }
    Ok(())
}

#[test]
fn completion_gate_allows_ready_manuscript_projection() -> Result<(), String> {
    let snapshot = build_snapshot(complete_manuscript()).map_err(format_error)?;
    let decision = lkjagent_runtime::kernel::reduce(&snapshot, RuntimeEvent::CaseClosed)
        .map_err(format_error)?;

    assert!(
        decision.completion_allowed,
        "{:?}",
        decision.completion_blockers
    );
    assert_eq!(decision.kind, RuntimeDecisionKind::CloseCase);
    Ok(())
}

fn complete_input() -> SnapshotAdapterInput {
    SnapshotAdapterInput {
        snapshot_id: 1,
        case_id: Some("case-17".to_string()),
        queue_head: Some("queue-17".to_string()),
        pending_owner_count: 1,
        owner_objective: Some("finish Chronos Fracture story bible".to_string()),
        artifact_root: Some("stories/chronos-fracture".to_string()),
        existing_evidence: vec![
            "plan".to_string(),
            "observation".to_string(),
            "document-structure".to_string(),
            "artifact-readiness".to_string(),
        ],
        ..SnapshotAdapterInput::default()
    }
}

fn without_objective() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.owner_objective = None;
    input
}

fn with_missing_evidence() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.missing_evidence = vec!["final-audit".to_string()];
    input
}

fn with_latest_fault() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.latest_fault = Some(RuntimeFault::CompletionRefused);
    input
}

fn with_artifact_not_ready() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.required_evidence = vec!["artifact-readiness".to_string()];
    input
        .existing_evidence
        .retain(|item| item != "artifact-readiness");
    input.artifact_readiness = Some("blocked".to_string());
    input
}

fn with_missing_content_atom() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.artifact_atom_total = 2;
    input.artifact_atom_ready = 1;
    input.artifact_atom_missing = 1;
    input.artifact_next_atom = Some("atom-2".to_string());
    input
}

fn with_missing_manuscript_path() -> SnapshotAdapterInput {
    let mut input = manuscript_input();
    input.latest_successful_observation = Some(
        "manuscript_missing_paths=stories/chronos-fracture/manuscript/chapter-01.md\n\
         manuscript_word_count=900\n\
         manuscript_target_words=1000\n\
         next_manuscript_path=stories/chronos-fracture/manuscript/scenes/chapter-01/scene-01.md"
            .to_string(),
    );
    input
}

fn with_short_manuscript() -> SnapshotAdapterInput {
    let mut input = manuscript_input();
    input.latest_successful_observation = Some(
        "manuscript_missing_paths=none\n\
         manuscript_word_count=100\n\
         manuscript_target_words=1000\n\
         manuscript_chapter_count=1"
            .to_string(),
    );
    input
}

fn complete_manuscript() -> SnapshotAdapterInput {
    let mut input = manuscript_input();
    input.latest_successful_observation = Some(
        "manuscript_missing_paths=none\n\
         manuscript_word_count=900\n\
         manuscript_target_words=1000\n\
         manuscript_chapter_count=1\n\
         scene_atoms_unassembled=none"
            .to_string(),
    );
    input
}

fn manuscript_input() -> SnapshotAdapterInput {
    let mut input = complete_input();
    input.owner_objective = Some(
        "Create a 1000 word manuscript under stories/chronos-fracture/manuscript/chapter-01.md."
            .to_string(),
    );
    input.artifact_kind = Some("manuscript".to_string());
    input.artifact_readiness = Some("ready".to_string());
    input.artifact_measured_total = 900;
    input.artifact_accepted_floor = 850;
    input
}

fn format_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
