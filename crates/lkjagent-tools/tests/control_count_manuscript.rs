use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;

#[test]
fn direct_chapter_path_does_not_create_count_guard() {
    let mut state = ControlState::default();
    state.start_task(
        "Write one 700 to 900 word chapter at \
         stories/the-bell-rings-twice/manuscript/chapter-01.md. \
         Do not create structured-output.",
    );

    assert_eq!(state.guard, CompletionGuard::None);
}

#[test]
fn prose_word_range_does_not_create_count_guard() {
    let mut state = ControlState::default();
    state.start_task("Write a 700 to 900 word chapter for a school romance novel.");

    assert_eq!(state.guard, CompletionGuard::None);
}

#[test]
fn exact_markdown_file_count_still_creates_guard() {
    let mut state = ControlState::default();
    state.start_task("Create exactly 12 Markdown files about onboarding.");

    assert_eq!(
        state.guard,
        CompletionGuard::MarkdownCount {
            target: 12,
            mode: CountMode::Exact
        }
    );
}

#[test]
fn approximate_document_file_count_still_creates_guard() {
    let mut state = ControlState::default();
    state.start_task("Create about 20 documentation files for the project.");

    assert!(matches!(
        state.guard,
        CompletionGuard::FileCount {
            target: 20,
            mode: CountMode::Approximate
        } | CompletionGuard::MarkdownCount {
            target: 20,
            mode: CountMode::Approximate
        }
    ));
}
