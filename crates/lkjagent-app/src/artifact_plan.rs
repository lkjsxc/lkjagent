use std::collections::BTreeSet;
use std::path::Path;

use lkjagent_core::runtime_artifact::{artifact_fingerprint, ArtifactUnit};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::EffectTargetRevision;
use lkjagent_store::artifact_rows::{artifacts, ArtifactRow};
use rusqlite::Connection;

const SIZE_JUSTIFICATION: &str =
    "generated artifact body is stored in checked parts to keep this file readable";

pub struct WritePlan {
    pub target_path: Option<String>,
    pub prior_fingerprint: String,
    pub intended_fingerprint: String,
    pub targets: Vec<EffectTargetRevision>,
}

#[rustfmt::skip]
pub fn plan_write(conn: &Connection, workspace: &Path, case_id: &str, raw_path: &str,
    content: &str, append: bool, now: &str) -> Result<WritePlan, String> {
    let path = crate::artifact_effects::normalize_path(raw_path)?;
    let prior = crate::artifact_effects::read_optional(workspace, &path)?;
    let owned = owned_parts(conn, workspace, case_id, &path)?;
    let source = prior_source(workspace, prior.as_deref(), &owned, content, append)?;
    let (body, units) = crate::artifact_effects::assemble_content(&path, &source)?;
    let file_id = stable_artifact_id(case_id, &path, &body)?;
    let parent = artifact_row(case_id, &file_id, &path, &body, None, &file_metadata(&units), now)?;
    let split = units.len() > 1;
    let intended = if split { units.iter().map(|unit| crate::artifact_effects::part_path(&path, unit.ordinal)).collect() }
        else { BTreeSet::new() };
    let current = crate::artifact_effects::managed_parts(workspace, &path)?;
    if let Some(path) = current.iter().find(|path| !owned.contains(*path)) {
        return Err(format!("artifact part path is not managed: {path}"));
    }
    let mut targets = vec![membership_target(&path, &current, &intended)?];
    if split { for unit in &units {
        let part = crate::artifact_effects::part_path(&path, unit.ordinal);
        let row = artifact_row(case_id, &format!("{file_id}-unit-{:04}", unit.ordinal), &part,
            &unit.content, Some(file_id.clone()), &unit_metadata(&path, unit), now)?;
        targets.push(target(workspace, &part, "part", Some(unit.content.as_bytes().to_vec()), vec![row])?);
    }}
    for stale in owned.difference(&intended) {
        targets.push(target(workspace, stale, "stale-part", None, Vec::new())?);
    }
    let mut main_artifacts = vec![parent];
    if !split { for unit in &units {
        main_artifacts.push(artifact_row(case_id, &format!("{file_id}-unit-{:04}", unit.ordinal), &path,
            &unit.content, Some(file_id.clone()), &unit_metadata(&path, unit), now)?);
    }}
    targets.push(target(workspace, &path, "main", Some(body.as_bytes().to_vec()), main_artifacts)?);
    for (index, target) in targets.iter_mut().enumerate() {
        target.target_ordinal = i64::try_from(index + 1).map_err(|error| error.to_string())?;
    }
    let main = targets.last().ok_or_else(|| "artifact plan has no main target".to_string())?;
    Ok(WritePlan { target_path: Some(path), prior_fingerprint: main.prior_fingerprint.clone(),
        intended_fingerprint: stable_fingerprint(&body).map_err(|error| error.message)?, targets })
}

fn prior_source(
    workspace: &Path,
    prior: Option<&[u8]>,
    owned: &BTreeSet<String>,
    content: &str,
    append: bool,
) -> Result<String, String> {
    if !append {
        return Ok(content.to_string());
    }
    let mut body = if owned.is_empty() {
        String::from_utf8(prior.unwrap_or_default().to_vec()).map_err(|error| error.to_string())?
    } else {
        let mut parts = Vec::new();
        for part in owned {
            let bytes = crate::artifact_effects::read_optional(workspace, part)?
                .ok_or_else(|| format!("managed artifact part is missing: {part}"))?;
            parts.push(String::from_utf8(bytes).map_err(|error| error.to_string())?);
        }
        parts.concat()
    };
    body.push_str(content);
    Ok(body)
}

