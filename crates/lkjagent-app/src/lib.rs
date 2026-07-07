mod admission_bridge;
mod arg_helpers;
pub mod args;
mod artifact_effects;
pub mod cli;
pub mod clock;
pub mod config;
pub mod console;
mod context_admin;
mod context_bridge;
mod context_resolution_bridge;
pub mod daemon;
mod daemon_intake;
mod daemon_lock;
mod diagnostics;
mod diagnostics_support;
mod effect_error;
pub mod endpoint;
mod exchange_bridge;
mod exchange_record;
mod explore;
mod inspect;
mod lease_status;
mod log_view;
mod model_call;
mod model_io;
mod observation_bridge;
mod prompt_bridge;
mod record_args;
mod record_files;
mod record_state;
mod recovery_bridge;
mod runtime_bridge;
mod runtime_cell;
mod runtime_projection;
mod snapshot_state;
pub mod state;
pub mod status;
mod task_view;
pub mod tui_event {
    pub use crate::tui_state::TuiEvent;
}
pub mod tui_render {
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
        let keep = (model.height.saturating_sub(10) as usize).max(3);
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
}
pub mod tui_state;
mod turn_effects;
mod watch_view;
pub mod workbench;
mod workbench_commands;
mod workbench_render;
pub mod workbench_state;
mod workspace_index;
mod workspace_rebalance;
