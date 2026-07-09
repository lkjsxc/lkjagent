use unicode_segmentation::UnicodeSegmentation;

use crate::tui_transcript::{append_agent_draft, complete_agent_draft, push_entry};
use crate::tui_types::{ToolCard, TranscriptSource, TuiEffect, TuiEvent, TuiModel, TuiRunState};

pub fn reduce(mut model: TuiModel, event: TuiEvent) -> (TuiModel, Vec<TuiEffect>) {
    let mut effects = vec![TuiEffect::Redraw];
    match event {
        TuiEvent::UserInputChanged(text) => set_composer(&mut model, text),
        TuiEvent::UserInsertChar(character) => insert_text(&mut model, &character.to_string()),
        TuiEvent::UserComposerNewline => insert_text(&mut model, "\n"),
        TuiEvent::UserBackspace => backspace(&mut model),
        TuiEvent::UserDelete => delete(&mut model),
        TuiEvent::UserMoveComposer(delta) => move_cursor(&mut model, delta),
        TuiEvent::UserComposerHome => model.composer_cursor = 0,
        TuiEvent::UserComposerEnd => model.composer_cursor = grapheme_count(&model.composer),
        TuiEvent::UserSubmit => submit_composer(&mut model, &mut effects),
        TuiEvent::UserInterrupt => interrupt(&mut model, &mut effects),
        TuiEvent::UserApproveTool => approve_tool(&mut model, &mut effects),
        TuiEvent::UserRejectTool(reason) => reject_tool(&mut model, &mut effects, reason),
        TuiEvent::UserOpenPalette => model.palette_open = true,
        TuiEvent::UserCloseModal => model.palette_open = false,
        TuiEvent::UserSearchChanged(query) => {
            model.search = query;
            model.follow = false;
        }
        TuiEvent::UserScroll(delta) => {
            model.follow = false;
            model.scroll = scroll(model.scroll, delta);
        }
        TuiEvent::UserFollow(enabled) => {
            model.follow = enabled;
            if enabled {
                model.scroll = 0;
            }
        }
        TuiEvent::UserSelectPane(pane) => model.active_pane = pane,
        TuiEvent::AgentTextDelta(text) => {
            model.run_state = TuiRunState::Running;
            append_agent_draft(&mut model, &text);
        }
        TuiEvent::AgentMessageComplete => complete_agent_draft(&mut model),
        TuiEvent::ToolCallProposed { name, decision_id } => {
            model.run_state = TuiRunState::ToolPending;
            model.pending_tool = Some(ToolCard { name, decision_id });
        }
        TuiEvent::ToolCallStarted(name) => {
            model.run_state = TuiRunState::ToolRunning;
            push_entry(
                &mut model,
                TranscriptSource::Tool,
                format!("started {name}"),
            );
        }
        TuiEvent::ToolCallFinished(summary) => {
            model.run_state = TuiRunState::Running;
            push_entry(&mut model, TranscriptSource::Tool, summary);
        }
        TuiEvent::StateTransitionObserved(text) => {
            push_entry(&mut model, TranscriptSource::State, text);
        }
        TuiEvent::ArtifactCreated(text) => push_entry(&mut model, TranscriptSource::System, text),
        TuiEvent::WorkspaceChanged(text) => push_entry(&mut model, TranscriptSource::System, text),
        TuiEvent::TimerTick => {}
        TuiEvent::TerminalResize { width, height } => {
            model.width = width.max(40);
            model.height = height.max(10);
        }
        TuiEvent::ErrorObserved(text) => {
            model.last_error = Some(text.clone());
            push_entry(&mut model, TranscriptSource::Error, text);
        }
        TuiEvent::SaveTranscript => effects.push(TuiEffect::SaveTranscript),
        TuiEvent::QuitRequested => effects.push(TuiEffect::Quit),
    }
    (model, effects)
}

fn set_composer(model: &mut TuiModel, text: String) {
    model.composer = text;
    model.composer_cursor = grapheme_count(&model.composer);
}

fn insert_text(model: &mut TuiModel, text: &str) {
    clamp_cursor(model);
    let byte = byte_cursor(model);
    model.composer.insert_str(byte, text);
    model.composer_cursor += grapheme_count(text);
}

fn backspace(model: &mut TuiModel) {
    clamp_cursor(model);
    if model.composer_cursor == 0 {
        return;
    }
    let end = byte_cursor(model);
    model.composer_cursor -= 1;
    let start = byte_cursor(model);
    model.composer.replace_range(start..end, "");
}

fn delete(model: &mut TuiModel) {
    clamp_cursor(model);
    let count = grapheme_count(&model.composer);
    if model.composer_cursor >= count {
        return;
    }
    let start = byte_cursor(model);
    let end = byte_index(&model.composer, model.composer_cursor + 1);
    model.composer.replace_range(start..end, "");
}

fn move_cursor(model: &mut TuiModel, delta: isize) {
    clamp_cursor(model);
    model.composer_cursor = if delta.is_negative() {
        model.composer_cursor.saturating_sub(delta.unsigned_abs())
    } else {
        model
            .composer_cursor
            .saturating_add(delta as usize)
            .min(grapheme_count(&model.composer))
    };
}

fn clamp_cursor(model: &mut TuiModel) {
    model.composer_cursor = model.composer_cursor.min(grapheme_count(&model.composer));
}

fn byte_cursor(model: &TuiModel) -> usize {
    byte_index(&model.composer, model.composer_cursor)
}

fn byte_index(text: &str, cursor: usize) -> usize {
    text.grapheme_indices(true)
        .nth(cursor)
        .map(|(index, _)| index)
        .unwrap_or(text.len())
}

fn grapheme_count(text: &str) -> usize {
    text.graphemes(true).count()
}

fn submit_composer(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    let text = model.composer.trim().to_string();
    if text.is_empty() {
        return;
    }
    model.composer.clear();
    model.composer_cursor = 0;
    model.palette_open = false;
    push_entry(model, TranscriptSource::Owner, text.clone());
    effects.push(TuiEffect::SubmitOwnerMessage(text));
}

fn interrupt(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    model.run_state = TuiRunState::Interrupted;
    push_entry(model, TranscriptSource::System, "interrupt requested");
    effects.push(TuiEffect::InterruptRun);
}

fn approve_tool(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    if let Some(card) = model.pending_tool.take() {
        model.run_state = TuiRunState::ToolRunning;
        effects.push(TuiEffect::ApproveTool(card));
    }
}

fn reject_tool(model: &mut TuiModel, effects: &mut Vec<TuiEffect>, reason: String) {
    if let Some(card) = model.pending_tool.take() {
        model.run_state = TuiRunState::Running;
        effects.push(TuiEffect::RejectTool { card, reason });
    }
}

fn scroll(current: usize, delta: isize) -> usize {
    if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize)
    }
}