fn owned_parts(
    conn: &Connection,
    workspace: &Path,
    case_id: &str,
    path: &str,
) -> Result<BTreeSet<String>, String> {
    let rows = artifacts(conn, case_id).map_err(|error| error.to_string())?;
    let parent = rows
        .iter()
        .filter(|row| row.path == path && row.parent_artifact_id.is_none())
        .max_by(|left, right| (&left.created_at, &left.id).cmp(&(&right.created_at, &right.id)));
    let Some(parent) = parent else {
        return Ok(BTreeSet::new());
    };
    let dir = format!("{}/", crate::artifact_effects::part_dir(path));
    let mut owned = BTreeSet::new();
    for row in rows
        .iter()
        .filter(|row| row.parent_artifact_id.as_deref() == Some(&parent.id))
        .filter(|row| row.path.starts_with(&dir))
    {
        let path = crate::artifact_effects::normalize_path(&row.path)?;
        let bytes = crate::artifact_effects::read_optional(workspace, &path)?
            .ok_or_else(|| format!("managed artifact part is missing: {path}"))?;
        let content = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let fingerprint = artifact_fingerprint(&path, &content).map_err(|error| error.message)?;
        if fingerprint != row.fingerprint {
            return Err(format!("managed artifact part drifted: {path}"));
        }
        owned.insert(path);
    }
    Ok(owned)
}

fn target(
    workspace: &Path,
    path: &str,
    role: &str,
    intended_bytes: Option<Vec<u8>>,
    artifacts: Vec<ArtifactRow>,
) -> Result<EffectTargetRevision, String> {
    let prior_bytes = crate::artifact_effects::read_optional(workspace, path)?;
    revision(path, role, prior_bytes, intended_bytes, artifacts)
}

fn membership_target(
    main: &str,
    prior: &BTreeSet<String>,
    intended: &BTreeSet<String>,
) -> Result<EffectTargetRevision, String> {
    revision(
        &crate::artifact_effects::part_dir(main),
        "parts-membership",
        Some(crate::artifact_effects::list_bytes(prior)),
        Some(crate::artifact_effects::list_bytes(intended)),
        Vec::new(),
    )
}

fn revision(
    path: &str,
    role: &str,
    prior_bytes: Option<Vec<u8>>,
    intended_bytes: Option<Vec<u8>>,
    artifacts: Vec<ArtifactRow>,
) -> Result<EffectTargetRevision, String> {
    Ok(EffectTargetRevision {
        target_ordinal: 0,
        role: role.to_string(),
        path: path.to_string(),
        prior_fingerprint: stable_fingerprint(&prior_bytes).map_err(|error| error.message)?,
        intended_fingerprint: stable_fingerprint(&intended_bytes).map_err(|error| error.message)?,
        prior_bytes,
        intended_bytes,
        artifacts,
    })
}

#[rustfmt::skip]
fn artifact_row(case_id: &str, id: &str, path: &str, content: &str, parent_artifact_id: Option<String>, metadata_json: &str, now: &str) -> Result<ArtifactRow, String> {
    Ok(ArtifactRow { id: id.to_string(), case_id: case_id.to_string(),
        kind: if parent_artifact_id.is_some() { "unit" } else { "file" }.to_string(),
        path: path.to_string(), fingerprint: artifact_fingerprint(path, content).map_err(|error| error.message)?,
        parent_artifact_id, metadata_json: metadata_json.to_string(), created_at: now.to_string() })
}

#[rustfmt::skip]
fn stable_artifact_id(case_id: &str, path: &str, body: &str) -> Result<String, String> {
    let task = case_id.parse::<u64>().map_or_else(|_| serde_json::json!(case_id), |value| serde_json::json!(value));
    stable_fingerprint(&serde_json::json!({ "task": task, "path": path, "content": body }))
        .map(|fingerprint| format!("task-{case_id}-artifact-{fingerprint}")).map_err(|error| error.message)
}

#[rustfmt::skip]
fn file_metadata(units: &[ArtifactUnit]) -> String {
    if units.len() <= 1 { "{}".to_string() }
    else { serde_json::json!({ "part_count": units.len(), "size_justification": SIZE_JUSTIFICATION }).to_string() }
}

fn unit_metadata(path: &str, unit: &ArtifactUnit) -> String {
    serde_json::json!({ "target_tokens": unit.target_tokens, "target_words": unit.target_words,
        "ordinal": unit.ordinal, "assembled_path": path })
    .to_string()
}
