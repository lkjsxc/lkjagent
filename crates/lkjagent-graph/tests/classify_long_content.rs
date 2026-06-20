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
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "stories/long-sf-story" && document.kind == "content-artifact"
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
fn big_bread_cookbook_routes_to_content_artifact() {
    let state = initial_state("Write a big bread cookbook.", Some(4));

    assert_eq!(state.family, TaskFamily::Documentation);
    assert_eq!(state.subroute, "content-artifact");
    assert!(state.document.as_ref().is_some_and(|document| {
        document.root == "cookbooks/bread-cookbook" && document.kind == "content-artifact"
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
