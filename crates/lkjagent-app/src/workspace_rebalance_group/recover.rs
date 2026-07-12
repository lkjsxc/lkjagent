use std::path::Path;

use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_core::workspace_manifest::RebalanceMove;
use lkjagent_store::record_rows::{record, upsert_record};
use lkjagent_store::workspace_rows::{
    insert_alias_and_audit, transition_operation, update_operation_error, OperationRow,
    PathAliasRow,
};
use rusqlite::Connection;

use super::prepare::{load, member_revisions, Loaded};

pub(super) fn recover(
    conn: &Connection,
    data_dir: &Path,
    operation: &OperationRow,
    now: &str,
    allow_start: bool,
) -> Result<(), String> {
    let result = recover_inner(conn, data_dir, operation, now, allow_start);
    if let Err(error) = &result {
        update_operation_error(conn, &operation.id, error, now).map_err(|e| e.to_string())?;
    }
    result
}

fn recover_inner(
    conn: &Connection,
    data_dir: &Path,
    operation: &OperationRow,
    now: &str,
    allow_start: bool,
) -> Result<(), String> {
    let workspace = crate::config::workspace_root(data_dir)?;
    let loaded = load(conn, operation)?;
    let mut phase = operation.phase.as_str();
    if phase == "prepared" {
        verify_sources(&workspace, &loaded)?;
        if !allow_start {
            return Err("rebalance group requires explicit apply".to_string());
        }
        transition_operation(conn, &operation.id, "prepared", "moving", now)
            .map_err(|error| error.to_string())?;
        phase = "moving";
    }
    if phase == "moving" {
        move_members(&workspace, &loaded)?;
        transition_operation(conn, &operation.id, "moving", "projecting", now)
            .map_err(|error| error.to_string())?;
        phase = "projecting";
    }
    if phase == "projecting" {
        project(conn, data_dir, &workspace, &loaded, now)?;
        transition_operation(conn, &operation.id, "projecting", "settled", now)
            .map_err(|error| error.to_string())?;
        return Ok(());
    }
    Err(format!("unsupported rebalance group phase: {phase}"))
}

fn verify_sources(workspace: &Path, loaded: &Loaded) -> Result<(), String> {
    for (ordinal, (item, original)) in loaded.moves.iter().zip(&loaded.originals).enumerate() {
        let (prior, _) = member_revisions(&loaded.revisions, ordinal)?;
        if crate::effect_files::path_occupied(workspace, &item.new_path)? {
            return Err("rebalance group target is occupied".to_string());
        }
        let bytes = crate::workspace_rebalance::verified_file_bytes(
            workspace,
            &item.old_path,
            &original.fingerprint,
        )?;
        if bytes != prior.bytes {
            return Err("rebalance group source changed".to_string());
        }
    }
    Ok(())
}

fn move_members(workspace: &Path, loaded: &Loaded) -> Result<(), String> {
    for (ordinal, (item, original)) in loaded.moves.iter().zip(&loaded.originals).enumerate() {
        let (prior, intended) = member_revisions(&loaded.revisions, ordinal)?;
        let source = crate::effect_files::path_occupied(workspace, &item.old_path)?;
        let target = crate::effect_files::path_occupied(workspace, &item.new_path)?;
        match (source, target) {
            (true, false) => {
                let bytes = crate::workspace_rebalance::verified_file_bytes(
                    workspace,
                    &item.old_path,
                    &original.fingerprint,
                )?;
                if bytes != prior.bytes {
                    return Err("rebalance group source changed".to_string());
                }
                crate::record_files::move_relative_if_absent(
                    workspace,
                    &item.old_path,
                    &item.new_path,
                )?;
            }
            (false, true) => {}
            (true, true) => {
                return Err("rebalance group source and target both occupied".to_string());
            }
            (false, false) => {
                return Err("rebalance group source and target missing".to_string());
            }
        }
        crate::record_files::sync_relative_move(workspace, &item.old_path, &item.new_path)?;
        let target = crate::effect_files::read_bytes(workspace, &item.new_path)
            .map_err(|_| "rebalance group target conflicts".to_string())?;
        if crate::effect_files::path_occupied(workspace, &item.old_path)?
            || target != intended.bytes
        {
            return Err("rebalance group target conflicts".to_string());
        }
    }
    Ok(())
}

fn project(
    conn: &Connection,
    data_dir: &Path,
    workspace: &Path,
    loaded: &Loaded,
    now: &str,
) -> Result<(), String> {
    move_members(workspace, loaded)?;
    for (item, original) in loaded.moves.iter().zip(&loaded.originals) {
        let current = record(conn, &item.entity_id)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| format!("record not found: {}", item.entity_id))?;
        if current.path != item.new_path {
            let mut row = original.clone();
            row.path = item.new_path.clone();
            upsert_record(conn, &row).map_err(|error| error.to_string())?;
        }
    }
    for item in &loaded.moves {
        crate::workspace_root::refresh_for_path(workspace, &item.old_path)?;
        crate::workspace_root::refresh_for_path(workspace, &item.new_path)?;
        let mut audited = item.clone();
        let repaired = crate::workspace_root::repair_record_links(
            conn,
            workspace,
            &item.entity_id,
            &item.old_path,
            &item.new_path,
            now,
        )?;
        audited
            .validation
            .push(format!("links-repaired:{repaired}"));
        insert_alias_and_audit(conn, &alias(item, now), &audit_id(item)?, &audited, now)
            .map_err(|error| error.to_string())?;
    }
    crate::workspace_index::rebuild(conn, data_dir, now).map(|_| ())
}

fn alias(item: &RebalanceMove, now: &str) -> PathAliasRow {
    PathAliasRow {
        old_path: item.old_path.clone(),
        entity_id: item.entity_id.clone(),
        entity_kind: item.entity_kind.clone(),
        new_path: item.new_path.clone(),
        decision_id: item.decision_id.clone(),
        created_at: now.to_string(),
    }
}

fn audit_id(item: &RebalanceMove) -> Result<String, String> {
    stable_fingerprint(&format!(
        "{}\0{}\0{}",
        item.entity_id, item.old_path, item.new_path
    ))
    .map(|value| format!("rebalance-{value}"))
    .map_err(|error| error.message)
}
