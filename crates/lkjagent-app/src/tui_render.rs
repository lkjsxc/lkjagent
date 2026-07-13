use unicode_segmentation::UnicodeSegmentation;

use crate::tui_model::TuiModel;
use crate::tui_viewport;
use crate::tui_wrap::{display_width, wrap};

pub fn render(model: &TuiModel) -> String {
    lines(model).join("\r\n")
}

pub fn lines(model: &TuiModel) -> Vec<String> {
    let width = model.width;
    let height = model.height;
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let mut output = vec![clip_display(&status(model), width)];
    let search_rows = usize::from(model.search_active);
    let activity_rows = if model.screen.activity.expanded {
        height.saturating_sub(2 + search_rows).min(5)
    } else {
        0
    };
    let conversation_height = model.conversation_height();
    let rows = model
        .screen
        .rows(width.saturating_sub(7).max(1), &model.search);
    for row in tui_viewport::visible(&model.viewport, &rows, conversation_height) {
        output.push(clip_display(
            &format!("{}: {}", role(&row.role), row.text),
            width,
        ));
    }
    while output.len() < 1 + conversation_height {
        output.push(String::new());
    }
    if activity_rows > 0 {
        output.push(clip_display("activity (F2 collapse)", width));
        let room = activity_rows.saturating_sub(1);
        let skip = model.screen.activity.items.len().saturating_sub(room);
        for item in model.screen.activity.items.iter().skip(skip) {
            output.push(clip_display(
                &format!("  {} {}", item.kind, item.status),
                width,
            ));
        }
        while output.len() < 1 + conversation_height + activity_rows {
            output.push(String::new());
        }
    }
    if model.search_active {
        output.push(clip_display(&format!("find: {}", model.search), width));
    }
    output.push(clip_display(&composer(model, width), width));
    output.truncate(height);
    output
}

pub fn clip_display(text: &str, width: usize) -> String {
    let mut output = String::new();
    let mut used: usize = 0;
    for grapheme in text.graphemes(true) {
        let columns = display_width(grapheme);
        if used.saturating_add(columns) > width {
            break;
        }
        output.push_str(grapheme);
        used = used.saturating_add(columns);
    }
    output
}

fn status(model: &TuiModel) -> String {
    let value = &model.screen.status;
    format!(
        "matters o:{} b:{} c:{} work d:{} x:{} e:{} bad a:{} o:{} checks {}/{} cells:{}",
        value.open_matters,
        value.blocked_matters,
        value.closed_matters,
        value.unfinished_decisions,
        value.unfinished_exchanges,
        value.unfinished_effects,
        value.rejected_admissions,
        value.failed_observations,
        value.passing_checks,
        value.current_checks,
        value.active_cells,
    )
}

fn composer(model: &TuiModel, width: usize) -> String {
    if let Some(error) = &model.composer.last_error {
        return format!("! {}", bounded(error));
    }
    let available = width.saturating_sub(2).max(1);
    let byte = model
        .composer
        .text
        .grapheme_indices(true)
        .nth(model.composer.cursor)
        .map_or(model.composer.text.len(), |(index, _)| index);
    let mut marked = model.composer.text.clone();
    marked.insert(byte, '│');
    let rows = wrap(&marked, available);
    let current = rows
        .iter()
        .find(|row| row.contains('│'))
        .map_or("│", String::as_str);
    format!("> {current}")
}

fn bounded(value: &str) -> String {
    clip_display(value, 160)
}

fn role(value: &str) -> &str {
    match value {
        "owner" => "owner",
        "final" | "agent" => "agent",
        _ => "message",
    }
}
