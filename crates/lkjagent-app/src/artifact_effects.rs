use lkjagent_core::model::TaskSnapshot;
use lkjagent_core::runtime_artifact::{
    artifact_fingerprint, assemble_checked_units, ArtifactUnit, DEFAULT_UNIT_TARGET_TOKENS,
};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::artifact_rows::{insert_artifact, ArtifactRow};
use rusqlite::Connection;

const UNIT_TARGET_WORDS: usize = 384;

pub fn assemble_content(path: &str, content: &str) -> Result<(String, Vec<ArtifactUnit>), String> {
    let units = checked_units(path, content);
    let assembled = assemble_checked_units(path, &units).map_err(|error| error.message)?;
    Ok((assembled.content, units))
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
    insert_artifact(
        conn,
        &artifact_row(snapshot, &file_id, path, body, None, "{}", now)?,
    )
    .map_err(|error| error.to_string())?;
    for unit in units {
        let metadata = serde_json::json!({
            "target_tokens": DEFAULT_UNIT_TARGET_TOKENS,
            "ordinal": unit.ordinal,
        })
        .to_string();
        insert_artifact(
            conn,
            &artifact_row(
                snapshot,
                &format!("{file_id}-unit-{:04}", unit.ordinal),
                path,
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
    let kind = if parent_artifact_id.is_some() {
        "unit"
    } else {
        "file"
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
