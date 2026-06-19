use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;

#[test]
fn english_plural_scale_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("Create roughly hundreds of files total for docs and main content.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}
