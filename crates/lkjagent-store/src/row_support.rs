use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::admission_rows::EffectTargetRevision;
use crate::error::{StoreError, StoreResult};

pub(crate) fn json_string<T>(value: &T) -> StoreResult<String>
where
    T: serde::Serialize,
{
    serde_json::to_string(value).map_err(json_error)
}

pub(crate) fn json_value<T>(text: &str) -> StoreResult<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(text).map_err(json_error)
}

pub(crate) fn json_error(error: serde_json::Error) -> StoreError {
    StoreError::InvalidState(error.to_string())
}

pub(crate) fn fingerprint_error(
    error: lkjagent_core::runtime_fingerprint::FingerprintError,
) -> StoreError {
    StoreError::InvalidState(error.message)
}

pub fn target_fingerprint(workspace: &Path, target: &EffectTargetRevision) -> StoreResult<String> {
    let bytes = if target.role == "parts-membership" {
        Some(membership_bytes(workspace, &target.path)?)
    } else {
        read_optional(workspace, &target.path)?
    };
    lkjagent_core::runtime_fingerprint::stable_fingerprint(&bytes)
        .map_err(|error| StoreError::InvalidState(error.message))
}

fn read_optional(workspace: &Path, target: &str) -> StoreResult<Option<Vec<u8>>> {
    let path = guarded_path(workspace, target)?;
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(StoreError::InvalidState(
            "effect target must not be a symlink".to_string(),
        )),
        Ok(_) => fs::read(path)
            .map(Some)
            .map_err(|error| StoreError::Sql(error.to_string())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(StoreError::Sql(error.to_string())),
    }
}

fn membership_bytes(workspace: &Path, dir: &str) -> StoreResult<Vec<u8>> {
    let path = guarded_path(workspace, dir)?;
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(StoreError::Sql(error.to_string())),
        Ok(metadata) if !metadata.is_dir() => {
            return Err(StoreError::InvalidState(
                "effect part membership target is not a directory".to_string(),
            ))
        }
        Ok(_) => {}
    }
    let mut paths = BTreeSet::new();
    for entry in fs::read_dir(path).map_err(sql_error)? {
        let name = entry
            .map_err(sql_error)?
            .file_name()
            .to_string_lossy()
            .to_string();
        if managed_name(&name) {
            paths.insert(format!("{dir}/{name}"));
        }
    }
    Ok(paths
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes())
}

fn managed_name(name: &str) -> bool {
    name.strip_prefix("part-")
        .and_then(|value| value.strip_suffix(".md"))
        .is_some_and(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
}

fn guarded_path(workspace: &Path, target: &str) -> StoreResult<PathBuf> {
    let relative = Path::new(target);
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(StoreError::InvalidState(
            "effect target escapes workspace".to_string(),
        ));
    }
    let root = workspace.canonicalize().map_err(sql_error)?;
    let path = root.join(relative);
    let mut parent = path.clone();
    while fs::symlink_metadata(&parent).is_err() {
        parent = parent
            .parent()
            .ok_or_else(|| {
                StoreError::InvalidState("effect target has no existing parent".to_string())
            })?
            .to_path_buf();
    }
    let resolved = parent.canonicalize().map_err(sql_error)?;
    if !resolved.starts_with(&root) {
        return Err(StoreError::InvalidState(
            "effect target resolves outside workspace".to_string(),
        ));
    }
    Ok(path)
}

fn sql_error(error: std::io::Error) -> StoreError {
    StoreError::Sql(error.to_string())
}
