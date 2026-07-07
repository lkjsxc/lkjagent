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
    if model.transcript.is_empty() {
        return "[no transcript entries]".to_string();
    }
    let keep = (model.height.saturating_sub(10) as usize).max(3);
    model
        .transcript
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

fn bounded(text: &str) -> String {
    if text.len() <= CAP {
        return text.to_string();
    }
    format!(
        "{}\n[tui truncated]",
        text.chars().take(CAP - 16).collect::<String>()
    )
}
