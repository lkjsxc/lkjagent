use std::fs;
use std::path::{Path, PathBuf};

use crate::error::EffectResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangePaths {
    pub dir: PathBuf,
    pub request: PathBuf,
    pub response: PathBuf,
    pub outcome: PathBuf,
    pub timing: PathBuf,
}

pub fn write_exchange(
    root: &Path,
    task_id: u64,
    step_ordinal: u32,
    attempt_ordinal: u32,
    files: ExchangeFiles<'_>,
) -> EffectResult<ExchangePaths> {
    let dir = root
        .join(format!("task-{task_id}"))
        .join(format!("step-{step_ordinal}"))
        .join(format!("attempt-{attempt_ordinal}"));
    fs::create_dir_all(&dir)?;
    let paths = ExchangePaths {
        request: dir.join("request.json"),
        response: dir.join("response.json"),
        outcome: dir.join("outcome.json"),
        timing: dir.join("timing.json"),
        dir,
    };
    fs::write(&paths.request, files.request)?;
    fs::write(&paths.response, files.response)?;
    fs::write(&paths.outcome, files.outcome)?;
    fs::write(&paths.timing, files.timing)?;
    Ok(paths)
}

#[derive(Debug, Clone, Copy)]
pub struct ExchangeFiles<'a> {
    pub request: &'a str,
    pub response: &'a str,
    pub outcome: &'a str,
    pub timing: &'a str,
}
