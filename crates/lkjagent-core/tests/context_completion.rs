use lkjagent_core::runtime_completion::{can_close, CheckEvidence, CompletionRequirement};
use lkjagent_core::runtime_context::{
    detect_contradictions, select_normal_context, ContaminationClass, ContextItem,
};

#[test]
fn contradictions_render_as_conflicts_and_contamination_is_excluded() {
    let one = ContextItem::clean_fact("item-1", "target-root", "stories/a");
    let two = ContextItem::clean_fact("item-2", "target-root", "stories/b");
    let mut bad = ContextItem::clean_fact("item-3", "target-root", "stories/c");
    bad.contamination = ContaminationClass::FailedModelOutput;

    let items = vec![one, two, bad];
    let normal = select_normal_context(&items);
    let conflicts = detect_contradictions(&items);

    assert_eq!(normal.len(), 2);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].semantic_key, "target-root");
    assert_eq!(conflicts[0].item_ids, vec!["item-1", "item-2"]);
}

#[test]
fn completion_requires_fresh_matching_evidence() {
    let requirement = CompletionRequirement {
        check_name: "links_resolve".to_string(),
        artifact_fingerprint: "artifact:new".to_string(),
    };
    let stale = CheckEvidence {
        check_name: "links_resolve".to_string(),
        artifact_fingerprint: "artifact:old".to_string(),
        passed: true,
        decision_id: "decision-1".to_string(),
        created_at: "now".to_string(),
    };
    let fresh = CheckEvidence {
        artifact_fingerprint: "artifact:new".to_string(),
        ..stale.clone()
    };

    assert!(!can_close(&[], std::slice::from_ref(&fresh)));
    assert!(!can_close(std::slice::from_ref(&requirement), &[stale]));
    assert!(can_close(&[requirement], &[fresh]));
}
