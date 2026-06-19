use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

pub fn workspace(data_dir: &Path) -> Result<PathBuf, CliError> {
    let path = data_dir.join("workspace");
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn skill_library() -> PathBuf {
    let image = Path::new("/usr/local/share/lkjagent/skills");
    if image.exists() {
        return image.to_path_buf();
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../lkjagent-skills/seeds")
}
