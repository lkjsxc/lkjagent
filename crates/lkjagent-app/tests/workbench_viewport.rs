use lkjagent_app::workbench_state::{reduce, UiEvent, UiState, Viewport, WorkbenchMode};

#[test]
fn reducer_tracks_mode_refresh_and_scroll() {
    let state = UiState::new(WorkbenchMode::Append);
    let state = reduce(state, UiEvent::Refresh("body".to_string()));
    let state = reduce(state, UiEvent::Mode(WorkbenchMode::Pane));
    let state = reduce(state, UiEvent::Scroll(4));
    let state = reduce(state, UiEvent::Scroll(-1));
    let state = reduce(state, UiEvent::Top);
    let state = reduce(state, UiEvent::Follow(true));
    let state = reduce(state, UiEvent::Search("daemon".to_string()));

    assert_eq!(state.mode, WorkbenchMode::Pane);
    assert_eq!(state.refreshes, 1);
    assert_eq!(state.scroll, 0);
    assert!(!state.follow);
    assert_eq!(state.search, "daemon");
    assert_eq!(state.latest, "body");
    assert_eq!(state.viewport, Viewport::Manual { top_line: 0 });
}

#[test]
fn scroll_down_to_bottom_reenables_follow() {
    let mut state = UiState::new(WorkbenchMode::Pane);
    state.height = 14;
    state = reduce(state, UiEvent::Refresh(lines(8)));
    state = reduce(state, UiEvent::Scroll(-1));
    assert_eq!(state.viewport, Viewport::Manual { top_line: 5 });
    assert!(!state.follow);

    state = reduce(state, UiEvent::Scroll(10));

    assert_eq!(state.viewport, Viewport::Follow);
    assert!(state.follow);
    assert_eq!(state.scroll, 0);
}

#[test]
fn refresh_preserves_manual_top_line() {
    let mut state = UiState::new(WorkbenchMode::Pane);
    state.height = 14;
    state = reduce(state, UiEvent::Refresh(lines(8)));
    state = reduce(state, UiEvent::Scroll(-2));
    state = reduce(state, UiEvent::Refresh(lines(10)));

    assert_eq!(state.viewport, Viewport::Manual { top_line: 4 });
    assert_eq!(state.scroll, 4);
}

fn lines(count: usize) -> String {
    (1..=count)
        .map(|index| format!("line-{index}"))
        .collect::<Vec<_>>()
        .join("\n")
}
