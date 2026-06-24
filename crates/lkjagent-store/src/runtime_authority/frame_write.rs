use rusqlite::{params, Connection};

use crate::error::StoreResult;

use super::codec::encode_list;
use super::model::{PromptFrameInput, RuntimeObservationInput};

pub fn record_prompt_frame(conn: &Connection, input: &PromptFrameInput<'_>) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_prompt_frames
         (decision_id, case_scope, case_id, frame_kind, prompt_fingerprint,
          context_package_ids, rendered_summary, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.decision_id,
            input.case_scope,
            input.case_id,
            input.frame_kind,
            input.prompt_fingerprint,
            encode_list(input.context_package_ids),
            input.rendered_summary,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn record_runtime_observation(
    conn: &Connection,
    input: &RuntimeObservationInput<'_>,
) -> StoreResult<i64> {
    conn.execute(
        "INSERT INTO runtime_observations
         (decision_id, admission_id, effect_id, observation_event_id, observation_kind,
          status, summary, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.decision_id,
            input.admission_id,
            input.effect_id,
            input.observation_event_id,
            input.observation_kind,
            input.status,
            input.summary,
            input.created_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}
