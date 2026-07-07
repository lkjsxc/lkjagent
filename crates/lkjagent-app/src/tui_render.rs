use crate::tui_state::{TranscriptSource, TuiModel, TuiPane, TuiRunState};

const CAP: usize = 12_000;

pub fn render_non_tty(model: &TuiModel) -> String {
    bounded(&format!(
        "== tui session ==\nmode: non-tty\nstate: {}\npane: {}\npalette: {}\ncomposer: {}\nlast-error: {}\n-- transcript --\n{}\n-- hints --\nenter submits | ctrl-p palette | ctrl-c interrupt | ctrl-q quit",
        run_state(model.run_state),
        pane(model.active_pane),
        model.palette_open,
        if model.composer.is_empty() { "empty" } else { "editing" },
        model.last_error.as_deref().unwrap_or("none"),
        transcript(model),
    ))
}

fn transcript(model: &TuiModel) -> String {
    if model.transcript.is_empty() {
        return "[no transcript entries]".to_string();
    }
    let height = model.height.saturating_sub(10) as usize;
    let keep = height.max(3);
    model
        .transcript
        .iter()
        .rev()
        .take(keep)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|entry| format!("{}: {}", source(entry.source), entry.text.trim()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn source(source: TranscriptSource) -> &'static str {
    match source {
        TranscriptSource::Owner => "owner",
        TranscriptSource::Agent => "agent",
        TranscriptSource::Tool => "tool",
        TranscriptSource::State => "state",
        TranscriptSource::System => "system",
        TranscriptSource::Error => "error",
    }
}

fn run_state(state: TuiRunState) -> &'static str {
    match state {
        TuiRunState::Idle => "idle",
        TuiRunState::Running => "running",
        TuiRunState::ToolPending => "tool-pending",
        TuiRunState::ToolRunning => "tool-running",
        TuiRunState::Interrupted => "interrupted",
    }
}

fn pane(pane: TuiPane) -> &'static str {
    match pane {
        TuiPane::Transcript => "transcript",
        TuiPane::Tasks => "tasks",
        TuiPane::Tools => "tools",
        TuiPane::StateGraph => "state-graph",
        TuiPane::Workspace => "workspace",
        TuiPane::Artifacts => "artifacts",
        TuiPane::Help => "help",
    }
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
