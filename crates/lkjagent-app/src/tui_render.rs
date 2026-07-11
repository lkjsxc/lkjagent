use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use unicode_segmentation::UnicodeSegmentation;
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
    let before = model
        .composer
        .graphemes(true)
        .take(model.composer_cursor)
        .collect::<String>();
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

pub fn render_palette(frame: &mut Frame<'_>, area: Rect) {
    let area = centered(area, 60, 50);
    frame.render_widget(Clear, area);
    let title = Span::styled(
        "Command Palette",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    let lines = vec![
        Line::from(title),
        Line::from("F1-F6 switch panes"),
        Line::from("Ctrl+L apply composer text as search"),
        Line::from("Ctrl+S save transcript"),
        Line::from("Esc close palette"),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Palette")),
        area,
    );
}

fn centered(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup[1])[1]
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
        model.composer_cursor = 2;

        let pos = composer_position(Rect::new(10, 5, 20, 4), &model);

        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 6);
    }
}
