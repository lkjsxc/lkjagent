use ratatui::layout::{Position, Rect};
use unicode_width::UnicodeWidthStr;

use crate::tui_types::{pane_label, run_state_label, source_label, TuiModel};

const CAP: usize = 12_000;

pub fn render_non_tty(model: &TuiModel) -> String {
    bounded(&format!(
        "== tui session ==\nmode: non-tty\nstate: {}\npane: {}\npalette: {}\ncomposer: {}\nsearch: {}\nlast-error: {}\n-- transcript --\n{}\n-- hints --\nenter submits | ctrl-p palette | ctrl-c interrupt | ctrl-q quit",
        run_state_label(model.run_state),
        pane_label(model.active_pane),
        model.palette_open,
        if model.composer.is_empty() { "empty" } else { "editing" },
        if model.search.is_empty() { "none" } else { &model.search },
        model.last_error.as_deref().unwrap_or("none"),
        transcript(model),
    ))
}

pub fn transcript(model: &TuiModel) -> String {
    let mut entries = model.transcript.clone();
    if let Some(draft) = &model.agent_draft {
        entries.push(draft.clone());
    }
    if entries.is_empty() {
        return "[no transcript entries]".to_string();
    }
    let keep = (model.height.saturating_sub(10) as usize).max(3);
    entries
        .iter()
        .rev()
        .take(keep)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|entry| format!("{}: {}", source_label(entry.source), entry.text.trim()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn composer_position(area: Rect, model: &TuiModel) -> Position {
    let before = &model.composer[..model.composer_cursor.min(model.composer.len())];
    let line_index = before.lines().count().saturating_sub(1) as u16;
    let col = before.rsplit('\n').next().unwrap_or("").width() as u16;
    let inner_width = area.width.saturating_sub(2);
    let inner_height = area.height.saturating_sub(2);
    Position::new(
        area.x
            .saturating_add(1)
            .saturating_add(col.min(inner_width)),
        area.y
            .saturating_add(1)
            .saturating_add(line_index.min(inner_height)),
    )
}

fn bounded(text: &str) -> String {
    if text.len() <= CAP {
        return text.to_string();
    }
    format!(
        "{}\n[tui truncated]",
        text.chars().take(CAP - 16).collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn japanese_cursor_uses_display_width() {
        let mut model = TuiModel::new();
        model.composer = "あいx".to_string();
        model.composer_cursor = "あい".len();

        let pos = composer_position(Rect::new(10, 5, 20, 4), &model);

        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 6);
    }
}
