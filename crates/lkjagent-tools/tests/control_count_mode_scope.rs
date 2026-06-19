use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::CountMode;

#[test]
fn exact_design_subcount_does_not_force_approximate_total_exact() {
    let mut state = ControlState::default();

    state.start_task(
        "Create about 100 files total for a structured story, with exactly twenty design memos.",
    );

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn approximate_design_subcount_does_not_weaken_exact_total() {
    let mut state = ControlState::default();

    state.start_task(
        "Create exactly 100 files total for a structured story, with about twenty design memos.",
    );

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Exact
        }
    );
}

#[test]
fn bare_not_exact_total_becomes_approximate() {
    let mut state = ControlState::default();

    state.start_task("Create 100 files total for docs and main content, not exactly.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}
