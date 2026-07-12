pub mod args;
pub mod automatic_checks;
pub mod cli;
pub mod clock;
pub mod config;
mod config_registry;
mod model_io;
pub mod public_loop;
pub mod workspace_root;

pub mod endpoint {
    pub use crate::model_io::{CompletionRecord, Endpoint, LlmEndpoint, ScriptedEndpoint};
}
