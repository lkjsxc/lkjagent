use lkjagent_app::tui_composer::reduce as compose;
use lkjagent_app::tui_model::{ComposerEvent as C, TuiEvent, TuiModel};
use lkjagent_app::tui_screen::{ActivityItem, ConversationItem, ScreenModel};
use lkjagent_app::tui_viewport::{Anchor, Viewport};
use lkjagent_store::tui_snapshot::{
    self, ConversationRow, SnapshotPage, StatusCounts, TuiSnapshot,
};
use rusqlite::Connection;
use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(1);

type TestResult = Result<(), Box<dyn Error>>;

fn path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "lkjagent-tui-{label}-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}

fn message(id: &str, sequence: i64, body: &str) -> ConversationRow {
    ConversationRow {
        id: id.into(),
        sequence,
        role: "owner".into(),
        body: body.as_bytes().to_vec(),
        body_truncated: false,
        lifecycle: "active".into(),
        matter_id: "matter".into(),
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

#[test]
fn renderer_clips_and_keeps_activity_out_of_conversation() {
    let mut model = TuiModel::new(12, 8);
    model.screen.conversation.push(ConversationItem {
        id: "m1".into(),
        sequence: Some(1),
        role: "owner".into(),
        body: "日本語 long body".into(),
        lifecycle: "active".into(),
        durable: true,
    });
    model.screen.activity.expanded = true;
    model.screen.activity.items.push(ActivityItem {
        id: "decision/d1".into(),
        kind: "decision".into(),
        matter_id: "matter".into(),
        status: "selected".into(),
        monotonic_ms: 1,
    });
    let lines = lkjagent_app::tui_render::lines(&model);
    assert!(lines.iter().any(|line| line.starts_with("owner:")));
    let activity = lines.iter().position(|line| line.starts_with("activity"));
    assert!(activity.is_some());
    assert!(lines
        .iter()
        .all(|line| lkjagent_app::tui_wrap::display_width(line) <= 12));
    assert!(lines
        .iter()
        .take(activity.unwrap_or(0))
        .all(|line| !line.contains("selected")));
}

#[test]
fn older_pages_merge_by_identity_in_sequence_order() {
    let mut screen = ScreenModel::project(
        &snapshot(vec![message("m3", 3, "three"), message("m4", 4, "four")]),
        None,
    );
    screen.merge(
        &snapshot(vec![
            message("m1", 1, "one"),
            message("m2", 2, "two"),
            message("m3", 3, "three updated"),
        ]),
        None,
    );
    let ids = screen
        .conversation
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, ["m1", "m2", "m3", "m4"]);
    assert_eq!(screen.conversation[2].body, "three updated");
}

#[test]
fn older_page_merge_preserves_logical_anchor() {
    let mut model = TuiModel::new(20, 4);
    model.viewport = Viewport::Manual(Anchor {
        message_id: "m3".into(),
        wrapped_row: 0,
    });
    let (model, _) = lkjagent_app::tui_reducer::reduce(
        model,
        TuiEvent::Snapshot(snapshot(vec![message("m3", 3, "three")])),
    );
    let (model, _) = lkjagent_app::tui_reducer::reduce(
        model,
        TuiEvent::Snapshot(snapshot(vec![
            message("m1", 1, "one"),
            message("m2", 2, "two"),
        ])),
    );
    assert_eq!(
        model.viewport,
        Viewport::Manual(Anchor {
            message_id: "m3".into(),
            wrapped_row: 0
        })
    );
}

#[test]
fn typed_intake_uses_durable_id_and_preserves_failed_text() -> TestResult {
    let data = path("submit");
    let (state, _) = compose(Default::default(), C::Replace("same 日本".into()));
    let (state, _) = compose(
        state,
        C::Submit {
            message_id: "request".into(),
        },
    );
    let receipt = lkjagent_app::public_loop::send_message(&data, "same 日本", false)?;
    let (state, _) = compose(
        state,
        C::SubmitCommitted {
            request_id: "request".into(),
            message_id: receipt.message_id.clone(),
        },
    );
    assert!(state.text.is_empty());
    let mut connection = Connection::open(data.join("lkjagent.sqlite3"))?;
    let frame = tui_snapshot::snapshot(
        &mut connection,
        &SnapshotPage {
            conversation_before: None,
            conversation_limit: 10,
            activity_before: None,
            activity_limit: 10,
        },
    )?;
    assert_eq!(frame.conversation[0].id, receipt.message_id);

    let bad = path("not-directory");
    std::fs::write(&bad, b"file")?;
    let (failed, _) = compose(Default::default(), C::Replace("retain".into()));
    let (failed, _) = compose(
        failed,
        C::Submit {
            message_id: "bad".into(),
        },
    );
    let error = lkjagent_app::public_loop::send_message(&bad, "retain", false)
        .err()
        .ok_or("intake unexpectedly succeeded")?;
    let (failed, _) = compose(
        failed,
        C::SubmitFailed {
            message_id: "bad".into(),
            error,
        },
    );
    assert_eq!(failed.text, "retain");
    assert!(failed.last_error.is_some());
    Ok(())
}
