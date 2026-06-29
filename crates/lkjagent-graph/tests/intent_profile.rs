use lkjagent_graph::{classify_intent, initial_state, TaskFamily};

#[test]
fn compact_title_routes_as_story_artifact() -> Result<(), String> {
    let state = initial_state(
        "Create a long novel named \"Compact Compass\" with detailed settings.",
        None,
    );
    let doc = state.document.ok_or("artifact document")?;

    assert_eq!(
        classify_intent(&state.objective.raw_owner_message),
        TaskFamily::Documentation
    );
    assert_eq!(state.subroute, "content-artifact");
    assert_eq!(doc.root, "stories/compact-compass");
    assert_eq!(doc.profile.as_deref(), Some("story"));
    assert_eq!(doc.requested_scale.as_deref(), Some("large-story"));
    Ok(())
}

#[test]
fn counted_chapters_are_large_story_artifact() -> Result<(), String> {
    let state = initial_state("Write 20 chapters named \"Moon Gate\".", None);
    let doc = state.document.ok_or("artifact document")?;

    assert_eq!(state.subroute, "content-artifact");
    assert_eq!(doc.root, "stories/moon-gate");
    assert_eq!(doc.requested_scale.as_deref(), Some("full-draft"));
    Ok(())
}

#[test]
fn non_ascii_title_keeps_exact_title_and_stable_alias() -> Result<(), String> {
    let state = initial_state("長編小説 named \"星の海\" を作ってください。", None);
    let doc = state.document.ok_or("artifact document")?;
    let again = initial_state(&state.objective.raw_owner_message, None)
        .document
        .ok_or("second artifact document")?;

    assert_eq!(doc.exact_title.as_deref(), Some("星の海"));
    assert!(doc.root.starts_with("stories/title-"), "{}", doc.root);
    assert_eq!(doc.root, again.root);
    Ok(())
}

#[test]
fn code_change_intent_preempts_story_words() {
    let family = classify_intent("Fix the docs for the novel classifier implementation.");

    assert_eq!(family, TaskFamily::BugFix);
}
