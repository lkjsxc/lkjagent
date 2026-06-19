use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;

#[test]
fn hyphenated_english_number_word_keeps_full_value() {
    let mut state = ControlState::default();

    state.start_task("Create twenty-two files total for docs and main content.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 22,
            mode: CountMode::Exact
        }
    );
}
