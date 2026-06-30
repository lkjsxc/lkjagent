use lkjagent_graph::{initial_state, TaskFamily};

#[test]
fn long_sf_story_routes_to_content_artifact() {
    let state = initial_state("Create long SF story.", Some(1));

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state
        .context
        .selected_packages
        .contains(&"doc-construction".to_string()));
    assert!(state.evidence.knows_requirement("document-structure"));
    assert!(state.evidence.knows_requirement("artifact-readiness"));
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/story" && document.kind == "content-artifact"
    }));
}

#[test]
fn very_long_story_structured_followup_routes_to_content_artifact() {
    let state = initial_state("Write a structured story deliverable.", Some(2));

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state
        .context
        .selected_packages
        .contains(&"doc-construction".to_string()));
}

#[test]
fn long_novel_structured_settings_uses_short_alias() {
    let state = initial_state("Create a long novel. with structured settings.", Some(6));
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/novel" && document.root.split('/').all(|s| s.len() <= 24)
    }));
}

#[test]
fn compact_title_long_novel_stays_content_artifact() {
    let state = initial_state(
        "Create a long novel named \"Compact Compass\" with structured settings.",
        Some(7),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/compact-compass" && document.kind == "content-artifact"
    }));
}

#[test]
fn named_novel_root_preserves_owner_title() {
    let state = initial_state(
        "Create a long novel named \"iwanna\" with detailed structured settings.",
        Some(8),
    );

    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/iwanna" && document.kind == "content-artifact"
    }));
}

#[test]
fn explicit_context_compaction_stays_compaction() {
    let state = initial_state("Compact the context after token pressure.", Some(9));

    assert_eq!(state.family, TaskFamily::Compaction);
}

#[test]
fn big_bread_cookbook_routes_to_content_artifact() {
    let state = initial_state("Write a big bread cookbook.", Some(4));

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "cookbooks/bread-cookbook" && document.kind == "content-artifact"
    }));
}

#[test]
fn detailed_bread_dictionary_routes_to_dictionary_artifact() {
    let state = initial_state(
        "Create a detailed bread dictionary with definitions and examples.",
        Some(5),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state.evidence.knows_requirement("artifact-readiness"));
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "dictionaries/bread-dictionary" && document.kind == "content-artifact"
    }));
}

#[test]
fn exact_manuscript_path_routes_to_story_artifact() {
    let state = initial_state(
        "Write one 700 to 900 word chapter at stories/the-bell-rings-twice/manuscript/chapter-01.md.",
        Some(10),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/the-bell-rings-twice"
            && document.profile.as_deref() == Some("story")
            && document.requested_scale.as_deref() == Some("full-draft")
    }));
}

#[test]
fn ten_thousand_word_novel_records_full_draft_scale() {
    let state = initial_state(
        "Create a 10,000 word high-school romance novel named \"The Bell Rings Twice\" in ten chapters.",
        Some(11),
    );

    assert_eq!(state.subroute, "content-artifact");
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/bell-rings-twice"
            && document.requested_scale.as_deref() == Some("full-draft")
    }));
}

#[test]
fn long_story_bug_fix_stays_bug_fix() {
    let state = initial_state("Fix the bug in the long story generator.", Some(3));

    assert_eq!(state.family, TaskFamily::BugFix);
    assert!(!state
        .context
        .selected_packages
        .contains(&"doc-construction".to_string()));
}
