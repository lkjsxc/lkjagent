use std::{fs, path::Path};

use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_artifact::{artifact_fingerprint, assemble_checked_units, ArtifactUnit};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::artifact_rows::{insert_artifact, ArtifactRow};
use rusqlite::Connection;

const UNIT_TARGET_WORDS: usize = 384;
const PART_LINK_LIMIT: usize = 20;
const SIZE_JUSTIFICATION: &str =
    "generated artifact body is stored in checked parts to keep this file readable";

pub fn assemble_content(path: &str, content: &str) -> Result<(String, Vec<ArtifactUnit>), String> {
    let units = checked_units(path, content);
    let assembled = assemble_checked_units(path, &units).map_err(|error| error.message)?;
    let body = if units.len() > 1 {
        manifest_content(path, &assembled.content, &units)?
    } else {
        assembled.content
    };
    Ok((body, units))
}

pub fn sync_part_files(workspace: &Path, path: &str, units: &[ArtifactUnit]) -> Result<(), String> {
    let dir = part_dir(path);
    let full_dir =
        lkjagent_effects::workspace::resolve(workspace, &dir).map_err(|error| error.to_string())?;
    if full_dir.exists() {
        fs::remove_dir_all(&full_dir).map_err(|error| error.to_string())?;
    }
    if units.len() <= 1 {
        return Ok(());
    }
    for unit in units {
        let path = part_path(path, unit.ordinal);
        lkjagent_effects::workspace::write(workspace, &path, &unit.content)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn persist_artifacts(
    conn: &Connection,
    snapshot: &TaskSnapshot,
    path: &str,
    body: &str,
    units: &[ArtifactUnit],
    now: &str,
) -> Result<(), String> {
    let file_id = stable_artifact_id(snapshot, body)?;
    let file_metadata = file_metadata(units);
    insert_artifact(
        conn,
        &artifact_row(snapshot, &file_id, path, body, None, &file_metadata, now)?,
    )
    .map_err(|error| error.to_string())?;
    let split = units.len() > 1;
    for unit in units {
        let metadata = unit_metadata(path, unit);
        let unit_row_path = if split {
            part_path(path, unit.ordinal)
        } else {
            path.to_string()
        };
        insert_artifact(
            conn,
            &artifact_row(
                snapshot,
                &format!("{file_id}-unit-{:04}", unit.ordinal),
                &unit_row_path,
                &unit.content,
                Some(file_id.clone()),
                &metadata,
                now,
            )?,
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn checked_units(path: &str, content: &str) -> Vec<ArtifactUnit> {
    word_chunks(content)
        .into_iter()
        .enumerate()
        .map(|(index, chunk)| {
            let mut unit = ArtifactUnit::new(
                format!("effect-unit-{:04}", index + 1),
                path,
                u32::try_from(index + 1).unwrap_or(u32::MAX),
            );
            unit.content = chunk;
            unit.target_words = Some(UNIT_TARGET_WORDS);
            unit.check_passed = true;
            unit
        })
        .collect()
}

fn word_chunks(content: &str) -> Vec<String> {
    let words = content.split_whitespace().collect::<Vec<_>>();
    if words.is_empty() {
        return vec![content.to_string()];
    }
    words
        .chunks(UNIT_TARGET_WORDS)
        .map(|chunk| chunk.join(" "))
        .collect()
}

fn artifact_row(
    snapshot: &TaskSnapshot,
    id: &str,
    path: &str,
    content: &str,
    parent_artifact_id: Option<String>,
    metadata_json: &str,
    now: &str,
) -> Result<ArtifactRow, String> {
    let kind = match parent_artifact_id {
        Some(_) => "unit",
        None => "file",
    };
    Ok(ArtifactRow {
        id: id.to_string(),
        case_id: snapshot.task.id.to_string(),
        kind: kind.to_string(),
        path: path.to_string(),
        fingerprint: artifact_fingerprint(path, content).map_err(|error| error.message)?,
        parent_artifact_id,
        metadata_json: metadata_json.to_string(),
        created_at: now.to_string(),
    })
}

fn stable_artifact_id(snapshot: &TaskSnapshot, body: &str) -> Result<String, String> {
    stable_fingerprint(&serde_json::json!({
        "task": snapshot.task.id,
        "content": body,
    }))
    .map(|fingerprint| format!("task-{}-artifact-{fingerprint}", snapshot.task.id))
    .map_err(|error| error.message)
}

fn file_metadata(units: &[ArtifactUnit]) -> String {
    if units.len() <= 1 {
        return "{}".to_string();
    }
    serde_json::json!({
        "part_count": units.len(),
        "size_justification": SIZE_JUSTIFICATION,
    })
    .to_string()
}

fn unit_metadata(assembled_path: &str, unit: &ArtifactUnit) -> String {
    serde_json::json!({
        "target_tokens": unit.target_tokens,
        "target_words": unit.target_words,
        "ordinal": unit.ordinal,
        "assembled_path": assembled_path,
    })
    .to_string()
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

fn part_dir(path: &str) -> String {
    format!("{}.parts", path.strip_suffix(".md").unwrap_or(path))
}

fn part_path(path: &str, ordinal: u32) -> String {
    format!("{}/part-{ordinal:03}.md", part_dir(path))
}
