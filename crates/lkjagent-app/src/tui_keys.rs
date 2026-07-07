use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui_state::{reduce, TuiEffect, TuiEvent, TuiModel, TuiPane};

pub fn apply_key(model: &mut TuiModel, key: KeyEvent) -> Vec<TuiEffect> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return ctrl_key(model, key.code);
    }
    match key.code {
        KeyCode::Enter => apply_event(model, TuiEvent::UserSubmit),
        KeyCode::Backspace => apply_event(model, TuiEvent::UserBackspace),
        KeyCode::Left => apply_event(model, TuiEvent::UserMoveComposer(-1)),
        KeyCode::Right => apply_event(model, TuiEvent::UserMoveComposer(1)),
        KeyCode::Esc => apply_event(model, TuiEvent::UserCloseModal),
        KeyCode::Up => apply_event(model, TuiEvent::UserScroll(-1)),
        KeyCode::Down => apply_event(model, TuiEvent::UserScroll(1)),
        KeyCode::PageUp => apply_event(model, TuiEvent::UserScroll(-10)),
        KeyCode::PageDown => apply_event(model, TuiEvent::UserScroll(10)),
        KeyCode::Home => apply_event(model, TuiEvent::UserScroll(-1000)),
        KeyCode::End => apply_event(model, TuiEvent::UserFollow(true)),
        KeyCode::F(n) => apply_pane(model, n),
        KeyCode::Char(c) => apply_event(model, TuiEvent::UserInsertChar(c)),
        _ => vec![TuiEffect::Redraw],
    }
}

fn ctrl_key(model: &mut TuiModel, code: KeyCode) -> Vec<TuiEffect> {
    match code {
        KeyCode::Char('q') => apply_event(model, TuiEvent::QuitRequested),
        KeyCode::Char('c') => apply_event(model, TuiEvent::UserInterrupt),
        KeyCode::Char('p') => apply_event(model, TuiEvent::UserOpenPalette),
        KeyCode::Char('s') => apply_event(model, TuiEvent::SaveTranscript),
        KeyCode::Char('j') => apply_event(model, TuiEvent::UserComposerNewline),
        KeyCode::Char('l') => {
            apply_event(model, TuiEvent::UserSearchChanged(model.composer.clone()))
        }
        _ => vec![TuiEffect::Redraw],
    }
}

fn apply_pane(model: &mut TuiModel, n: u8) -> Vec<TuiEffect> {
    let pane = match n {
        1 => TuiPane::Transcript,
        2 => TuiPane::Tasks,
        3 => TuiPane::Tools,
        4 => TuiPane::StateGraph,
        5 => TuiPane::Workspace,
        6 => TuiPane::Help,
        _ => return vec![TuiEffect::Redraw],
    };
    apply_event(model, TuiEvent::UserSelectPane(pane))
}

fn apply_event(model: &mut TuiModel, event: TuiEvent) -> Vec<TuiEffect> {
    let (next, effects) = reduce(model.clone(), event);
    *model = next;
    effects
}
