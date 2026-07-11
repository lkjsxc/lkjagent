use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui_snapshot::TuiSnapshot;
use crate::tui_types::{pane_label, run_state_label, TuiModel, TuiPane};

pub fn draw(frame: &mut Frame<'_>, model: &TuiModel, snapshot: &TuiSnapshot) {
    let area = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(5),
            Constraint::Length(1),
        ])
        .split(area);
    render_header(frame, vertical[0], model);
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(vertical[1]);
    frame.render_widget(panel("Activity", activity(model, snapshot)), main[0]);
    frame.render_widget(panel("Session", side(model, snapshot)), main[1]);
    render_composer(frame, vertical[2], model);
    frame.set_cursor_position(crate::tui_render::composer_position(vertical[2], model));
    render_footer(frame, vertical[3], model);
    if model.palette_open {
        crate::tui_render::render_palette(frame, area);
    }
}

fn render_header(frame: &mut Frame<'_>, area: Rect, model: &TuiModel) {
    let title = format!(
        " lkjagent workbench  state={} pane={} follow={} search={} ",
        run_state_label(model.run_state),
        pane_label(model.active_pane),
        model.follow,
        if model.search.is_empty() {
            "none"
        } else {
            &model.search
        }
    );
    frame.render_widget(
        Paragraph::new(title).block(Block::default().borders(Borders::ALL)),
        area,
    );
}

fn render_composer(frame: &mut Frame<'_>, area: Rect, model: &TuiModel) {
    let text = if model.composer.is_empty() {
        "type owner message; Ctrl+J newline; Enter submit".to_string()
    } else {
        model.composer.clone()
    };
    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Composer")),
        area,
    );
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, model: &TuiModel) {
    let error = model.last_error.as_deref().unwrap_or("ok");
    frame.render_widget(
        Paragraph::new(format!(
            "Ctrl+Q quit  Ctrl+P palette  Ctrl+S save  Ctrl+C interrupt  F1-F6 panes  error={error}"
        )),
        area,
    );
}

fn panel<'a>(title: &'a str, lines: Vec<Line<'a>>) -> Paragraph<'a> {
    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title(title))
}

fn activity<'a>(model: &TuiModel, snapshot: &'a TuiSnapshot) -> Vec<Line<'a>> {
    let text = match model.active_pane {
        TuiPane::Transcript => transcript(model, snapshot),
        TuiPane::Tasks => snapshot.tasks.clone(),
        TuiPane::Tools => snapshot.tools.clone(),
        TuiPane::StateGraph => format!("{}\n\n{}", snapshot.status, snapshot.proof),
        TuiPane::Workspace => snapshot.workspace.clone(),
        TuiPane::Artifacts => snapshot.proof.clone(),
        TuiPane::Help => help(),
    };
    into_lines(window(&filter(&text, &model.search), model))
}

fn side<'a>(model: &TuiModel, snapshot: &'a TuiSnapshot) -> Vec<Line<'a>> {
    let mut text = [
        "== status ==".to_string(),
        first_lines(&snapshot.status, 12),
        "== queue ==".to_string(),
        snapshot.queue.clone(),
        "== proof ==".to_string(),
        snapshot.proof.clone(),
        "== logs ==".to_string(),
        first_lines(&snapshot.logs, 8),
    ]
    .join("\n");
    if let Some(card) = &model.pending_tool {
        text.push_str(&format!(
            "\n== pending tool ==\n{} {}",
            card.name, card.decision_id
        ));
    }
    into_lines(text)
}

fn transcript(model: &TuiModel, snapshot: &TuiSnapshot) -> String {
    let lines = crate::tui_transcript::display_lines(model, snapshot);
    if lines.is_empty() {
        "system: no transcript entries yet".to_string()
    } else {
        lines.join("\n")
    }
}

fn help() -> String {
    [
        "Keys",
        "Ctrl+Q quit | Ctrl+C interrupt | Ctrl+P palette | Ctrl+S save transcript",
        "Ctrl+L search using composer | Ctrl+J newline | Enter submit",
        "F1 transcript | F2 matters | F3 tools | F4 graph | F5 workspace | F6 help",
        "Up/Down scroll | PageUp/PageDown page | Home/End move composer",
    ]
    .join("\n")
}

fn filter(text: &str, query: &str) -> String {
    if query.is_empty() {
        return text.to_string();
    }
    let needle = query.to_ascii_lowercase();
    text.lines()
        .filter(|line| line.to_ascii_lowercase().contains(&needle))
        .collect::<Vec<_>>()
        .join("\n")
}

fn window(text: &str, model: &TuiModel) -> String {
    crate::tui_types::visible_text(
        text,
        model.height.saturating_sub(10) as usize,
        model.follow,
        model.scroll,
    )
}

fn first_lines(text: &str, count: usize) -> String {
    text.lines().take(count).collect::<Vec<_>>().join("\n")
}

fn into_lines(text: String) -> Vec<Line<'static>> {
    text.lines()
        .map(|line| Line::from(line.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_matches_key_bindings() {
        let text = help();
        assert!(text.contains("Home/End move composer"));
        assert!(!text.contains("Home top"));
    }

    #[test]
    fn transcript_uses_durable_snapshot_agent_messages() {
        let mut snapshot = TuiSnapshot::empty();
        snapshot
            .transcript_entries
            .push(crate::tui_types::TranscriptEntry {
                id: "event:1".to_string(),
                source: crate::tui_types::TranscriptSource::Agent,
                text: "durable answer".to_string(),
                path: Some("sqlite:events:1".to_string()),
            });

        let text = transcript(&TuiModel::new(), &snapshot);

        assert!(text.contains("agent: durable answer"));
    }
}
