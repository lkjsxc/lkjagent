use lkjagent_graph::{initial_state, TaskFamily};

#[test]
fn counted_story_bug_fix_stays_bug_fix() {
    let state = initial_state(
        "Create exactly 3 files fixing a story bug in the runtime.",
        Some(21),
    );

    assert_eq!(state.family, TaskFamily::BugFix);
    assert!(!state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_story_refactor_stays_code_change() {
    let state = initial_state(
        "Create exactly 3 files refactoring a story export helper.",
        Some(22),
    );

    assert_eq!(state.family, TaskFamily::CodeChange);
    assert!(!state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_japanese_story_fix_stays_bug_fix() {
    let state = initial_state(
        "物語エクスポートのバグ修正として、ちょうど三ファイルを作ってください。",
        Some(24),
    );

    assert_eq!(state.family, TaskFamily::BugFix);
    assert!(!state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_architecture_artifact_still_selects_document_construction() {
    let state = initial_state(
        "Create about 100 files total for a structured architecture artifact with docs and main content.",
        Some(23),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_architecture_playbook_with_implementation_chapters_selects_documents() {
    let state = initial_state(
        "Create about one hundred files total for a product architecture playbook. Use \
         twenty-four decision records. The rest as ordered implementation chapters. Count docs \
         and main content together.",
        Some(28),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_body_files_select_document_construction() {
    let state = initial_state(
        "Create about 100 files total, split between twenty planning notes and body files.",
        Some(25),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_draft_files_select_document_construction() {
    let state = initial_state(
        "Create about 100 files total, split between twenty planning notes and draft files.",
        Some(26),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}

#[test]
fn counted_japanese_main_files_select_document_construction() {
    let state = initial_state(
        "合計百ファイルほどを、二十個の設計メモと本編ファイルに分けて作ってください。",
        Some(27),
    );

    assert_eq!(state.family, TaskFamily::Documentation);
    assert!(state
        .selected_packages()
        .contains(&"doc-construction".to_string()));
}
