use std::fs;
use std::path::{Component, Path, PathBuf};

pub const DEFAULT_WORKSPACE_ROOT: &str = "../workspace";

pub fn resolve(data_root: &Path, configured: &str) -> Result<PathBuf, String> {
    validate_text(configured)?;
    let configured = Path::new(configured);
    let root = if configured.is_absolute() {
        clean(configured)
    } else {
        clean(&data_root.join(configured))
    };
    reject_internal_root(data_root, &root)?;
    Ok(root)
}

pub fn open(root: &Path) -> Result<PathBuf, String> {
    if !root.exists() {
        fs::create_dir(root).map_err(|error| format!("create workspace root: {error}"))?;
    }
    let metadata =
        fs::symlink_metadata(root).map_err(|error| format!("open workspace root: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("open workspace root: root must be a real directory".to_string());
    }
    root.canonicalize()
        .map_err(|error| format!("open workspace root: {error}"))
}

fn validate_text(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err("workspace root must be nonempty text without control characters".to_string());
    }
    Ok(())
}

fn reject_internal_root(data_root: &Path, root: &Path) -> Result<(), String> {
    let data = absolute_clean(data_root)?;
    let root = absolute_clean(root)?;
    if root == data || root.starts_with(&data) {
        return Err(
            "workspace root must be separate from runtime data, database, logs, and temp"
                .to_string(),
        );
    }
    Ok(())
}

fn absolute_clean(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(clean(path))
    } else {
        std::env::current_dir()
            .map(|cwd| clean(&cwd.join(path)))
            .map_err(|error| error.to_string())
    }
}

fn clean(path: &Path) -> PathBuf {
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                output.pop();
            }
            other => output.push(other.as_os_str()),
        }
    }
    output
}
