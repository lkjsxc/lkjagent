mod exchange;
mod index;
mod ledger;
mod sections;
mod text;
mod turn_files;

use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_context::budget::ContextBudgetPolicy;
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};

pub use exchange::{
    json_escape, record_provider_error, record_provider_request, record_provider_response,
    ProviderLogContext, ProviderLogHandle,
};
pub use index::record_provider_index;
pub use turn_files::{
    record_parsed_action, record_provider_admission, record_provider_observation,
};

pub const CURRENT_MODEL_LOG: &str = "logs/current-model-run.md";

pub fn current_log_path(data_dir: &Path) -> PathBuf {
    data_dir.join(CURRENT_MODEL_LOG)
}

pub fn write_current_log(
    conn: &Connection,
    path: &Path,
    now: &str,
    budget: ContextBudgetPolicy,
) -> RuntimeResult<PathBuf> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    fs::write(path, render_current_log(conn, now, budget)?).map_err(io_error)?;
    Ok(path.to_path_buf())
}

pub fn render_current_log(
    conn: &Connection,
    now: &str,
    budget: ContextBudgetPolicy,
) -> RuntimeResult<String> {
    sections::render(conn, now, budget)
}

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Store(error.to_string())
}
