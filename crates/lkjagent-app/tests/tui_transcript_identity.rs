use lkjagent_app::tui_event::TuiEvent;
use lkjagent_app::tui_snapshot::TuiSnapshot;
use lkjagent_app::tui_state::{reduce, TranscriptEntry, TranscriptSource, TuiModel};

#[test]
fn streaming_deltas_commit_as_one_agent_message() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::AgentTextDelta("hel".into()));
    let (model, _) = reduce(model, TuiEvent::AgentTextDelta("lo".into()));

    assert_eq!(model.transcript.len(), 0);
    assert_eq!(
        model.agent_draft.as_ref().map(|entry| entry.text.as_str()),
        Some("hello")
    );

    let (model, _) = reduce(model, TuiEvent::AgentMessageComplete);

    assert!(model.agent_draft.is_none());
    assert_eq!(model.transcript.len(), 1);
    assert_eq!(model.transcript[0].text, "hello");
}

#[test]
fn identical_text_with_different_ids_remains_visible() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::UserInputChanged("same".into()));
    let (model, _) = reduce(model, TuiEvent::UserSubmit);
    let (model, _) = reduce(model, TuiEvent::UserInputChanged("same".into()));
    let (model, _) = reduce(model, TuiEvent::UserSubmit);

    let lines = lkjagent_app::tui_transcript::display_lines(&model, &TuiSnapshot::empty());

    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "owner: same")
            .count(),
        2
    );
}

#[test]
fn durable_row_overrides_matching_ephemeral_id() {
    let (model, _) = reduce(TuiModel::new(), TuiEvent::AgentTextDelta("local".into()));
    let (model, _) = reduce(model, TuiEvent::AgentMessageComplete);
    let mut snapshot = TuiSnapshot::empty();
    snapshot
        .transcript_entries
        .push(agent_entry("agent:session:1", "durable", "sqlite:events:1"));

    let lines = lkjagent_app::tui_transcript::display_lines(&model, &snapshot);

    assert_eq!(lines, vec!["agent: durable"]);
}

#[test]
fn durable_rows_shadow_matching_identity_only() {
    let mut model = TuiModel::new();
    model
        .transcript
        .push(agent_entry("event:1", "done", "sqlite:events:1"));
    model
        .transcript
        .push(owner_entry("queue:7", "same", "sqlite:queue:7"));
    model
        .transcript
        .push(owner_entry("owner:session:2", "same", "session"));
    let mut snapshot = TuiSnapshot::empty();
    snapshot
        .transcript_entries
        .push(agent_entry("event:1", "done", "sqlite:events:1"));
    snapshot
        .transcript_entries
        .push(owner_entry("queue:7", "same", "sqlite:queue:7"));
    snapshot
        .transcript_entries
        .push(owner_entry("queue:8", "same", "sqlite:queue:8"));

    let lines = lkjagent_app::tui_transcript::display_lines(&model, &snapshot);

    assert_eq!(
        lines,
        vec!["agent: done", "owner: same", "owner: same", "owner: same"]
    );
}

#[test]
fn saved_transcript_includes_ids_and_paths() {
    let mut snapshot = TuiSnapshot::empty();
    snapshot
        .transcript_entries
        .push(owner_entry("queue:7", "hello", "sqlite:queue:7"));

    let text = lkjagent_app::tui_transcript::text(&TuiModel::new(), &snapshot);

    assert!(text.contains("id=queue:7"));
    assert!(text.contains("path=sqlite:queue:7"));
    assert!(text.contains("owner: hello"));
}

#[test]
fn display_transcript_hides_diagnostics_from_conversation() {
    let mut model = TuiModel::new();
    model.transcript.push(entry(
        "tool:session:1",
        TranscriptSource::Tool,
        "state: queue debug",
        "session",
    ));
    model.transcript.push(entry(
        "state:session:2",
        TranscriptSource::State,
        "state: queue: 1",
        "session",
    ));
    model
        .transcript
        .push(agent_entry("agent:session:3", "visible", "session"));

    let lines = lkjagent_app::tui_transcript::display_lines(&model, &TuiSnapshot::empty());

    assert_eq!(lines, vec!["agent: visible"]);
}

#[test]
fn clamped_viewport_windows_do_not_show_blank_space() {
    let text = (1..=4)
        .map(|index| format!("line-{index}"))
        .collect::<Vec<_>>()
        .join("\n");

    assert_eq!(lkjagent_app::tui_state::window_start(4, 3, false, 99), 1);
    assert_eq!(
        lkjagent_app::tui_state::visible_text(&text, 3, false, 99),
        "line-2\nline-3\nline-4"
    );
}

#[test]
fn slash_commands_submit_without_owner_transcript_echo() {
    let (model, _) = lkjagent_app::tui_state::reduce(
        TuiModel::new(),
        lkjagent_app::tui_event::TuiEvent::UserInputChanged("/status".into()),
    );
    let (model, effects) =
        lkjagent_app::tui_state::reduce(model, lkjagent_app::tui_event::TuiEvent::UserSubmit);

    assert!(matches!(
        effects.iter().find(|effect| matches!(effect, lkjagent_app::tui_state::TuiEffect::SubmitOwnerMessage(_))),
        Some(lkjagent_app::tui_state::TuiEffect::SubmitOwnerMessage(text)) if text == "/status"
    ));
    assert!(model.transcript.is_empty());
}

fn agent_entry(id: &str, text: &str, path: &str) -> TranscriptEntry {
    entry(id, TranscriptSource::Agent, text, path)
}

fn owner_entry(id: &str, text: &str, path: &str) -> TranscriptEntry {
    entry(id, TranscriptSource::Owner, text, path)
}

fn entry(id: &str, source: TranscriptSource, text: &str, path: &str) -> TranscriptEntry {
    TranscriptEntry {
        id: id.to_string(),
        source,
        text: text.to_string(),
        path: Some(path.to_string()),
    }
}
