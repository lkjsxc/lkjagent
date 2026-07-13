use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use unicode_segmentation::UnicodeSegmentation;

use crate::tui_model::{ComposerEvent, TuiEvent, TuiModel};

pub enum InputAction {
    Reduce(TuiEvent),
    Submit,
    LoadOlderAndScroll(isize),
    Quit,
}

pub fn map(event: Event, model: &TuiModel) -> Vec<InputAction> {
    match event {
        Event::Resize(width, height) => vec![reduce(TuiEvent::Resize {
            width: usize::from(width),
            height: usize::from(height),
        })],
        Event::Paste(text) => vec![text_input(model, text, true)],
        Event::Key(key) if key.kind != KeyEventKind::Release => key_actions(key, model),
        _ => Vec::new(),
    }
}

fn key_actions(key: KeyEvent, model: &TuiModel) -> Vec<InputAction> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('c' | 'C') => vec![InputAction::Quit],
            KeyCode::Char('f' | 'F') => vec![reduce(TuiEvent::SearchMode(true))],
            _ => Vec::new(),
        };
    }
    match key.code {
        KeyCode::Esc => vec![reduce(TuiEvent::SearchMode(false))],
        KeyCode::F(2) => vec![reduce(TuiEvent::ActivityExpanded(
            !model.screen.activity.expanded,
        ))],
        KeyCode::Enter if !model.search_active => vec![InputAction::Submit],
        KeyCode::Enter => vec![reduce(TuiEvent::SearchMode(false))],
        KeyCode::PageUp => vec![InputAction::LoadOlderAndScroll(page(model, -1))],
        KeyCode::PageDown => vec![reduce(TuiEvent::Scroll(page(model, 1)))],
        KeyCode::Up => vec![InputAction::LoadOlderAndScroll(-1)],
        KeyCode::Down => vec![reduce(TuiEvent::Scroll(1))],
        KeyCode::End => vec![
            reduce(TuiEvent::Composer(ComposerEvent::End)),
            reduce(TuiEvent::Scroll(isize::MAX)),
        ],
        KeyCode::Home => vec![composer(ComposerEvent::Home)],
        KeyCode::Left => vec![composer(ComposerEvent::MoveLeft)],
        KeyCode::Right => vec![composer(ComposerEvent::MoveRight)],
        KeyCode::Backspace if model.search_active => {
            vec![reduce(TuiEvent::Search(search_backspace(&model.search)))]
        }
        KeyCode::Backspace => vec![composer(ComposerEvent::Backspace)],
        KeyCode::Delete if model.search_active => {
            vec![reduce(TuiEvent::Search(String::new()))]
        }
        KeyCode::Delete => vec![composer(ComposerEvent::Delete)],
        KeyCode::Char(character) => vec![text_input(model, character.to_string(), false)],
        _ => Vec::new(),
    }
}

fn text_input(model: &TuiModel, text: String, paste: bool) -> InputAction {
    if model.search_active {
        let mut next = model.search.clone();
        if next.len().saturating_add(text.len()) <= 1_024 {
            next.push_str(&text);
        }
        reduce(TuiEvent::Search(next))
    } else {
        let event = if paste {
            ComposerEvent::Paste(text)
        } else {
            ComposerEvent::Insert(text)
        };
        composer(event)
    }
}

fn search_backspace(search: &str) -> String {
    let end = search
        .grapheme_indices(true)
        .next_back()
        .map_or(0, |(at, _)| at);
    search[..end].to_string()
}

fn page(model: &TuiModel, direction: isize) -> isize {
    let amount = model.height.saturating_sub(4).max(1);
    isize::try_from(amount).unwrap_or(isize::MAX) * direction
}

fn composer(event: ComposerEvent) -> InputAction {
    reduce(TuiEvent::Composer(event))
}

fn reduce(event: TuiEvent) -> InputAction {
    InputAction::Reduce(event)
}
