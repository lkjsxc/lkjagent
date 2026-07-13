use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::tui_model::{ComposerEvent, ComposerState, PendingSubmission, TuiEffect};

pub fn reduce(mut state: ComposerState, event: ComposerEvent) -> (ComposerState, Vec<TuiEffect>) {
    let mut effects = Vec::new();
    match event {
        ComposerEvent::Replace(text) => replace(&mut state, text),
        ComposerEvent::Insert(text) | ComposerEvent::Paste(text) => insert(&mut state, &text),
        ComposerEvent::Backspace => backspace(&mut state),
        ComposerEvent::Delete => delete(&mut state),
        ComposerEvent::MoveLeft => state.cursor = state.cursor.saturating_sub(1),
        ComposerEvent::MoveRight => {
            state.cursor = state.cursor.saturating_add(1).min(count(&state.text));
        }
        ComposerEvent::Home => state.cursor = 0,
        ComposerEvent::End => state.cursor = count(&state.text),
        ComposerEvent::Submit { message_id } => submit(&mut state, message_id, &mut effects),
        ComposerEvent::SubmitSucceeded { message_id } => succeed(&mut state, &message_id),
        ComposerEvent::SubmitFailed { message_id, error } => {
            fail(&mut state, &message_id, error);
        }
    }
    (state, effects)
}

pub fn cursor_display_column(state: &ComposerState) -> usize {
    let byte = byte_index(&state.text, state.cursor.min(count(&state.text)));
    let prefix = &state.text[..byte];
    UnicodeWidthStr::width(prefix.rsplit('\n').next().unwrap_or(""))
}

fn replace(state: &mut ComposerState, text: String) {
    state.text = text;
    state.cursor = count(&state.text);
    changed(state);
}

fn insert(state: &mut ComposerState, text: &str) {
    clamp(state);
    let byte = byte_index(&state.text, state.cursor);
    state.text.insert_str(byte, text);
    state.cursor = count(&state.text[..byte.saturating_add(text.len())]);
    changed(state);
}

fn backspace(state: &mut ComposerState) {
    clamp(state);
    if state.cursor == 0 {
        return;
    }
    let end = byte_index(&state.text, state.cursor);
    state.cursor -= 1;
    let start = byte_index(&state.text, state.cursor);
    state.text.replace_range(start..end, "");
    changed(state);
}

fn delete(state: &mut ComposerState) {
    clamp(state);
    if state.cursor == count(&state.text) {
        return;
    }
    let start = byte_index(&state.text, state.cursor);
    let end = byte_index(&state.text, state.cursor + 1);
    state.text.replace_range(start..end, "");
    changed(state);
}

fn submit(state: &mut ComposerState, message_id: String, effects: &mut Vec<TuiEffect>) {
    if state.text.trim().is_empty() || state.pending.is_some() {
        return;
    }
    let body = state.text.clone();
    state.last_error = None;
    state.pending = Some(PendingSubmission {
        message_id: message_id.clone(),
        body: body.clone(),
        revision: state.revision,
        durable: false,
    });
    effects.push(TuiEffect::CommitOwnerMessage { message_id, body });
}

fn succeed(state: &mut ComposerState, message_id: &str) {
    let Some(pending) = state.pending.as_mut() else {
        return;
    };
    if pending.message_id != message_id {
        return;
    }
    pending.durable = true;
    state.last_error = None;
    if state.revision == pending.revision {
        state.text.clear();
        state.cursor = 0;
        state.revision = state.revision.saturating_add(1);
    }
}

fn fail(state: &mut ComposerState, message_id: &str, error: String) {
    if state.pending.as_ref().map(|item| item.message_id.as_str()) == Some(message_id) {
        state.pending = None;
        state.last_error = Some(error);
    }
}

fn changed(state: &mut ComposerState) {
    state.revision = state.revision.saturating_add(1);
    state.last_error = None;
}

fn clamp(state: &mut ComposerState) {
    state.cursor = state.cursor.min(count(&state.text));
}

fn byte_index(text: &str, cursor: usize) -> usize {
    text.grapheme_indices(true)
        .nth(cursor)
        .map_or(text.len(), |(index, _)| index)
}

fn count(text: &str) -> usize {
    text.graphemes(true).count()
}
