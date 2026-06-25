use crate::model::{ContextState, Frame, FrameKind, Message, Role};

pub fn assemble_messages(state: &ContextState) -> Vec<Message> {
    let mut messages = vec![Message::new(Role::System, render_prefix(&state.prefix))];
    for frame in &state.log {
        match frame.kind {
            FrameKind::ModelTurn => append_assistant_frame(&mut messages, frame),
            _ => append_user_frame(&mut messages, frame),
        }
    }
    messages
}

pub fn serialize_request(state: &ContextState) -> String {
    let mut bytes = String::from("system\n");
    bytes.push_str(&render_prefix(&state.prefix));
    bytes.push_str("\nlog\n");
    for frame in &state.log {
        bytes.push_str(role_name(role_for(frame)));
        bytes.push('\n');
        bytes.push_str(&frame.content);
        bytes.push('\n');
    }
    bytes
}

pub fn append_frame(state: &ContextState, frame: Frame) -> ContextState {
    let mut next = state.clone();
    next.log.push(frame);
    next
}

fn render_prefix(prefix: &[Frame]) -> String {
    prefix
        .iter()
        .map(|frame| frame.content.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn append_assistant_frame(messages: &mut Vec<Message>, frame: &Frame) {
    if valid_assistant_action(&frame.content) {
        messages.push(Message::new(Role::Assistant, &frame.content));
        return;
    }
    let summary = format!(
        "<notice>\n<kind>invalid-assistant-history</kind>\n<content>previous assistant turn omitted from assistant replay; reason={}</content>\n</notice>",
        invalid_reason(&frame.content)
    );
    append_user_content(messages, &summary);
}

fn append_user_frame(messages: &mut Vec<Message>, frame: &Frame) {
    append_user_content(messages, &frame.content);
}

fn append_user_content(messages: &mut Vec<Message>, content: &str) {
    if let Some(last) = messages.last_mut() {
        if last.role == Role::User {
            last.content.push('\n');
            last.content.push_str(content);
            return;
        }
    }
    messages.push(Message::new(Role::User, content));
}

fn valid_assistant_action(content: &str) -> bool {
    let trimmed = content.trim();
    trimmed.starts_with("<action>")
        && trimmed.ends_with("</action>")
        && !contains_hidden_reasoning(trimmed)
        && trimmed.matches("<action>").count() == 1
        && trimmed.matches("</action>").count() == 1
}

fn invalid_reason(content: &str) -> &'static str {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        "empty-content"
    } else if contains_hidden_reasoning(trimmed) {
        "hidden-reasoning"
    } else if !trimmed.starts_with("<action>") {
        "prose-before-action"
    } else {
        "invalid-action-envelope"
    }
}

fn contains_hidden_reasoning(content: &str) -> bool {
    content.to_ascii_lowercase().contains("<think")
}

fn role_for(frame: &Frame) -> Role {
    match frame.kind {
        FrameKind::ModelTurn => Role::Assistant,
        _ => Role::User,
    }
}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::Assistant => "assistant",
        Role::User => "user",
    }
}
