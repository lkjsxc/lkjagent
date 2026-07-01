use std::fs;
use std::path::{Component, Path};

use crate::error::{BenchError, BenchResult};
use crate::model::{FileSpec, Fixture};

pub fn materialize_starter(files: &[FileSpec], workspace: &Path) -> BenchResult<()> {
    materialize_files(files, workspace)
}

pub fn materialize_fixture(fixture: &Fixture, workspace: &Path) -> BenchResult<()> {
    if workspace.exists() {
        fs::remove_dir_all(workspace)?;
    }
    fs::create_dir_all(workspace)?;
    materialize_files(fixture.files, workspace)
}

fn materialize_files(files: &[FileSpec], workspace: &Path) -> BenchResult<()> {
    for file in files {
        validate_relative_path(file.path)?;
        let path = workspace.join(file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, expand_content(file.content))?;
    }
    Ok(())
}

fn expand_content(content: &str) -> String {
    let Some(rest) = content.strip_prefix("@repeat_word ") else {
        return content.to_string();
    };
    let parts = rest.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        return content.to_string();
    }
    let count = parts[1].parse::<usize>().ok().unwrap_or(1);
    std::iter::repeat_n(parts[0], count)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn validate_relative_path(path: &str) -> BenchResult<()> {
    if path.trim().is_empty() {
        return Err(BenchError::InvalidTask("empty fixture path".to_string()));
    }
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        return Err(BenchError::InvalidTask(format!(
            "fixture path is absolute: {path}"
        )));
    }
    for component in candidate.components() {
        match component {
            Component::Normal(_) => {}
            _ => {
                return Err(BenchError::InvalidTask(format!(
                    "fixture path escapes workspace: {path}"
                )));
            }
        }
    }
    Ok(())
}
