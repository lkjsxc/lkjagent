use super::{display, style};

pub fn top_lines(
    events: &[lkjagent_store::events::EventRow],
    pending: &[&lkjagent_store::queue::QueueRow],
    width: usize,
) -> Vec<String> {
    let mut lines = Vec::new();
    if events.is_empty() {
        lines.push("none".to_string());
    } else {
        lines.extend(events.iter().rev().take(32).rev().map(|event| {
            let prefix = format!(
                "#{:<4} {:<13} t={:<4} ",
                event.id,
                event.kind,
                event
                    .turn
                    .map_or_else(|| "-".to_string(), |turn| turn.to_string()),
            );
            let preview_width = width.saturating_sub(display::visible_width(&prefix));
            format!(
                "{prefix}{}",
                display::preview(&event.content, preview_width)
            )
        }));
    }
    if !pending.is_empty() {
        lines.push(style::muted("pending queue"));
        for row in pending.iter().take(3) {
            lines.push(format!(
                "#{:<4} {}",
                row.id,
                display::preview(&row.content, width.saturating_sub(8))
            ));
        }
    }
    lines
}

pub fn last_output(events: &[lkjagent_store::events::EventRow]) -> Option<String> {
    events
        .iter()
        .rev()
        .filter(|event| event.kind == "observation")
        .filter_map(|event| observation_content(&event.content))
        .find(|text| !text.starts_with("waiting"))
}

fn observation_content(content: &str) -> Option<String> {
    let text = between(content, "<content>", "</content>")?.trim();
    let useful = text
        .lines()
        .filter(str_has_signal)
        .collect::<Vec<_>>()
        .join(" ");
    (!useful.is_empty()).then_some(useful)
}

fn str_has_signal(line: &&str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && !trimmed.starts_with("exit_code=")
}

fn between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let after = text.split_once(start)?.1;
    Some(after.split_once(end)?.0)
}
