use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::detail_model::{DecisionDetailInput, SnapshotDetailInput};

pub fn record_snapshot_detail(
    conn: &Connection,
    input: &SnapshotDetailInput<'_>,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO runtime_snapshot_details
         (snapshot_id, graph_phase, artifact_root, weak_cursor, latest_observation,
          prompt_frame_head, authority_fingerprint)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            input.snapshot_id,
            input.graph_phase,
            input.artifact_root,
            input.weak_cursor,
            input.latest_observation,
            input.prompt_frame_head,
            input.authority_fingerprint,
        ],
    )?;
    Ok(())
}

pub fn record_decision_detail(
    conn: &Connection,
    input: &DecisionDetailInput<'_>,
) -> StoreResult<()> {
    conn.execute(
        "INSERT INTO runtime_decision_details
         (decision_id, decision_kind, graph_phase, exact_next_action_class,
          runtime_effect_kind, artifact_root, weak_cursor, latest_observation,
          prompt_frame_head, authority_fingerprint, staleness_fingerprint)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            input.decision_id,
            input.decision_kind,
            input.graph_phase,
            input.exact_next_action_class,
            input.runtime_effect_kind,
            input.artifact_root,
            input.weak_cursor,
            input.latest_observation,
            input.prompt_frame_head,
            input.authority_fingerprint,
            input.staleness_fingerprint,
        ],
    )?;
    Ok(())
}
