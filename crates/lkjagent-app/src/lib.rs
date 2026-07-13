pub mod args;
pub mod automatic_checks;
pub mod cli;
pub mod clock;
pub mod config;
mod config_registry;
mod model_io;
pub mod public_loop;
pub mod tui_composer;
pub mod tui_model;
pub mod tui_reducer;
pub mod tui_screen;
pub mod tui_viewport;
pub mod tui_wrap;
pub mod workspace_root;

pub mod endpoint {
    pub use crate::model_io::{CompletionRecord, Endpoint, LlmEndpoint, ScriptedEndpoint};
}
