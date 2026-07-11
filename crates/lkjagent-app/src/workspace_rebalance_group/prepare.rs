use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_core::workspace_record::{parse_record, record_fingerprint};
use lkjagent_store::record_rows::{record, RecordRow};
use lkjagent_store::workspace_rows::{
    operation_revisions, prepare_or_load_operation, OperationDraft, OperationPreparation,
    OperationRevision, OperationRow,
};
use rusqlite::Connection;

const KIND: &str = "rebalance-group";

pub(super) struct Loaded {
    pub moves: Vec<RebalanceMove>,
    pub originals: Vec<RecordRow>,
    pub revisions: Vec<OperationRevision>,
}

pub(super) fn prepare(
    conn: &Connection,
    data_dir: &Path,
    mut moves: Vec<RebalanceMove>,
    now: &str,
) -> Result<OperationRow, String> {
    super::validate_moves(&moves)?;
    let workspace = crate::config::workspace_root(data_dir)?;
    let old_paths = moves
        .iter()
        .map(|item| item.old_path.clone())
        .collect::<Vec<_>>();
    let mut originals = Vec::new();
    let mut revisions = Vec::new();
    for (ordinal, item) in moves.iter_mut().enumerate() {
        let row = record(conn, &item.entity_id)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| format!("record not found: {}", item.entity_id))?;
        if row.path != item.old_path {
            return Err("rebalance group record path changed".to_string());
        }
        if crate::effect_files::path_occupied(&workspace, &item.new_path)? {
            return Err("rebalance group target is occupied".to_string());
        }
        let bytes = crate::workspace_rebalance::verified_file_bytes(
            &workspace,
            &item.old_path,
            &row.fingerprint,
        )?;
        let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
        let parsed = parse_record(text)?;
        if parsed.links.iter().any(|link| old_paths.contains(link)) {
            return Err("rebalance group members contain inter-group links".to_string());
        }
        item.validation
            .push(format!("fingerprint-before:{}", row.fingerprint));
        let fingerprint = stable_fingerprint(&bytes).map_err(|error| error.message)?;
        revisions.push(revision(
            ordinal,
            "prior",
            &item.old_path,
            &bytes,
            &fingerprint,
        ));
        revisions.push(revision(
            ordinal,
            "intended",
            &item.new_path,
            &bytes,
            &fingerprint,
        ));
        originals.push(row);
    }
    let preimage = serde_json::json!({"records": &originals}).to_string();
    let intended = serde_json::json!({"moves": &moves}).to_string();
    let fingerprint = super::group_fingerprint(&preimage, &intended, &revisions)?;
    let id = format!("workspace-rebalance-group-{fingerprint}");
    let key = format!("rebalance-group:{fingerprint}");
    let draft = OperationDraft {
        id: &id,
        key: &key,
        kind: KIND,
        preimage: &preimage,
        intended: &intended,
        revisions: &revisions,
        now,
    };
    match prepare_or_load_operation(conn, &draft).map_err(|error| error.to_string())? {
        OperationPreparation::Prepared(row) | OperationPreparation::Existing(row) => Ok(row),
    }
}

pub(super) fn load(conn: &Connection, operation: &OperationRow) -> Result<Loaded, String> {
    if operation.kind != KIND {
        return Err("invalid rebalance group kind".to_string());
    }
    let moves = intended_moves(operation)?;
    let value: serde_json::Value =
        serde_json::from_str(&operation.preimage_json).map_err(|error| error.to_string())?;
    let originals: Vec<RecordRow> = serde_json::from_value(
        value
            .get("records")
            .cloned()
            .ok_or_else(|| "rebalance group records missing".to_string())?,
    )
    .map_err(|error| error.to_string())?;
    super::validate_moves(&moves)?;
    if moves.len() != originals.len() {
        return Err("rebalance group membership mismatch".to_string());
    }
    let revisions = operation_revisions(conn, &operation.id).map_err(|error| error.to_string())?;
    if revisions.len() != moves.len() * 2 {
        return Err("rebalance group revision count mismatch".to_string());
    }
    let fingerprint = super::group_fingerprint(
        &operation.preimage_json,
        &operation.intended_json,
        &revisions,
    )?;
    if operation.id != format!("workspace-rebalance-group-{fingerprint}")
        || operation.idempotency_key != format!("rebalance-group:{fingerprint}")
    {
        return Err("rebalance group identity mismatch".to_string());
    }
    for (ordinal, (item, original)) in moves.iter().zip(&originals).enumerate() {
        validate_member(conn, item, original, ordinal, &revisions)?;
    }
    Ok(Loaded {
        moves,
        originals,
        revisions,
    })
}

#[rustfmt::skip]
fn validate_member(conn: &Connection, item: &RebalanceMove, original: &RecordRow,
    ordinal: usize, revisions: &[OperationRevision]) -> Result<(), String> {
    if item.entity_id != original.id || item.old_path != original.path { return Err("rebalance group preimage mismatch".to_string()); }
    let current = record(conn, &item.entity_id).map_err(|error| error.to_string())?
        .ok_or_else(|| format!("record not found: {}", item.entity_id))?;
    let same = current.id == original.id && current.kind == original.kind
        && current.title == original.title && current.state == original.state
        && current.fingerprint == original.fingerprint && current.archived == original.archived
        && current.updated_at == original.updated_at;
    if !same || (current.path != item.old_path && current.path != item.new_path) { return Err("rebalance group record preimage changed".to_string()); }
    let (prior, intended) = member_revisions(revisions, ordinal)?;
    if prior.path != item.old_path || intended.path != item.new_path { return Err("rebalance group revision paths conflict".to_string()); }
    for row in [prior, intended] { if stable_fingerprint(&row.bytes).map_err(|error| error.message)? != row.fingerprint { return Err("rebalance group revision fingerprint changed".to_string()); } }
    if prior.bytes != intended.bytes || prior.fingerprint != intended.fingerprint { return Err("rebalance group revisions differ".to_string()); }
    let text = std::str::from_utf8(&prior.bytes).map_err(|error| error.to_string())?;
    if record_fingerprint(text).map_err(|error| error.message)? != original.fingerprint { return Err("rebalance group revision conflicts with preimage".to_string()); }
    Ok(())
}

pub(super) fn intended_moves(operation: &OperationRow) -> Result<Vec<RebalanceMove>, String> {
    let value: serde_json::Value =
        serde_json::from_str(&operation.intended_json).map_err(|error| error.to_string())?;
    serde_json::from_value(
        value
            .get("moves")
            .cloned()
            .ok_or_else(|| "rebalance group moves missing".to_string())?,
    )
    .map_err(|error| error.to_string())
}

#[rustfmt::skip]
pub(super) fn member_revisions(rows: &[OperationRevision], ordinal: usize)
    -> Result<(&OperationRevision, &OperationRevision), String> {
    let prior_role = format!("prior:{ordinal:04}"); let intended_role = format!("intended:{ordinal:04}");
    let prior = rows.iter().find(|row| row.role == prior_role).ok_or_else(|| "rebalance group prior revision missing".to_string())?;
    let intended = rows.iter().find(|row| row.role == intended_role).ok_or_else(|| "rebalance group intended revision missing".to_string())?;
    Ok((prior, intended))
}

fn revision(
    ordinal: usize,
    role: &str,
    path: &str,
    bytes: &[u8],
    fingerprint: &str,
) -> OperationRevision {
    OperationRevision {
        role: format!("{role}:{ordinal:04}"),
        path: path.to_string(),
        bytes: bytes.to_vec(),
        fingerprint: fingerprint.to_string(),
    }
}
