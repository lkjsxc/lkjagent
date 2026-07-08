use std::fs;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_app::tui_event::TuiEvent;
use lkjagent_app::tui_render::render_non_tty;
use lkjagent_app::tui_snapshot::{self, TuiSnapshot};
use lkjagent_app::tui_state::{reduce, TuiEffect, TuiModel, TuiPane, TuiRunState};
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

#[test]
fn composer_survives_streaming_agent_events() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("draft".into()));
    let (model, _) = reduce(model, TuiEvent::AgentTextDelta("working".into()));

    assert_eq!(model.composer, "draft");
    assert_eq!(model.run_state, TuiRunState::Running);
    assert_eq!(
        model.agent_draft.as_ref().map(|entry| entry.text.as_str()),
        Some("working")
    );
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
fn palette_search_scroll_follow_and_quit_are_pure() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserOpenPalette);
    let (model, _) = reduce(model, TuiEvent::UserSearchChanged("daemon".into()));
    let (model, _) = reduce(model, TuiEvent::UserScroll(5));
    let (model, _) = reduce(model, TuiEvent::UserSelectPane(TuiPane::Tools));
    let (model, _) = reduce(model, TuiEvent::UserFollow(true));
    let (model, effects) = reduce(model, TuiEvent::QuitRequested);

    assert!(model.palette_open);
    assert_eq!(model.search, "daemon");
    assert_eq!(model.scroll, 0);
    assert!(model.follow);
    assert_eq!(model.active_pane, TuiPane::Tools);
    assert!(effects.contains(&TuiEffect::Quit));
}

#[test]
fn japanese_composer_moves_cursor_by_grapheme() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInsertChar('あ'));
    let (model, _) = reduce(model, TuiEvent::UserInsertChar('い'));
    let (model, _) = reduce(model, TuiEvent::UserInsertChar('う'));
    let (model, _) = reduce(model, TuiEvent::UserMoveComposer(-1));
    let (model, _) = reduce(model, TuiEvent::UserInsertChar('X'));

    assert_eq!(model.composer, "あいXう");
    assert!(model.composer.is_char_boundary(model.composer_cursor));
}

#[test]
fn backspace_removes_whole_emoji_grapheme() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("👨‍👩‍👧‍👦a".into()));
    let (model, _) = reduce(model, TuiEvent::UserMoveComposer(-1));
    let (model, _) = reduce(model, TuiEvent::UserBackspace);

    assert_eq!(model.composer, "a");
    assert_eq!(model.composer_cursor, 0);
}

#[test]
fn multiline_composer_keeps_newlines() {
    let (model, _) = reduce(
        TuiModel::new(),
        TuiEvent::UserInputChanged("line one".into()),
    );
    let (model, _) = reduce(model, TuiEvent::UserComposerNewline);
    let text = format!("{}line two", model.composer);
    let (model, _) = reduce(model, TuiEvent::UserInputChanged(text));

    assert_eq!(model.composer, "line one\nline two");
}

#[test]
fn transcript_save_persists_japanese_entries() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::temp_dir().join(format!("lkjagent-tui-{}", std::process::id()));
    if data.exists() {
        fs::remove_dir_all(&data)?;
    }
    let (model, _) = reduce(
        TuiModel::new(),
        TuiEvent::UserInputChanged("記録して".into()),
    );
    let (model, _) = reduce(model, TuiEvent::UserSubmit);
    let (model, _) = reduce(model, TuiEvent::AgentTextDelta("保存しました".into()));
    let (model, _) = reduce(model, TuiEvent::AgentMessageComplete);

    let path = lkjagent_app::tui_transcript::save(&data, &model, &TuiSnapshot::empty())
        .map_err(std::io::Error::other)?;
    let text = fs::read_to_string(path)?;

    assert!(text.contains("owner: 記録して"));
    assert!(text.contains("agent: 保存しました"));
    Ok(())
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

#[test]
fn tui_snapshot_shows_durable_agent_message_after_daemon_turn(
) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::temp_dir().join(format!("lkjagent-tui-agent-{}", std::process::id()));
    if data.exists() {
        fs::remove_dir_all(&data)?;
    }
    fs::create_dir_all(&data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, "What is an agent?", "now")?;
    drop(conn);

    let mut endpoint = ScriptedEndpoint {
        outputs: vec!["<message>done</message>".to_string()],
        index: 0,
    };
    run_until_idle(&data, &mut endpoint, 3)?;

    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    let snapshot = tui_snapshot::load(&conn, &data)?;

    assert!(snapshot.transcript.contains("agent: done"));
    Ok(())
}
