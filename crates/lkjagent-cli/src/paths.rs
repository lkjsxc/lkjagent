use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

pub fn workspace(data_dir: &Path) -> Result<PathBuf, CliError> {
    let path = data_dir.join("workspace");
    fs::create_dir_all(&path)?;
    Ok(path)
}
