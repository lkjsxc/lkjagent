mod ledger;
mod sections;
mod text;

use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_context::budget::ContextBudgetPolicy;
use rusqlite::Connection;

use crate::error::{RuntimeError, RuntimeResult};

pub const CURRENT_GPT_LOG: &str = "logs/current-gpt-5.5-pro.md";

pub fn current_log_path(data_dir: &Path) -> PathBuf {
    data_dir.join(CURRENT_GPT_LOG)
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
