use lkjagent_app::tui_event::TuiEvent;
use lkjagent_app::tui_render::render_non_tty;
use lkjagent_app::tui_state::{reduce, TuiEffect, TuiModel, TuiRunState};

#[test]
fn composer_survives_streaming_agent_events() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("draft".into()));
    let (model, _) = reduce(model, TuiEvent::AgentTextDelta("working".into()));

    assert_eq!(model.composer, "draft");
    assert_eq!(model.run_state, TuiRunState::Running);
    assert_eq!(model.transcript[0].text, "working");
}

#[test]
fn submit_clears_composer_and_emits_owner_effect() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("hello".into()));
    let (model, effects) = reduce(model, TuiEvent::UserSubmit);

    assert_eq!(model.composer, "");
    assert!(effects.contains(&TuiEffect::SubmitOwnerMessage("hello".into())));
    assert_eq!(model.transcript[0].text, "hello");
}

#[test]
fn interrupt_and_resize_are_pure_state_changes() {
    let (model, effects) = reduce(TuiModel::new(), TuiEvent::UserInterrupt);
    let (model, _) = reduce(
        model,
        TuiEvent::TerminalResize {
            width: 10,
            height: 5,
        },
    );

    assert_eq!(model.run_state, TuiRunState::Interrupted);
    assert_eq!(model.width, 40);
    assert_eq!(model.height, 10);
    assert!(effects.contains(&TuiEffect::InterruptRun));
}

#[test]
fn tool_approval_preserves_composer_until_submit() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("note".into()));
    let (model, _) = reduce(
        model,
        TuiEvent::ToolCallProposed {
            name: "fs.write".into(),
            decision_id: "decision-1".into(),
        },
    );
    let (model, effects) = reduce(model, TuiEvent::UserApproveTool);

    assert_eq!(model.composer, "note");
    assert_eq!(model.run_state, TuiRunState::ToolRunning);
    assert!(
        matches!(effects.last(), Some(TuiEffect::ApproveTool(card)) if card.name == "fs.write")
    );
}

#[test]
fn non_tty_render_shows_state_without_raw_terminal_control() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::AgentTextDelta("hello".into()));
    let text = render_non_tty(&model);

    assert!(text.contains("mode: non-tty"));
    assert!(text.contains("state: running"));
    assert!(text.contains("agent: hello"));
    assert!(!text.contains("\u{1b}"));
}
