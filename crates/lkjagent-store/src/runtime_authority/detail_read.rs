use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::detail_model::{AuthorityChainRow, DecisionDetailRow, SnapshotDetailRow};
use super::model::PromptFrameRow;

pub fn snapshot_detail_for_snapshot(
    conn: &Connection,
    snapshot_id: i64,
) -> StoreResult<Option<SnapshotDetailRow>> {
    let sql = "SELECT snapshot_id, graph_phase, artifact_root, weak_cursor,
        latest_observation, prompt_frame_head, authority_fingerprint
        FROM runtime_snapshot_details WHERE snapshot_id = ?1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![snapshot_id])?;
    Ok(rows.next()?.map(snapshot_detail_row).transpose()?)
}

pub fn latest_decision_detail_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<DecisionDetailRow>> {
    let sql = "SELECT detail.decision_id, detail.decision_kind, detail.graph_phase,
        detail.exact_next_action_class, detail.runtime_effect_kind,
        detail.artifact_root, detail.weak_cursor, detail.latest_observation,
        detail.prompt_frame_head, detail.authority_fingerprint,
        detail.staleness_fingerprint FROM runtime_decision_details detail
        INNER JOIN runtime_authority_decisions decision ON decision.id = detail.decision_id
        WHERE decision.case_id = ?1 ORDER BY decision.id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(decision_detail_row).transpose()?)
}

pub fn latest_complete_chain_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<AuthorityChainRow>> {
    let sql = "SELECT decision.snapshot_id, decision.event_id, decision.id,
        (SELECT id FROM runtime_prompt_frames
            WHERE decision_id = decision.id ORDER BY id DESC LIMIT 1),
        (SELECT id FROM runtime_tool_admissions
            WHERE decision_id = decision.id ORDER BY id DESC LIMIT 1),
        (SELECT id FROM runtime_observations
            WHERE decision_id = decision.id ORDER BY id DESC LIMIT 1)
        FROM runtime_authority_decisions decision
        WHERE decision.case_id = ?1 ORDER BY decision.id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(chain_row).transpose()?)
}

pub fn latest_prompt_frame_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<PromptFrameRow>> {
    let sql = "SELECT frame.id, frame.decision_id, frame.frame_kind,
        frame.prompt_fingerprint, frame.context_package_ids, frame.rendered_summary
        FROM runtime_prompt_frames frame
        INNER JOIN runtime_authority_decisions decision ON decision.id = frame.decision_id
        WHERE decision.case_id = ?1 ORDER BY frame.id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(prompt_frame_row).transpose()?)
}

pub fn latest_admission_for_decision(
    conn: &Connection,
    decision_id: i64,
) -> StoreResult<Option<i64>> {
    let sql = "SELECT id FROM runtime_tool_admissions
        WHERE decision_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![decision_id])?;
    Ok(rows.next()?.map(|row| row.get(0)).transpose()?)
}

fn prompt_frame_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PromptFrameRow> {
    Ok(PromptFrameRow {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        frame_kind: row.get(2)?,
        prompt_fingerprint: row.get(3)?,
        context_package_ids: row.get(4)?,
        rendered_summary: row.get(5)?,
    })
}

fn snapshot_detail_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SnapshotDetailRow> {
    Ok(SnapshotDetailRow {
        snapshot_id: row.get(0)?,
        graph_phase: row.get(1)?,
        artifact_root: row.get(2)?,
        weak_cursor: row.get(3)?,
        latest_observation: row.get(4)?,
        prompt_frame_head: row.get(5)?,
        authority_fingerprint: row.get(6)?,
    })
}

fn decision_detail_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionDetailRow> {
    Ok(DecisionDetailRow {
        decision_id: row.get(0)?,
        decision_kind: row.get(1)?,
        graph_phase: row.get(2)?,
        exact_next_action_class: row.get(3)?,
        runtime_effect_kind: row.get(4)?,
        artifact_root: row.get(5)?,
        weak_cursor: row.get(6)?,
        latest_observation: row.get(7)?,
        prompt_frame_head: row.get(8)?,
        authority_fingerprint: row.get(9)?,
        staleness_fingerprint: row.get(10)?,
    })
}

fn chain_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthorityChainRow> {
    Ok(AuthorityChainRow {
        snapshot_id: row.get(0)?,
        event_id: row.get(1)?,
        decision_id: row.get(2)?,
        prompt_frame_id: row.get(3)?,
        admission_id: row.get(4)?,
        observation_id: row.get(5)?,
    })
}
