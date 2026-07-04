use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::RuntimeDecision;
use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn insert_prompt_frame(
    conn: &Connection,
    id: &str,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    body_ref: &str,
    created_at: &str,
) -> StoreResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO prompt_frames
         (id, case_id, decision_id, prompt_fingerprint,
          context_frame_fingerprint, tool_view_fingerprint, body_ref, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            id,
            decision.case_id,
            decision.id,
            prompt.fingerprint,
            decision.context_frame_fingerprint,
            decision.tool_view_fingerprint().unwrap_or_default(),
            body_ref,
            created_at,
        ],
    )?;
    Ok(())
}
