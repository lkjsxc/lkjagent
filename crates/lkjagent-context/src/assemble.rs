use crate::model::{ContextState, Frame, FrameKind, Message, Role};

pub fn assemble_messages(state: &ContextState) -> Vec<Message> {
    let mut messages = vec![Message::new(Role::System, render_prefix(&state.prefix))];
    for frame in &state.log {
        match frame.kind {
            FrameKind::ModelTurn => messages.push(Message::new(Role::Assistant, &frame.content)),
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

fn append_user_frame(messages: &mut Vec<Message>, frame: &Frame) {
    if let Some(last) = messages.last_mut() {
        if last.role == Role::User {
            last.content.push('\n');
            last.content.push_str(&frame.content);
            return;
        }
    }
    messages.push(Message::new(Role::User, &frame.content));
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
