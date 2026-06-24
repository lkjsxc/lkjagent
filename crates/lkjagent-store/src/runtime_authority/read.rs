use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::model::{
    AuthorityDecisionRow, AuthoritySnapshotRow, PromptFrameRow, RuntimeObservationRow,
    RuntimeTransitionRow, ToolAdmissionRow,
};

pub fn latest_snapshot_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<AuthoritySnapshotRow>> {
    let sql = "SELECT id, case_scope, case_id, active_mode, active_node,
        missing_evidence, staleness_fingerprint FROM runtime_snapshots
        WHERE case_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(snapshot_row).transpose()?)
}

pub fn latest_decision(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<AuthorityDecisionRow>> {
    latest_decision_for_case(conn, case_id)
}

pub fn latest_decision_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<AuthorityDecisionRow>> {
    let sql = "SELECT id, snapshot_id, case_scope, case_id, event_id, mission,
        active_mode, active_node, admitted_tools, blocked_tools, missing_evidence,
        forced_next_action, exact_valid_example, completion_allowed, completion_refusal,
        recovery_route, compaction_required, maintenance_allowed, authority_fingerprint,
        staleness_fingerprint FROM runtime_authority_decisions
        WHERE case_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(decision_row).transpose()?)
}

pub fn latest_transition_for_case(
    conn: &Connection,
    case_id: i64,
) -> StoreResult<Option<RuntimeTransitionRow>> {
    let sql = "SELECT id, decision_id, from_node, to_node, transition_kind
        FROM runtime_transitions WHERE case_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![case_id])?;
    Ok(rows.next()?.map(transition_row).transpose()?)
}

pub fn latest_prompt_frame_for_decision(
    conn: &Connection,
    decision_id: i64,
) -> StoreResult<Option<PromptFrameRow>> {
    let sql = "SELECT id, decision_id, frame_kind, prompt_fingerprint,
        context_package_ids, rendered_summary FROM runtime_prompt_frames
        WHERE decision_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![decision_id])?;
    Ok(rows.next()?.map(prompt_frame_row).transpose()?)
}

pub fn latest_observation_for_decision(
    conn: &Connection,
    decision_id: i64,
) -> StoreResult<Option<RuntimeObservationRow>> {
    let sql = "SELECT id, decision_id, admission_id, effect_id, observation_event_id,
        observation_kind, status, summary FROM runtime_observations
        WHERE decision_id = ?1 ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![decision_id])?;
    Ok(rows.next()?.map(observation_row).transpose()?)
}

pub fn admission_for_decision_and_tool(
    conn: &Connection,
    decision_id: i64,
    tool: &str,
) -> StoreResult<Option<ToolAdmissionRow>> {
    let sql = "SELECT id, decision_id, requested_tool, admitted, refusal_reason
        FROM runtime_tool_admissions WHERE decision_id = ?1 AND requested_tool = ?2
        ORDER BY id DESC LIMIT 1";
    let mut statement = conn.prepare(sql)?;
    let mut rows = statement.query(params![decision_id, tool])?;
    Ok(rows.next()?.map(admission_row).transpose()?)
}

fn snapshot_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthoritySnapshotRow> {
    Ok(AuthoritySnapshotRow {
        id: row.get(0)?,
        case_scope: row.get(1)?,
        case_id: row.get(2)?,
        active_mode: row.get(3)?,
        active_node: row.get(4)?,
        missing_evidence: row.get(5)?,
        staleness_fingerprint: row.get(6)?,
    })
}

fn decision_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthorityDecisionRow> {
    Ok(AuthorityDecisionRow {
        id: row.get(0)?,
        snapshot_id: row.get(1)?,
        case_scope: row.get(2)?,
        case_id: row.get(3)?,
        event_id: row.get(4)?,
        mission: row.get(5)?,
        active_mode: row.get(6)?,
        active_node: row.get(7)?,
        admitted_tools: row.get(8)?,
        blocked_tools: row.get(9)?,
        missing_evidence: row.get(10)?,
        forced_next_action: row.get(11)?,
        exact_valid_example: row.get(12)?,
        completion_allowed: row.get::<_, i64>(13)? != 0,
        completion_refusal: row.get(14)?,
        recovery_route: row.get(15)?,
        compaction_required: row.get::<_, i64>(16)? != 0,
        maintenance_allowed: row.get::<_, i64>(17)? != 0,
        authority_fingerprint: row.get(18)?,
        staleness_fingerprint: row.get(19)?,
    })
}

fn admission_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ToolAdmissionRow> {
    Ok(ToolAdmissionRow {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        requested_tool: row.get(2)?,
        admitted: row.get::<_, i64>(3)? != 0,
        refusal_reason: row.get(4)?,
    })
}

fn transition_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RuntimeTransitionRow> {
    Ok(RuntimeTransitionRow {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        from_node: row.get(2)?,
        to_node: row.get(3)?,
        transition_kind: row.get(4)?,
    })
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

fn observation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RuntimeObservationRow> {
    Ok(RuntimeObservationRow {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        admission_id: row.get(2)?,
        effect_id: row.get(3)?,
        observation_event_id: row.get(4)?,
        observation_kind: row.get(5)?,
        status: row.get(6)?,
        summary: row.get(7)?,
    })
}
