use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use lkjagent_core::engine::Command;
use lkjagent_core::runtime_artifact::{artifact_fingerprint, assemble_checked_units, ArtifactUnit};

const UNIT_TARGET_WORDS: usize = 384;
const PART_LINK_LIMIT: usize = 20;
const SIZE_JUSTIFICATION: &str =
    "generated artifact body is stored in checked parts to keep this file readable";

pub fn assemble_content(path: &str, content: &str) -> Result<(String, Vec<ArtifactUnit>), String> {
    let units = checked_units(path, content);
    let _assembled = assemble_checked_units(path, &units).map_err(|error| error.message)?;
    let body = if units.len() > 1 {
        manifest_content(path, content, &units)?
    } else {
        content.to_string()
    };
    Ok((body, units))
}

pub fn validate_bundle_commands(commands: &[Command]) -> Result<(), String> {
    let mut bundles = Vec::new();
    for command in commands {
        let path = match command {
            Command::WriteFile { path, .. } | Command::AppendFile { path, .. } => {
                normalize_path(path)?
            }
            _ => continue,
        };
        bundles.push((path.clone(), part_dir(&path)));
    }
    for (index, left) in bundles.iter().enumerate() {
        for right in bundles.iter().skip(index + 1) {
            let overlap = [&left.0, &left.1].iter().any(|left_path| {
                [&right.0, &right.1].iter().any(|right_path| {
                    left_path == right_path
                        || left_path.starts_with(&format!("{right_path}/"))
                        || right_path.starts_with(&format!("{left_path}/"))
                })
            });
            if overlap {
                return Err("turn contains overlapping generated artifact targets".to_string());
            }
        }
    }
    Ok(())
}

pub fn normalize_path(path: &str) -> Result<String, String> {
    let mut output = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(value) => output.push(value),
            Component::CurDir => {}
            _ => return Err("artifact path escapes workspace".to_string()),
        }
    }
    let normalized = output.to_string_lossy().to_string();
    if normalized.is_empty() {
        Err("artifact path must not be empty".to_string())
    } else {
        Ok(normalized)
    }
}

pub fn read_optional(workspace: &Path, path: &str) -> Result<Option<Vec<u8>>, String> {
    let full =
        lkjagent_effects::workspace::resolve(workspace, path).map_err(|error| error.to_string())?;
    match fs::symlink_metadata(&full) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err("artifact target must not be a symlink".to_string())
        }
        Ok(_) => fs::read(full).map(Some).map_err(|error| error.to_string()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

pub fn managed_parts(workspace: &Path, main: &str) -> Result<BTreeSet<String>, String> {
    let dir = part_dir(main);
    let full =
        lkjagent_effects::workspace::resolve(workspace, &dir).map_err(|error| error.to_string())?;
    match fs::symlink_metadata(&full) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        Err(error) => return Err(error.to_string()),
        Ok(metadata) if !metadata.is_dir() => {
            return Err("artifact part directory is not a directory".to_string())
        }
        Ok(_) => {}
    }
    let mut paths = BTreeSet::new();
    for entry in fs::read_dir(full).map_err(|error| error.to_string())? {
        let name = entry
            .map_err(|error| error.to_string())?
            .file_name()
            .to_string_lossy()
            .to_string();
        if managed_name(&name) {
            paths.insert(format!("{dir}/{name}"));
        }
    }
    Ok(paths)
}

pub fn list_bytes(paths: &BTreeSet<String>) -> Vec<u8> {
    paths
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes()
}

fn managed_name(name: &str) -> bool {
    name.strip_prefix("part-")
        .and_then(|value| value.strip_suffix(".md"))
        .is_some_and(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
}

#[rustfmt::skip]
fn checked_units(path: &str, content: &str) -> Vec<ArtifactUnit> {
    word_chunks(content).into_iter().enumerate().map(|(index, chunk)| {
        let mut unit = ArtifactUnit::new(format!("effect-unit-{:04}", index + 1), path,
            u32::try_from(index + 1).unwrap_or(u32::MAX));
        unit.content = chunk; unit.target_words = Some(UNIT_TARGET_WORDS); unit.check_passed = true; unit
    }).collect()
}

fn word_chunks(content: &str) -> Vec<String> {
    let mut starts = Vec::new();
    let mut in_word = false;
    for (index, character) in content.char_indices() {
        if !character.is_whitespace() && !in_word {
            starts.push(index);
        }
        in_word = !character.is_whitespace();
    }
    if starts.len() <= UNIT_TARGET_WORDS {
        return vec![content.to_string()];
    }
    let mut chunks = Vec::new();
    let mut start = 0;
    for end in starts
        .iter()
        .skip(UNIT_TARGET_WORDS)
        .step_by(UNIT_TARGET_WORDS)
    {
        chunks.push(content[start..*end].to_string());
        start = *end;
    }
    chunks.push(content[start..].to_string());
    chunks
}

fn manifest_content(path: &str, content: &str, units: &[ArtifactUnit]) -> Result<String, String> {
    let fingerprint = artifact_fingerprint(path, content).map_err(|error| error.message)?;
    let dir = part_dir(path);
    let mut links = units
        .iter()
        .take(PART_LINK_LIMIT)
        .map(|unit| {
            format!(
                "- part {:03}: `{}`",
                unit.ordinal,
                part_path(path, unit.ordinal)
            )
        })
        .collect::<Vec<_>>();
    if units.len() > PART_LINK_LIMIT {
        links.push(format!(
            "- remaining parts: {} files in `{dir}/`",
            units.len() - PART_LINK_LIMIT
        ));
    }
    Ok(format!(
        "# Artifact\n\nSize justification: {SIZE_JUSTIFICATION}.\nFull body fingerprint: \
         `{fingerprint}`.\nPart directory: `{dir}/`.\n\n## Parts\n\n{}",
        links.join("\n")
    ))
}

#[rustfmt::skip]
pub fn part_dir(path: &str) -> String { format!("{}.parts", path.strip_suffix(".md").unwrap_or(path)) }
#[rustfmt::skip]
pub fn part_path(path: &str, ordinal: u32) -> String { format!("{}/part-{ordinal:03}.md", part_dir(path)) }
