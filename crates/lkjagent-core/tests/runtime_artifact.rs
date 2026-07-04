use lkjagent_core::runtime_artifact::{
    artifact_fingerprint, assemble_checked_units, ArtifactUnit, DEFAULT_UNIT_TARGET_TOKENS,
};
use lkjagent_core::runtime_completion::{can_close, CheckEvidence, CompletionRequirement};

#[test]
fn artifact_units_use_512_token_target_and_require_checks() {
    let mut unit = ArtifactUnit::new("unit-1", "stories/chapter.md", 1);
    unit.content = "Aurora opened the ledger".to_string();

    assert_eq!(unit.target_tokens, DEFAULT_UNIT_TARGET_TOKENS);
    assert!(assemble_checked_units("stories/chapter.md", &[unit.clone()]).is_err());

    unit.check_passed = true;
    let artifact = match assemble_checked_units("stories/chapter.md", &[unit]) {
        Ok(artifact) => artifact,
        Err(err) => {
            assert_eq!(err.message, "expected checked unit to assemble");
            return;
        }
    };
    assert_eq!(artifact.word_count, 4);
    assert_eq!(artifact.unit_ids, vec!["unit-1"]);
}

#[test]
fn closure_requires_checks_for_current_artifact_fingerprint() {
    let old = artifact_fingerprint("out.md", "old text").unwrap_or_default();
    let current = artifact_fingerprint("out.md", "new text").unwrap_or_default();
    let requirements = vec![CompletionRequirement {
        check_name: "min_words".to_string(),
        artifact_fingerprint: current.clone(),
    }];
    let stale = vec![CheckEvidence {
        check_name: "min_words".to_string(),
        artifact_fingerprint: old,
        passed: true,
        decision_id: "decision-1".to_string(),
        created_at: "t1".to_string(),
    }];
    let fresh = vec![CheckEvidence {
        check_name: "min_words".to_string(),
        artifact_fingerprint: current,
        passed: true,
        decision_id: "decision-2".to_string(),
        created_at: "t2".to_string(),
    }];

    assert!(!can_close(&requirements, &stale));
    assert!(can_close(&requirements, &fresh));
}
