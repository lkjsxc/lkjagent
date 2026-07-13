use lkjagent_app::tui_composer::{cursor_display_column, reduce as compose};
use lkjagent_app::tui_model::{ComposerEvent as C, TuiEffect, TuiEvent, TuiModel};
use lkjagent_app::tui_reducer::reduce;
use lkjagent_app::tui_screen::ScreenModel;
use lkjagent_app::tui_viewport::{scroll, visible, Anchor, ViewRow, Viewport};
use lkjagent_app::tui_wrap::{display_width, wrap};
use lkjagent_store::tui_snapshot::{ConversationRow, StatusCounts, TuiSnapshot};

fn message(id: &str, sequence: i64, body: &str) -> ConversationRow {
    ConversationRow {
        id: id.into(),
        sequence,
        role: "owner".into(),
        body: body.as_bytes().to_vec(),
        body_truncated: false,
        lifecycle: "active".into(),
        matter_id: "m".into(),
        replacement_id: None,
    }
}

fn snapshot(messages: Vec<ConversationRow>) -> TuiSnapshot {
    TuiSnapshot {
        conversation: messages,
        activity: Vec::new(),
        status: StatusCounts::default(),
    }
}

fn rows(count: usize) -> Vec<ViewRow> {
    (0..count)
        .map(|index| ViewRow {
            message_id: format!("m{index}"),
            wrapped_row: 0,
            role: "owner".into(),
            text: format!("row {index}"),
        })
        .collect()
}

fn manual(id: &str, row: usize) -> Viewport {
    Viewport::Manual(Anchor {
        message_id: id.into(),
        wrapped_row: row,
    })
}

#[test]
fn identity_is_exact_once_and_never_body_based() {
    let (model, _) = reduce(
        TuiModel::new(20, 4),
        TuiEvent::Composer(C::Replace("same".into())),
    );
    let (model, _) = reduce(
        model,
        TuiEvent::Composer(C::Submit {
            message_id: "eventual".into(),
        }),
    );
    let same = snapshot(vec![message("a", 1, "same"), message("b", 2, "same")]);
    let (model, _) = reduce(model, TuiEvent::Snapshot(same));
    assert_eq!(model.screen.conversation.len(), 3);
    assert!(model.screen.conversation.iter().any(|m| !m.durable));
    assert_eq!(model.composer.text, "same");

    let durable = snapshot(vec![
        message("a", 1, "same"),
        message("b", 2, "same"),
        message("eventual", 3, "same"),
    ]);
    let (model, _) = reduce(model, TuiEvent::Snapshot(durable));
    assert_eq!(model.screen.conversation.len(), 3);
    assert!(model.screen.conversation.iter().all(|m| m.durable));
    assert!(model.composer.text.is_empty());
    assert!(!model.screen.activity.expanded);
}

#[test]
fn composer_edits_graphemes_and_reports_display_columns() {
    let (state, _) = compose(Default::default(), C::Insert("日e".into()));
    let (state, _) = compose(state, C::Insert("\u{301}".into()));
    let (state, _) = compose(state, C::Paste("👩‍💻".into()));
    assert_eq!(state.cursor, 3);
    assert_eq!(cursor_display_column(&state), 5);
    assert_eq!(display_width(&state.text), 5);
    let (state, _) = compose(state, C::MoveLeft);
    assert_eq!(cursor_display_column(&state), 3);
    let (state, _) = compose(state, C::Backspace);
    assert_eq!(state.text, "日👩‍💻");
    let (state, _) = compose(state, C::Delete);
    assert_eq!(state.text, "日");
    let (state, _) = compose(state, C::Paste("かな".into()));
    assert_eq!(
        (state.text.as_str(), cursor_display_column(&state)),
        ("日かな", 6)
    );
    assert_eq!(wrap("日本e\u{301}👩‍💻", 4), ["日本", "e\u{301}👩‍💻"]);
}

#[test]
fn submit_waits_for_durability_and_preserves_failures_and_later_edits() {
    let (state, _) = compose(Default::default(), C::Replace("first".into()));
    let (state, effects) = compose(
        state,
        C::Submit {
            message_id: "m1".into(),
        },
    );
    assert_eq!(state.text, "first");
    assert_eq!(
        effects,
        [TuiEffect::CommitOwnerMessage {
            message_id: "m1".into(),
            body: "first".into()
        }]
    );
    let (failed, _) = compose(
        state,
        C::SubmitFailed {
            message_id: "m1".into(),
            error: "busy".into(),
        },
    );
    assert_eq!(
        (failed.text.as_str(), failed.last_error.as_deref()),
        ("first", Some("busy"))
    );

    let (state, _) = compose(
        failed,
        C::Submit {
            message_id: "m2".into(),
        },
    );
    let (edited, _) = compose(state, C::Insert(" later".into()));
    let (edited, _) = compose(
        edited,
        C::SubmitSucceeded {
            message_id: "m2".into(),
        },
    );
    assert_eq!(edited.text, "first later");
    let (plain, _) = compose(Default::default(), C::Replace("done".into()));
    let (plain, _) = compose(
        plain,
        C::Submit {
            message_id: "m3".into(),
        },
    );
    let (plain, _) = compose(
        plain,
        C::SubmitSucceeded {
            message_id: "m3".into(),
        },
    );
    assert!(plain.text.is_empty());
}

#[test]
fn follow_and_manual_append_have_durable_anchors() {
    let initial = rows(6);
    let mut viewport = Viewport::Follow;
    assert_eq!(visible(&viewport, &initial, 2)[0].message_id, "m4");
    scroll(&mut viewport, &initial, 2, -2);
    assert_eq!(viewport, manual("m2", 0));
    let appended = rows(8);
    assert_eq!(visible(&viewport, &appended, 2)[0].message_id, "m2");
    scroll(&mut viewport, &appended, 2, 99);
    assert_eq!(viewport, Viewport::Follow);
    assert_eq!(visible(&viewport, &appended, 2)[0].message_id, "m6");
}

#[test]
fn resize_search_shrink_and_overscroll_clamp_without_blank_windows() {
    let items = snapshot(vec![message("a", 1, "abcdefgh"), message("b", 2, "needle")]);
    let screen = ScreenModel::project(&items, None);
    let wide = screen.rows(4, "");
    let mut viewport = manual("a", 1);
    assert_eq!(visible(&viewport, &wide, 2)[0].text, "efgh");
    let narrow = screen.rows(2, "");
    assert_eq!(visible(&viewport, &narrow, 2)[0].text, "cd");
    let searched = screen.rows(2, "needle");
    assert_eq!(visible(&viewport, &searched, 2)[0].message_id, "b");
    assert_eq!(visible(&viewport, &narrow, 2)[0].message_id, "a");

    let shrunk = ScreenModel::project(&snapshot(vec![message("a", 1, "x")]), None).rows(2, "");
    lkjagent_app::tui_viewport::reconcile(&mut viewport, &shrunk, 3);
    assert_eq!(viewport, manual("a", 0));
    assert_eq!(visible(&viewport, &shrunk, 99).len(), 1);
    scroll(&mut viewport, &shrunk, 1, 999);
    assert_eq!(viewport, Viewport::Follow);
    assert!(!visible(&viewport, &shrunk, 1).is_empty());
}
