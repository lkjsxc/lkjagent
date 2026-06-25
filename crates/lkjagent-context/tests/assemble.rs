use lkjagent_context::assemble::assemble_messages;
use lkjagent_context::model::{ContextState, Frame, FrameKind, PrefixSection, Role};

#[test]
fn invalid_assistant_history_is_not_replayed_as_assistant() {
    let state = ContextState::new(
        vec![Frame::new(
            FrameKind::Prefix(PrefixSection::Identity),
            "identity",
            1,
        )],
        vec![Frame::new(
            FrameKind::ModelTurn,
            "<think>bad</think>\n<action>\n<tool>graph.state</tool>\n</action>",
            10,
        )],
    );

    let messages = assemble_messages(&state);

    assert_eq!(messages.len(), 2);
    assert_eq!(messages[1].role, Role::User);
    assert!(messages[1].content.contains("invalid-assistant-history"));
    assert!(messages[1].content.contains("hidden-reasoning"));
    assert!(!messages
        .iter()
        .any(|message| { message.role == Role::Assistant && message.content.contains("<think>") }));
}

#[test]
fn valid_action_history_stays_assistant() {
    let action = "<action>\n<tool>graph.state</tool>\n</action>";
    let state = ContextState::new(
        vec![Frame::new(
            FrameKind::Prefix(PrefixSection::Identity),
            "identity",
            1,
        )],
        vec![Frame::new(FrameKind::ModelTurn, action, 5)],
    );

    let messages = assemble_messages(&state);

    assert_eq!(messages.len(), 2);
    assert_eq!(messages[1].role, Role::Assistant);
    assert_eq!(messages[1].content, action);
}
