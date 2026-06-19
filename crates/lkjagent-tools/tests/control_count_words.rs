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

#[test]
fn english_ish_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("Create 100-ish files total for docs and main content.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn finish_word_does_not_make_exact_file_count_approximate() {
    let mut state = ControlState::default();

    state.start_task("Create 100 files total for docs and main content, then finish.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Exact
        }
    );
}

#[test]
fn english_or_so_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("Create hundred files or so for docs and main content.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn japanese_hodo_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("合計百ファイルほどの大きな成果物を作ってください。");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}
